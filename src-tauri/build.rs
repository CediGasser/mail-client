use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        // Filter to include only desired env vars
        if key.starts_with("GOOGLE_") || key == "API_URL" || key == "OUT_DIR" {
            println!("cargo:rustc-env={}={}", key, value);
        }
    }

    tauri_build::build()
}
