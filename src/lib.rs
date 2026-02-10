// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use lazy_static::lazy_static;
use log::{Level, Log, Metadata, Record, SetLoggerError};
use std::fs::OpenOptions;
use std::sync::Mutex;

lazy_static! {
    pub static ref LOG_FILE: Mutex<std::fs::File> = {
        Mutex::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open("btcli_app.log")
                .expect("无法创建日志文件"),
        )
    };
}

// 自定义日志宏 - 仅在启用调试时记录日志
#[macro_export]
macro_rules! log_to_file {
    ($($arg:tt)*) => {
        if let Ok(config) = crate::conf::try_init_conf() {
            if config.enable_logging {
                if let Ok(mut file) = crate::LOG_FILE.lock() {
                    use std::io::Write;
                    // 使用 write! 替代 writeln!，手动添加换行符和处理编码
                    let log_msg = format!("[{}] {}\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), format_args!($($arg)*));
                    let _ = file.write_all(log_msg.as_bytes());
                    let _ = file.flush(); // 确保立即写入
                }
            }
        } else {
            // 如果无法加载配置，也记录日志（这可能是早期阶段）
            if let Ok(mut file) = crate::LOG_FILE.lock() {
                use std::io::Write;
                // 使用 write! 替代 writeln!，手动添加换行符和处理编码
                let log_msg = format!("[{}] {}\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), format_args!($($arg)*));
                let _ = file.write_all(log_msg.as_bytes());
                let _ = file.flush(); // 确保立即写入
            }
        }
    };
}

// 与 Rust 标准日志系统兼容的文件日志记录器
pub struct FileLogger;

impl Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // 检查配置是否启用了日志记录
        if let Ok(config) = crate::conf::try_init_conf() {
            config.enable_logging && metadata.level() <= Level::Info
        } else {
            // 如果无法加载配置，默认启用
            metadata.level() <= Level::Info
        }
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Ok(mut file) = LOG_FILE.lock() {
                use std::io::Write;
                let log_msg = format!(
                    "[{}] [{}] {}\n",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                );
                let _ = file.write_all(log_msg.as_bytes());
                let _ = file.flush();
            }
        }
    }

    fn flush(&self) {
        if let Ok(file) = LOG_FILE.lock() {
            let _ = file.sync_all();
        }
    }
}

impl FileLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_boxed_logger(Box::new(FileLogger))
            .map(|()| log::set_max_level(log::LevelFilter::Info))
    }
}

// 导出必要的模块
pub mod cli;
pub mod conf;
pub mod expect_react;
pub mod extract_help;
pub mod fancy_egg;
pub mod fycore;
pub mod fyerrcodes;

// 仅在启用UI特性时包含UI模块
#[cfg(feature = "ui")]
pub mod ui;
mod fydocsrv;
