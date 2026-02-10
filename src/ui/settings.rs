// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(feature = "ui")]
use crate::ui::lovely_items;
#[cfg(feature = "ui")]
use cursive::Cursive;
#[cfg(feature = "ui")]
use cursive::traits::{Nameable, Resizable};
#[cfg(feature = "ui")]
use cursive::views::{Button, Checkbox, Dialog, EditView, LinearLayout, TextView};

#[cfg(feature = "ui")]
use log::debug;
#[cfg(feature = "ui")]
use std::fs;

#[cfg(feature = "ui")]
pub fn build_settings_view() -> Dialog {
    let settings_layout = LinearLayout::vertical()
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("AppID: ").fixed_width(10))
                .child(EditView::new().with_name("appid").fixed_width(20)),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("API Key: ").fixed_width(10))
                .child(EditView::new().with_name("key").fixed_width(20)),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("源语言: ").fixed_width(10))
                .child(EditView::new().with_name("source_lang").fixed_width(20)),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("目标语言: ").fixed_width(10))
                .child(EditView::new().with_name("target_lang").fixed_width(20)),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("开启调试: ").fixed_width(10))
                .child(
                    Checkbox::new()
                        .with_checked(false)
                        .with_name("enable_debug"),
                ),
        )
        .child(
            LinearLayout::horizontal()
                .child(Button::new("保存", |s| confirm_save_settings(s)))
                .child(Button::new("返回", |s| {
                    let _ = s.pop_layer();
                })),
        );

    let dialog = Dialog::around(settings_layout).title("设置");

    // 立即返回对话框，然后在下一个事件循环中填充配置
    // 但我们需要一种方式在对话框添加后填充配置
    dialog
}

// 添加只读设置视图
#[cfg(feature = "ui")]
pub fn build_view_only_settings_view() -> Dialog {
    let settings_layout = LinearLayout::vertical()
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("AppID: ").fixed_width(10))
                .child(TextView::new("").with_name("view_appid").fixed_width(20)),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("API Key: ").fixed_width(10))
                .child(TextView::new("").with_name("view_key").fixed_width(20)),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("目标语言: ").fixed_width(10))
                .child(
                    TextView::new("")
                        .with_name("view_source_lang")
                        .fixed_width(20),
                ),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("目标语言: ").fixed_width(10))
                .child(
                    TextView::new("")
                        .with_name("view_target_lang")
                        .fixed_width(20),
                ),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("开启调试: ").fixed_width(10))
                .child(
                    TextView::new("")
                        .with_name("view_enable_debug")
                        .fixed_width(20),
                ),
        )
        .child(
            LinearLayout::horizontal()
                .child(Button::new("修改设置", |s| {
                    // 移除当前层，显示编辑设置界面
                    s.pop_layer();
                    s.add_layer(build_settings_view());
                    // 延迟填充设置，确保UI控件已完全加载
                    s.cb_sink()
                        .send(Box::new(|s| {
                            populate_settings_view(s);
                        }))
                        .unwrap_or(());
                }))
                .child(Button::new("清理日志", |s| {
                    // 清理日志文件
                    clear_log_file(s);
                }))
                .child(Button::new("返回", |s| {
                    let _ = s.pop_layer();
                })),
        );

    let dialog = Dialog::around(settings_layout).title("查看设置");

    // 立即返回对话框，然后在下一个事件循环中填充配置
    dialog
}

// 清理日志文件的函数
#[cfg(feature = "ui")]
fn clear_log_file(s: &mut Cursive) {
    match fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("btcli_app.log")
    {
        Ok(_) => {
            log_to_file!("日志文件已清理");
            lovely_items::show_info(s, "日志文件已清理");
        }
        Err(e) => {
            log_to_file!("清理日志文件失败: {}", e);
            lovely_items::show_error(s, &format!("清理日志文件失败: {}", e));
        }
    }
}

// 专门用于设置初始值的函数
#[cfg(feature = "ui")]
pub fn populate_settings_view(s: &mut Cursive) {
    debug!("Attempting to populate settings view");
    log_to_file!("开始填充设置界面");

    match crate::conf::try_init_conf() {
        Ok(config) => {
            debug!("Configuration loaded successfully, populating fields");
            debug!("AppID: {}", config.appid);
            debug!("Key length: {}", config.key.len());
            debug!("Source Language: {}", config.source_lang);
            debug!("Target Language: {}", config.target_lang);
            debug!("Enable Logging: {}", config.enable_logging);

            log_to_file!(
                "成功加载配置，开始填充界面: AppID={}, SourceLang={}, TargetLang={}",
                config.appid,
                config.source_lang,
                config.target_lang
            );

            // 使用call_on_name来更新每个控件的值
            if let Some(_) = s.call_on_name("appid", |view: &mut EditView| {
                debug!("Setting appid field: {}", &config.appid);
                log_to_file!("设置AppID字段: {}", &config.appid);
                view.set_content(&config.appid);
            }) {
                debug!("Appid field updated");
                log_to_file!("AppID字段已更新");
            } else {
                debug!("Failed to update appid field - control may not exist");
                log_to_file!("更新AppID字段失败 - 控件可能不存在");
            }

            if let Some(_) = s.call_on_name("key", |view: &mut EditView| {
                debug!(
                    "Setting key field: {}",
                    format!("{}...", &config.key.chars().take(3).collect::<String>())
                );
                log_to_file!("设置API Key字段");
                view.set_content(&config.key);
            }) {
                debug!("Key field updated");
                log_to_file!("API Key字段已更新");
            } else {
                debug!("Failed to update key field - control may not exist");
                log_to_file!("更新API Key字段失败 - 控件可能不存在");
            }

            if let Some(_) = s.call_on_name("source_lang", |view: &mut EditView| {
                debug!("Setting source_lang field: {}", &config.source_lang);
                log_to_file!("设置目标语言字段: {}", &config.source_lang);
                view.set_content(&config.source_lang);
            }) {
                debug!("Source language field updated");
                log_to_file!("目标语言字段已更新");
            } else {
                debug!("Failed to update source_lang field - control may not exist");
                log_to_file!("更新目标语言字段失败 - 控件可能不存在");
            }

            if let Some(_) = s.call_on_name("target_lang", |view: &mut EditView| {
                debug!("Setting target_lang field: {}", &config.target_lang);
                log_to_file!("设置目标语言字段: {}", &config.target_lang);
                view.set_content(&config.target_lang);
            }) {
                debug!("Target language field updated");
                log_to_file!("目标语言字段已更新");
            } else {
                debug!("Failed to update target_lang field - control may not exist");
                log_to_file!("更新目标语言字段失败 - 控件可能不存在");
            }

            if let Some(_) = s.call_on_name("enable_debug", |view: &mut Checkbox| {
                debug!("Setting enable_debug checkbox: {}", config.enable_logging);
                log_to_file!("设置调试复选框: {}", config.enable_logging);
                view.set_checked(config.enable_logging);
            }) {
                debug!("Debug checkbox updated");
                log_to_file!("调试复选框已更新");
            } else {
                debug!("Failed to update enable_debug checkbox - control may not exist");
                log_to_file!("更新调试复选框失败 - 控件可能不存在");
            }
        }
        Err(e) => {
            debug!("Failed to load configuration: {}", e);
            log_to_file!("加载配置失败: {}", e);
            // 如果配置文件不存在或有错误，不填充任何值，让用户从头开始配置
        }
    }
}

// 专门用于填充只读设置视图的函数
#[cfg(feature = "ui")]
pub fn populate_view_only_settings_view(s: &mut Cursive) {
    debug!("Attempting to populate view-only settings view");
    log_to_file!("开始填充只读设置界面");

    match crate::conf::try_init_conf() {
        Ok(config) => {
            debug!("Configuration loaded successfully, populating view-only fields");
            debug!("AppID: {}", config.appid);
            debug!("Key length: {}", config.key.len());
            debug!("Source Language: {}", config.source_lang);
            debug!("Target Language: {}", config.target_lang);
            debug!("Enable Logging: {}", config.enable_logging);

            log_to_file!(
                "成功加载配置，开始填充只读界面: AppID={}, SourceLang={}, TargetLang={}",
                config.appid,
                config.source_lang,
                config.target_lang
            );

            // 使用call_on_name来更新每个只读控件的值
            if let Some(_) = s.call_on_name("view_appid", |view: &mut TextView| {
                debug!("Setting view_appid field: {}", &config.appid);
                log_to_file!("设置只读AppID字段: {}", &config.appid);
                view.set_content(&config.appid);
            }) {
                debug!("View appid field updated");
                log_to_file!("只读AppID字段已更新");
            } else {
                debug!("Failed to update view_appid field - control may not exist");
                log_to_file!("更新只读AppID字段失败 - 控件可能不存在");
            }

            if let Some(_) = s.call_on_name("view_key", |view: &mut TextView| {
                // 显示隐藏的密钥
                let masked_key = if config.key.len() > 6 {
                    let chars: Vec<char> = config.key.chars().collect();
                    let visible_part: String = chars.iter().take(3).collect();
                    let hidden_count = chars.len() - 6;
                    format!("{}***{}", visible_part, &config.key[hidden_count..])
                } else {
                    "*".repeat(config.key.len())
                };

                debug!("Setting view_key field: {}", &masked_key);
                log_to_file!("设置只读API Key字段");
                view.set_content(masked_key);
            }) {
                debug!("View key field updated");
                log_to_file!("只读API Key字段已更新");
            } else {
                debug!("Failed to update view_key field - control may not exist");
                log_to_file!("更新只读API Key字段失败 - 控件可能不存在");
            }

            if let Some(_) = s.call_on_name("view_source_lang", |view: &mut TextView| {
                debug!("Setting view_source_lang field: {}", &config.source_lang);
                log_to_file!("设置只读目标语言字段: {}", &config.source_lang);
                view.set_content(&config.source_lang);
            }) {
                debug!("View source language field updated");
                log_to_file!("只读目标语言字段已更新");
            } else {
                debug!("Failed to update view_source_lang field - control may not exist");
                log_to_file!("更新只读目标语言字段失败 - 控件可能不存在");
            }

            if let Some(_) = s.call_on_name("view_target_lang", |view: &mut TextView| {
                debug!("Setting view_target_lang field: {}", &config.target_lang);
                log_to_file!("设置只读目标语言字段: {}", &config.target_lang);
                view.set_content(&config.target_lang);
            }) {
                debug!("View target language field updated");
                log_to_file!("只读目标语言字段已更新");
            } else {
                debug!("Failed to update view_target_lang field - control may not exist");
                log_to_file!("更新只读目标语言字段失败 - 控件可能不存在");
            }

            if let Some(_) = s.call_on_name("view_enable_debug", |view: &mut TextView| {
                let debug_status = if config.enable_logging { "是" } else { "否" };
                debug!("Setting view_enable_debug field: {}", debug_status);
                log_to_file!("设置只读调试字段: {}", debug_status);
                view.set_content(debug_status);
            }) {
                debug!("View debug checkbox updated");
                log_to_file!("只读调试字段已更新");
            } else {
                debug!("Failed to update view_enable_debug field - control may not exist");
                log_to_file!("更新只读调试字段失败 - 控件可能不存在");
            }
        }
        Err(e) => {
            debug!("Failed to load configuration: {}", e);
            log_to_file!("加载配置失败: {}", e);
            // 如果配置文件不存在或有错误，显示提示信息
            let error_msg = format!("配置未设置: {}", e);
            s.call_on_name("view_appid", |view: &mut TextView| {
                view.set_content(&error_msg);
            });
            s.call_on_name("view_key", |view: &mut TextView| {
                view.set_content(&error_msg);
            });
            s.call_on_name("view_source_lang", |view: &mut TextView| {
                view.set_content(&error_msg);
            });
            s.call_on_name("view_target_lang", |view: &mut TextView| {
                view.set_content(&error_msg);
            });
            s.call_on_name("view_enable_debug", |view: &mut TextView| {
                view.set_content("未设置");
            });
        }
    }
}

#[cfg(feature = "ui")]
fn confirm_save_settings(s: &mut Cursive) {
    log_to_file!("显示保存确认对话框");
    lovely_items::show_confirmation(
        s,
        "确认保存",
        "您确定要保存这些设置吗？",
        |s| save_settings(s), // "是" 回调
        |_s| {},              // "否" 回调 - 什么也不做，返回到设置界面
    );
}

#[cfg(feature = "ui")]
fn save_settings(s: &mut Cursive) {
    log_to_file!("开始保存设置");
    // 获取输入并保存
    let appid = s
        .call_on_name("appid", |view: &mut EditView| view.get_content())
        .unwrap_or_default();
    let key = s
        .call_on_name("key", |view: &mut EditView| view.get_content())
        .unwrap_or_default();
    let source_lang = s
        .call_on_name("source_lang", |view: &mut EditView| view.get_content())
        .unwrap_or_default();
    let target_lang = s
        .call_on_name("target_lang", |view: &mut EditView| view.get_content())
        .unwrap_or_default();
    let enable_debug = s
        .call_on_name("enable_debug", |view: &mut Checkbox| view.is_checked())
        .unwrap_or_default();

    log_to_file!(
        "从UI获取的设置 - AppID: {}, Key长度: {}, SourceLang: {}, TargetLang: {}, EnableDebug: {}",
        appid,
        key.len(),
        source_lang,
        target_lang,
        enable_debug
    );

    // 验证输入
    if appid.is_empty() || key.is_empty() || source_lang.is_empty() || target_lang.is_empty() {
        log_to_file!("输入验证失败 - 存在空字段");
        lovely_items::show_error(s, "请填写所有字段");
        return;
    }

    // 保存到配置
    match crate::conf::save_conf_with_debug(&appid, &key, &source_lang, &target_lang, enable_debug)
    {
        Ok(_) => {
            log_to_file!(
                "配置保存成功 - AppID: {}, SourceLang: {}, TargetLang: {}",
                appid,
                source_lang,
                target_lang
            );
            // 先关闭确认对话框，再关闭设置窗口
            s.pop_layer(); // 关闭确认对话框
            s.pop_layer(); // 关闭设置窗口
            lovely_items::show_info(s, "配置保存成功");
        }
        Err(e) => {
            log_to_file!("配置保存失败: {}", e);
            lovely_items::show_error(s, &format!("保存配置失败: {}", e));
        }
    }
}
