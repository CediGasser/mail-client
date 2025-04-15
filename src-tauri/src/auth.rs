// This file contains the authentication logic for the application.
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use oauth2::basic::{BasicErrorResponseType, BasicTokenType};
use oauth2::{
    basic::BasicClient, reqwest, AuthUrl, AuthorizationCode, ClientId, CsrfToken, RedirectUrl,
    RevocationUrl, TokenResponse, TokenUrl,
};
use oauth2::{
    Client, ClientSecret, EmptyExtraTokenFields, EndpointNotSet, EndpointSet, PkceCodeChallenge,
    PkceCodeVerifier, RevocationErrorResponseType, StandardErrorResponse, StandardRevocableToken,
    StandardTokenIntrospectionResponse, StandardTokenResponse,
};
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;
use tauri::async_runtime::JoinHandle;
use tauri::Manager;
use tauri_plugin_opener::open_url;

use crate::util::navigate;
use crate::{
    constants::{
        GOOGLE_AUTH_URI, GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET, GOOGLE_MAIL_SCOPE,
        GOOGLE_PROFILE_API, GOOGLE_PROFILE_MAIL_SCOPE, GOOGLE_PROFILE_SCOPE, GOOGLE_REVOKATION_URI,
        GOOGLE_TOKEN_URI,
    },
    error::Error,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GmailOAuth2 {
    user: String,
    access_token: String,
}

impl imap::Authenticator for GmailOAuth2 {
    type Response = String;
    #[allow(unused_variables)]
    fn process(&self, data: &[u8]) -> Self::Response {
        return format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user, self.access_token
        );
    }
}

type OAuthClient = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
    EndpointSet,
>;

#[derive(Clone)]
struct AuthState {
    csrf_token: CsrfToken,
    pkce: Arc<(PkceCodeChallenge, String)>,
    client: Arc<OAuthClient>,
    socket_addr: SocketAddr,
}

fn create_client(redirect_url: RedirectUrl) -> OAuthClient {
    let client_id = ClientId::new(GOOGLE_CLIENT_ID.to_string());
    let client_secret = ClientSecret::new(GOOGLE_CLIENT_SECRET.to_string());
    let auth_uri = AuthUrl::new(GOOGLE_AUTH_URI.to_string()).expect("Invalid authorization URL");
    let token_uri = TokenUrl::new(GOOGLE_TOKEN_URI.to_string()).expect("Invalid token URL");
    let revokation_url =
        RevocationUrl::new(GOOGLE_REVOKATION_URI.to_string()).expect("Invalid revocation URL");

    let client = BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_uri)
        .set_token_uri(token_uri)
        .set_revocation_url(revokation_url)
        .set_redirect_uri(redirect_url);

    return client;
}

fn get_available_addr() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);

    return addr;
}

pub async fn init_google_oauth_flow(handle: tauri::AppHandle) -> Result<(), Error> {
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let socket_addr = get_available_addr(); // or any other port
    let redirect_url = format!("http://{socket_addr}/callback").to_string();

    let auth = AuthState {
        csrf_token: CsrfToken::new_random(),
        pkce: Arc::new((
            pkce_code_challenge,
            PkceCodeVerifier::secret(&pkce_code_verifier).to_string(),
        )),
        client: Arc::new(create_client(RedirectUrl::new(redirect_url).unwrap())),
        socket_addr,
    };

    handle.manage(auth);

    let auth_clone = handle.state::<AuthState>().clone();

    let scopes: Vec<oauth2::Scope> = vec![
        oauth2::Scope::new(GOOGLE_MAIL_SCOPE.to_string()),
        oauth2::Scope::new(GOOGLE_PROFILE_SCOPE.to_string()),
        oauth2::Scope::new(GOOGLE_PROFILE_MAIL_SCOPE.to_string()),
    ];

    let (auth_url, _) = auth_clone
        .client
        .authorize_url(|| auth_clone.csrf_token.clone())
        .add_scopes(scopes)
        .set_pkce_challenge(auth_clone.pkce.0.clone())
        .url();

    // Open the authorization URL in the user's default browser
    print!("Opening URL: {}\n", auth_url);
    open_url(auth_url, None::<String>).expect("Failed to open URL");

    let handle_clone = handle.clone();

    let server_handle: JoinHandle<Result<(), axum::Error>> =
        tauri::async_runtime::spawn(async move { run_server(&handle_clone).await });

    // Store the server handle in the app state
    handle.manage(server_handle);

    return Ok(());
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: AuthorizationCode,
    state: CsrfToken,
}

#[derive(Deserialize)]
struct ProfileResponse {
    // sub: String,
    // name: String,
    // given_name: String,
    // family_name: String,
    // picture: Url,
    email: String,
    // email_verified: bool,
}

async fn authorize(
    handle: Extension<tauri::AppHandle>,
    query: Query<CallbackQuery>,
) -> impl IntoResponse {
    let auth = handle.state::<AuthState>();

    if query.state.secret() != auth.csrf_token.secret() {
        return "Not authorized".to_string();
    }

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to build HTTP client");

    println!("Received authorization code: {}", query.code.secret());

    // Exchange the authorization code for an access token
    let token = auth
        .client
        .exchange_code(query.code.clone())
        .set_pkce_verifier(PkceCodeVerifier::new(auth.pkce.1.clone()))
        .request_async(&http_client)
        .await;

    if let Err(e) = token {
        println!("Error exchanging code: {:?}", e);
        return e.to_string();
    }
    let token = token.unwrap();

    // Get the email address from the profile endpoint
    let profile_response = reqwest::Client::new()
        .get(GOOGLE_PROFILE_API)
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .expect("Failed to get profile");

    let profile_json = profile_response
        .json::<ProfileResponse>()
        .await
        .expect("Failed to parse profile");

    let email = profile_json.email.as_str();
    let user = email.to_string();
    let access_token = token.access_token().secret().to_string();

    let gmail_oauth2 = GmailOAuth2 {
        user: user.clone(),
        access_token: access_token.clone(),
    };

    println!("User: {:?}", user);

    handle.manage(gmail_oauth2);

    // Signal the server to shut down
    let server_handle = handle.try_state::<JoinHandle<Result<(), axum::Error>>>();
    match server_handle {
        Some(handle) => {
            handle.abort();
        }
        None => {
            panic!("Failed to close server. Something went wrong.");
        }
    }

    // Open the inbox in the tauri app
    let window = handle.get_webview_window("main").unwrap();
    navigate(window, "/mail");

    "Login successful.
You can close this window."
        .to_string()
}

async fn run_server(handle: &tauri::AppHandle) -> Result<(), axum::Error> {
    let app = Router::new()
        .route("/callback", get(authorize))
        .layer(Extension(handle.clone()));

    let _ = axum::Server::bind(&handle.state::<AuthState>().socket_addr.clone())
        .serve(app.into_make_service())
        .await;

    Ok(())
}
