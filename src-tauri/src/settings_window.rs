use tauri::{AppHandle, Manager as _, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

const SETTINGS_WINDOW_LABEL: &str = "settings";
const SETTINGS_PAGE_URL: &str = "settings.html";

pub fn show(app: &AppHandle) -> eyre::Result<()> {
    if let Some(window) = app.get_webview_window(SETTINGS_WINDOW_LABEL) {
        return focus_window(&window);
    }

    let window = build_settings_window(app)?;
    window.set_shadow(true)?;
    window.center()?;
    Ok(())
}

fn focus_window(window: &WebviewWindow) -> eyre::Result<()> {
    window.show()?;
    window.set_focus()?;
    Ok(())
}

fn build_settings_window(app: &AppHandle) -> tauri::Result<WebviewWindow> {
    WebviewWindowBuilder::new(
        app,
        SETTINGS_WINDOW_LABEL,
        WebviewUrl::App(SETTINGS_PAGE_URL.into()),
    )
    .title("设置")
    .inner_size(520.0, 420.0)
    .resizable(false)
    .maximizable(false)
    .closable(true)
    .devtools(true)
    .build()
}
