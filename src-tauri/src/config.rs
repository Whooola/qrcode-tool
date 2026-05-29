use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub hotkeys: Vec<HotkeyEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyEntry {
    pub id: String,
    pub key: String,
    pub press_count: usize,
    pub timeout_ms: u64,
    pub action: String,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        HotkeyConfig {
            hotkeys: vec![HotkeyEntry {
                id: "smart-trigger".into(),
                key: "Control".into(),
                press_count: 2,
                timeout_ms: 500,
                action: "smart-trigger".into(),
            }],
        }
    }
}

pub fn load() -> HotkeyConfig {
    // Try user config: ~/.qrcode-tool/config.json
    if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
        let user_path = std::path::PathBuf::from(&home)
            .join(".qrcode-tool")
            .join("config.json");
        if user_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&user_path) {
                if let Ok(cfg) = serde_json::from_str::<HotkeyConfig>(&content) {
                    return cfg;
                }
            }
        }
    }

    // Fall back to bundled config
    let bundled = include_str!("../config.json");
    serde_json::from_str(bundled).unwrap_or_default()
}
