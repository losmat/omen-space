use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Auto,
    Tr,
    En,
}

#[derive(Debug, Clone, Deserialize)]
struct GuiConfig {
    #[serde(default)]
    language: String,
}

pub fn get_language() -> Language {
    let mut selected = Language::Auto;
    if let Some(home) = std::env::var_os("HOME") {
        let mut p = PathBuf::from(home);
        p.push(".config");
        p.push("omenspace");
        p.push("gui_config.json");

        if let Ok(content) = fs::read_to_string(&p) {
            if let Ok(cfg) = serde_json::from_str::<GuiConfig>(&content) {
                selected = match cfg.language.as_str() {
                    "tr" => Language::Tr,
                    "en" => Language::En,
                    _ => Language::Auto,
                };
            }
        }
    }

    if selected == Language::Auto {
        for var in &["LC_MESSAGES", "LC_ALL", "LANG", "LANGUAGE"] {
            if let Ok(val) = std::env::var(var) {
                let lower = val.to_lowercase();
                if lower.starts_with("tr") || lower.contains("tr_tr") || lower.contains("turkish") {
                    return Language::Tr;
                }
            }
        }
        Language::En
    } else {
        selected
    }
}

pub fn t(key: &'static str) -> &'static str {
    match get_language() {
        Language::Tr => translate_tr(key),
        _ => translate_en(key),
    }
}

fn translate_tr(key: &'static str) -> &'static str {
    match key {
        // Tooltip values (match GUI terms: Otomatik etc.)
        "power_performance" => "Performans",
        "power_eco" => "Eko",
        "power_balanced" => "Dengeli",
        "fan_max" => "Maksimum",
        "fan_ec" => "EC (Donanım)",
        "fan_custom" => "Özel",
        "fan_auto" => "Otomatik",
        "tooltip_fmt" => "Güç: {}\nFan: {}",
        // Menu
        "tray_open" => "OMENSpace'i Aç",
        "tray_power" => "⚡ Güç Profili",
        "tray_perf" => "🔥 Performans",
        "tray_balanced" => "⚖️ Dengeli",
        "tray_eco" => "🍃 Eko",
        "tray_fan" => "❄️ Fan Modu",
        "tray_auto" => "🤖 Otomatik",
        "tray_max" => "🌪️ Maksimum",
        "tray_ec" => "⚙️ EC (Donanım)",
        "tray_exit" => "❌ Çıkış",
        // Logs ({} placeholders, formatted by caller)
        "log_power_ok" => "Güç profili ayarlandı ({}) -> {}",
        "log_power_err" => "Güç profili değiştirilemedi: {}",
        "log_fan_ok" => "Fan modu ayarlandı ({}) -> {}",
        "log_fan_err" => "Fan modu değiştirilemedi: {}",
        "log_already_running" => "omen-tray zaten çalışıyor, ikinci örnek sonlandırılıyor.",
        "log_starting" => "omen-tray başlatılıyor...",
        _ => key,
    }
}

fn translate_en(key: &'static str) -> &'static str {
    match key {
        "power_performance" => "Performance",
        "power_eco" => "Eco",
        "power_balanced" => "Balanced",
        "fan_max" => "Max",
        "fan_ec" => "EC (Hardware)",
        "fan_custom" => "Custom",
        "fan_auto" => "Auto",
        "tooltip_fmt" => "Power: {}\nFan: {}",
        "tray_open" => "Open OMENSpace",
        "tray_power" => "⚡ Power Profile",
        "tray_perf" => "🔥 Performance",
        "tray_balanced" => "⚖️ Balanced",
        "tray_eco" => "🍃 Eco",
        "tray_fan" => "❄️ Fan Mode",
        "tray_auto" => "🤖 Auto",
        "tray_max" => "🌪️ Max",
        "tray_ec" => "⚙️ EC (Hardware)",
        "tray_exit" => "❌ Exit",
        "log_power_ok" => "Power profile set ({}) -> {}",
        "log_power_err" => "Failed to change power profile: {}",
        "log_fan_ok" => "Fan mode set ({}) -> {}",
        "log_fan_err" => "Failed to change fan mode: {}",
        "log_already_running" => "omen-tray already running, exiting second instance.",
        "log_starting" => "Starting omen-tray...",
        _ => key,
    }
}
