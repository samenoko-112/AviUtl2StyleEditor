use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct StyleConfig {
    pub font: HashMap<String, String>,
    pub color: HashMap<String, String>,
    pub layout: HashMap<String, String>,
    pub format: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontSetting {
    pub key: String,
    pub label: String,
    pub description: String,
    pub input_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontValue {
    pub size: String,
    pub family: String,
}

#[tauri::command]
fn load_style_config(file_path: String) -> Result<StyleConfig, String> {
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("ファイルの読み込みに失敗しました: {}", e))?;
    
    parse_style_config(&content)
}

#[tauri::command]
fn save_style_config(file_path: String, config: StyleConfig) -> Result<(), String> {
    let content = serialize_style_config(&config);
    fs::write(&file_path, content)
        .map_err(|e| format!("ファイルの保存に失敗しました: {}", e))?;
    Ok(())
}

#[tauri::command]
fn get_default_style_path() -> String {
    "C:\\Program Files\\AviUtl2\\style.conf".to_string()
}

#[tauri::command]
fn get_user_style_path() -> String {
    let appdata = std::env::var("PROGRAMDATA").unwrap_or_else(|_| "C:\\ProgramData".to_string());
    format!("{}\\aviutl2\\style.conf", appdata)
}

#[tauri::command]
fn open_file_location(file_path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &file_path])
            .spawn()
            .map_err(|e| format!("ファイルを開けませんでした: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(["-R", &file_path])
            .spawn()
            .map_err(|e| format!("ファイルを開けませんでした: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .args([&file_path])
            .spawn()
            .map_err(|e| format!("ファイルを開けませんでした: {}", e))?;
    }
    
    Ok(())
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn get_system_fonts() -> Result<Vec<String>, String> {
    use winreg::enums::HKEY_LOCAL_MACHINE;
    use winreg::RegKey;
    let mut fonts = Vec::new();
    let mut font_names = std::collections::HashSet::new();
    
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let font_path = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts";
    if let Ok(fonts_key) = hklm.open_subkey(font_path) {
        for item in fonts_key.enum_values() {
            if let Ok((name, _)) = item {
                // フォント名からウェイト情報と&以降を除去
                let clean = name.split('(').next().unwrap_or("").trim();
                let first = clean.split('&').next().unwrap_or("").trim().to_string();
                if !first.is_empty() && font_names.insert(first.clone()) {
                    fonts.push(first);
                }
            }
        }
    }
    fonts.sort();
    Ok(fonts)
}

#[tauri::command]
fn get_font_settings() -> Vec<FontSetting> {
    vec![
        FontSetting {
            key: "DefaultFamily".to_string(),
            label: "標準フォント".to_string(),
            description: "標準のフォント名".to_string(),
            input_type: "familyOnly".to_string(),
        },
        FontSetting {
            key: "Control".to_string(),
            label: "コントロールフォント".to_string(),
            description: "標準のコントロールのフォントサイズ".to_string(),
            input_type: "sizeOnly".to_string(),
        },
        FontSetting {
            key: "EditControl".to_string(),
            label: "エディットコントロール".to_string(),
            description: "エディットコントロールのフォント（等幅推奨）".to_string(),
            input_type: "both".to_string(),
        },
        FontSetting {
            key: "PreviewTime".to_string(),
            label: "プレビュー時間表示".to_string(),
            description: "プレビュー時間表示のフォントサイズ".to_string(),
            input_type: "sizeOnly".to_string(),
        },
        FontSetting {
            key: "LayerObject".to_string(),
            label: "レイヤー・オブジェクト編集".to_string(),
            description: "レイヤー・オブジェクト編集部分のフォントサイズ".to_string(),
            input_type: "sizeOnly".to_string(),
        },
        FontSetting {
            key: "TimeGauge".to_string(),
            label: "フレーム時間ゲージ".to_string(),
            description: "フレーム時間ゲージのフォントサイズ".to_string(),
            input_type: "sizeOnly".to_string(),
        },
        FontSetting {
            key: "Footer".to_string(),
            label: "フッター".to_string(),
            description: "フッターのフォントサイズ".to_string(),
            input_type: "sizeOnly".to_string(),
        },
        FontSetting {
            key: "TextEdit".to_string(),
            label: "テキスト編集".to_string(),
            description: "テキスト編集のフォント（等幅推奨）".to_string(),
            input_type: "both".to_string(),
        },
        FontSetting {
            key: "Log".to_string(),
            label: "ログ".to_string(),
            description: "ログのフォント（等幅推奨）".to_string(),
            input_type: "both".to_string(),
        },
    ]
}

#[tauri::command]
fn parse_font_value(value: String) -> FontValue {
    let parts: Vec<&str> = value.split(',').collect();
    FontValue {
        size: parts.get(0).unwrap_or(&"").to_string(),
        family: parts.get(1).unwrap_or(&"").to_string(),
    }
}

#[tauri::command]
fn format_font_value(size: String, family: String, key: String) -> String {
    let settings = get_font_settings();
    let setting = settings.iter().find(|s| s.key == key);
    
    if let Some(setting) = setting {
        let clean_family = family.split(',').next().unwrap_or("").trim();
        
        match setting.input_type.as_str() {
            "familyOnly" => clean_family.to_string(),
            "sizeOnly" => size,
            "both" => {
                if !clean_family.is_empty() {
                    format!("{},{}", size, clean_family)
                } else {
                    size
                }
            }
            _ => size,
        }
    } else {
        size
    }
}

fn parse_style_config(content: &str) -> Result<StyleConfig, String> {
    let mut config = StyleConfig {
        font: HashMap::new(),
        color: HashMap::new(),
        layout: HashMap::new(),
        format: HashMap::new(),
    };
    
    let mut current_section = "";
    let mut in_section = false;
    
    for line in content.lines() {
        let line = line.trim();
        
        // コメント行をスキップ
        if line.starts_with(';') || line.is_empty() {
            continue;
        }
        
        // セクション開始
        if line.starts_with('[') && line.ends_with(']') {
            current_section = &line[1..line.len()-1];
            in_section = true;
            continue;
        }
        
        if in_section {
            if let Some(equal_pos) = line.find('=') {
                let key = line[..equal_pos].trim();
                let value = line[equal_pos + 1..].trim();
                
                match current_section {
                    "Font" => { config.font.insert(key.to_string(), value.to_string()); }
                    "Color" => { config.color.insert(key.to_string(), value.to_string()); }
                    "Layout" => { config.layout.insert(key.to_string(), value.to_string()); }
                    "Format" => { config.format.insert(key.to_string(), value.to_string()); }
                    _ => {}
                }
            }
        }
    }
    
    Ok(config)
}

fn serialize_style_config(config: &StyleConfig) -> String {
    let mut content = String::new();
    
    // ヘッダーコメント
    content.push_str("; 外観の設定\n");
    content.push_str("; UTF-8で記述する\n");
    content.push_str("; ProgramData\\aviutl2\\style.confで設定を上書き出来る\n\n");
    
    // Font セクション
    if !config.font.is_empty() {
        content.push_str("[Font]\n");
        for (key, value) in &config.font {
            content.push_str(&format!("{}={}\n", key, value));
        }
        content.push('\n');
    }
    
    // Color セクション
    if !config.color.is_empty() {
        content.push_str("[Color]\n");
        for (key, value) in &config.color {
            content.push_str(&format!("{}={}\n", key, value));
        }
        content.push('\n');
    }
    
    // Layout セクション
    if !config.layout.is_empty() {
        content.push_str("[Layout]\n");
        for (key, value) in &config.layout {
            content.push_str(&format!("{}={}\n", key, value));
        }
        content.push('\n');
    }
    
    // Format セクション
    if !config.format.is_empty() {
        content.push_str("[Format]\n");
        for (key, value) in &config.format {
            content.push_str(&format!("{}={}\n", key, value));
        }
        content.push('\n');
    }
    
    content
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_style_config,
            save_style_config,
            get_default_style_path,
            get_user_style_path,
            open_file_location,
            get_system_fonts,
            get_font_settings,
            parse_font_value,
            format_font_value
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
