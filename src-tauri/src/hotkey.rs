use crate::config::HotkeyConfig;
use rdev::{listen, Event, EventType, Key};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::Emitter;

fn key_name(key: &Key) -> String {
    match key {
        Key::ControlLeft | Key::ControlRight => "Control".into(),
        Key::Alt | Key::AltGr => "Alt".into(),
        Key::ShiftLeft | Key::ShiftRight => "Shift".into(),
        Key::MetaLeft | Key::MetaRight => "Meta".into(),
        _ => format!("{:?}", key),
    }
}

struct HotkeyState {
    last_press: HashMap<String, Vec<Instant>>,
}

pub fn start(handle: tauri::AppHandle, config: HotkeyConfig) {
    let state = Arc::new(Mutex::new(HotkeyState {
        last_press: HashMap::new(),
    }));

    std::thread::spawn(move || {
        let cb = move |event: Event| {
            if let EventType::KeyPress(key) = event.event_type {
                let name = key_name(&key);
                let is_target = config.hotkeys.iter().any(|h| h.key == name);

                if is_target {
                    let mut s = state.lock().unwrap();
                    let now = Instant::now();
                    let presses = s.last_press.entry(name.clone()).or_default();
                    presses.retain(|t| now.duration_since(*t).as_millis() < 1000);
                    presses.push(now);

                    for hk in &config.hotkeys {
                        if hk.key == name && presses.len() >= hk.press_count {
                            let first = presses[presses.len() - hk.press_count];
                            if now.duration_since(first).as_millis() <= hk.timeout_ms as u128 {
                                presses.clear();
                                let payload = serde_json::json!({ "action": &hk.action });
                                let _ = handle.emit("hotkey-triggered", payload);
                                break;
                            }
                        }
                    }
                } else {
                    let mut s = state.lock().unwrap();
                    s.last_press.clear();
                }
            }
        };
        if let Err(e) = listen(cb) {
            eprintln!("Hotkey listen error: {:?}", e);
        }
    });
}
