// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub appid: String,
    pub key: String,
    pub source_lang: String,
    pub target_lang: String,
    pub enable_logging: bool,
}

const EXAMPLE_CONF: &str = r#"
appid = "your appid"
key = "your key"
source_lang = "auto"
target_lang = "zh"
enable_logging = false
"#;

use std::fs::write;

fn create_conf() -> Result<(), std::io::Error> {
    write("config.toml", EXAMPLE_CONF)
}

use toml;

pub fn save_conf_with_debug(
    appid: &str,
    key: &str,
    source_lang: &str,
    target_lang: &str,
    enable_logging: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let conf = AppConfig {
        appid: appid.to_string(),
        key: key.to_string(),
        source_lang: source_lang.to_string(),
        target_lang: target_lang.to_string(),
        enable_logging,
    };
    let conf_str = toml::to_string(&conf)?;
    write("config.toml", conf_str)?;
    Ok(())
}

pub enum ConfigResult {
    Ok(AppConfig),
    Err(String),
}

fn load_file() -> ConfigResult {
    if !std::path::Path::new("config.toml").exists() {
        return ConfigResult::Err("config.toml not found, generating...".to_string());
    } else if std::path::Path::new("config.toml").is_dir() {
        return ConfigResult::Err(
            "config.toml is not a file, please delete it and try again.".to_string(),
        );
    }

    let raw = std::fs::read_to_string("config.toml");
    let raw_c = match raw {
        Ok(raw) => raw,
        Err(e) => return ConfigResult::Err(format!(":( Unable to read config.toml: {}", e)),
    };
    let read_result = toml::from_str::<AppConfig>(&raw_c);
    match read_result {
        Ok(config) => ConfigResult::Ok(config),
        Err(e) => ConfigResult::Err(format!(":( Unable to parse config.toml: {}", e)),
    }
}

pub fn try_init_conf() -> Result<AppConfig, String> {
    match load_file() {
        ConfigResult::Ok(config) => Ok(config),
        ConfigResult::Err(e) => {
            // 如果配置文件不存在，尝试创建示例配置文件
            if e.contains("not found") {
                match create_conf() {
                    Ok(_) => {
                        // 创建成功后，解析示例配置并返回
                        match toml::from_str::<AppConfig>(EXAMPLE_CONF) {
                            Ok(default_config) => Ok(default_config),
                            Err(parse_err) => {
                                Err(format!(":( Unable to parse example config: {}", parse_err))
                            }
                        }
                    }
                    Err(create_err) => {
                        Err(format!(":( Unable to create config.toml: {}", create_err))
                    }
                }
            } else {
                // 其他错误情况，直接返回错误
                Err(e)
            }
        }
    }
}
