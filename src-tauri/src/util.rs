use tauri::Emitter;

pub fn navigate(window: tauri::WebviewWindow, url: &str) {
    window.emit("navigate", url).unwrap();
}
