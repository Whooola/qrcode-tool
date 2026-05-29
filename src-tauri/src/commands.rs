use enigo::{Direction, Enigo, Key, Keyboard};
use std::sync::Mutex;
use std::time::Duration;
use tauri::{Manager, State, WebviewUrl, WebviewWindowBuilder};

pub struct AppState {
    pub has_open_window: Mutex<bool>,
}

#[tauri::command]
pub async fn handle_trigger(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let has_window = *state.has_open_window.lock().unwrap();

    if has_window {
        // Close any open QR/scanner windows
        if let Some(w) = app.get_webview_window("qr-display") {
            let _ = w.close();
        }
        if let Some(w) = app.get_webview_window("scanner") {
            let _ = w.close();
        }
        *state.has_open_window.lock().unwrap() = false;
        return Ok("closed".into());
    }

    // Try to copy selected text via Ctrl+C / Cmd+C
    let text = copy_selection().unwrap_or_default();

    if text.trim().is_empty() {
        // No selection — open scanner
        open_scanner_window(&app).map_err(|e| e.to_string())?;
    } else {
        // Has selection — show QR code
        open_qr_window(&app, &text).map_err(|e| e.to_string())?;
    }

    *state.has_open_window.lock().unwrap() = true;
    Ok("opened".into())
}

#[tauri::command]
pub async fn notify_closed(state: State<'_, AppState>) -> Result<(), String> {
    *state.has_open_window.lock().unwrap() = false;
    Ok(())
}

fn copy_selection() -> Result<String, Box<dyn std::error::Error>> {
    let mut enigo = Enigo::new(&enigo::Settings::default())?;
    let mut clipboard = arboard::Clipboard::new()?;

    // Save current clipboard content
    let saved = clipboard.get_text().unwrap_or_default();

    // Simulate Ctrl+C (Windows/Linux) or Cmd+C (macOS)
    #[cfg(target_os = "macos")]
    {
        enigo.key(Key::Meta, Direction::Press)?;
        enigo.key(Key::Unicode('c'), Direction::Click)?;
        enigo.key(Key::Meta, Direction::Release)?;
    }
    #[cfg(not(target_os = "macos"))]
    {
        enigo.key(Key::Control, Direction::Press)?;
        enigo.key(Key::Unicode('c'), Direction::Click)?;
        enigo.key(Key::Control, Direction::Release)?;
    }

    // Wait for clipboard to receive the selection
    std::thread::sleep(Duration::from_millis(60));

    let text = clipboard.get_text().unwrap_or_default();

    // Restore original clipboard
    let _ = clipboard.set_text(&saved);

    Ok(text)
}

fn open_qr_window(app: &tauri::AppHandle, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let encoded = urlencoding(text);
    let url = format!("qr-display.html?text={}", encoded);

    WebviewWindowBuilder::new(app, "qr-display", WebviewUrl::App(url.into()))
        .title("")
        .inner_size(320.0, 420.0)
        .decorations(false)
        .resizable(false)
        .center()
        .build()?;

    Ok(())
}

fn open_scanner_window(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    WebviewWindowBuilder::new(app, "scanner", WebviewUrl::App("scanner.html".into()))
        .title("")
        .inner_size(420.0, 500.0)
        .decorations(false)
        .resizable(false)
        .center()
        .build()?;

    Ok(())
}

fn urlencoding(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 3);
    for b in s.as_bytes() {
        match *b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(*b as char);
            }
            b' ' => result.push('+'),
            _ => {
                result.push('%');
                result.push(HEX_DIGITS[(*b >> 4) as usize] as char);
                result.push(HEX_DIGITS[(*b & 0x0f) as usize] as char);
            }
        }
    }
    result
}

const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";
