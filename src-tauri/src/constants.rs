pub const GOOGLE_CLIENT_ID: &str = env!("GOOGLE_CLIENT_ID");
pub const GOOGLE_CLIENT_SECRET: &str = env!("GOOGLE_CLIENT_SECRET");

pub const GOOGLE_AUTH_URI: &str = "https://accounts.google.com/o/oauth2/auth";
pub const GOOGLE_TOKEN_URI: &str = "https://oauth2.googleapis.com/token";
pub const GOOGLE_REVOKATION_URI: &str = "https://oauth2.googleapis.com/revoke";

pub const GOOGLE_MAIL_SCOPE: &str = "https://mail.google.com/";
pub const GOOGLE_PROFILE_SCOPE: &str = "https://www.googleapis.com/auth/userinfo.profile";
pub const GOOGLE_PROFILE_MAIL_SCOPE: &str = "email";

pub const GOOGLE_PROFILE_API: &str = "https://www.googleapis.com/oauth2/v3/userinfo";

pub const GOOGLE_IMAP_HOST: &str = "imap.gmail.com";
pub const GOOGLE_IMAP_PORT: u16 = 993;
pub const GOOGLE_SMTP_HOST: &str = "smtp.gmail.com";
