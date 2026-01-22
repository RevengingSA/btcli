// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;

// 引入lib.rs中的模块和宏
use btcli_lib::*;
mod cli;

fn main() {
    // 初始化日志系统 - 根据配置决定是否记录日志
    if let Err(_) = FileLogger::init() {
        // 如果初始化失败，至少保留原有的日志记录方式
        log_to_file!("警告：无法初始化文件日志记录器");
    }

    log_to_file!("应用程序启动");

    let args: Vec<String> = env::args().collect();

    // 如果提供了命令行参数，则使用纯命令行模式
    if args.len() > 1 {
        run_cli_mode(&args[1..]);
    } else {
        // 否则运行UI模式
        log_to_file!("启动UI模式");
        crate::ui::loader::ui_main();
        log_to_file!("UI模式结束");
    }
    log_to_file!("应用程序结束");
}

fn run_cli_mode(args: &[String]) {
    let cli_args = cli::parse_args(args);
    
    if cli_args.help {
        cli::show_help();
        return;
    }
    
    if cli_args.version {
        cli::show_version();
        return;
    }
    
    if cli_args.text.is_empty() {
        eprintln!("错误: 请提供要翻译的文本");
        cli::show_help();
        return;
    }
    
    log_to_file!("启动CLI模式，参数: {:?}", cli_args);
    
    // 尝试加载配置
    let config = match crate::conf::try_init_conf() {
        Ok(config) => {
            log_to_file!(
                "成功加载配置: AppID={}, TargetLang={}",
                config.appid,
                config.target_lang
            );
            config
        }
        Err(error_msg) => {
            log_to_file!("配置加载失败: {}", error_msg);
            eprintln!("配置错误: {}", error_msg);
            return;
        }
    };
    
    // 使用命令行参数覆盖配置中的语言设置
    let source_lang = cli_args.source_lang.unwrap_or(config.source_lang.clone());
    let target_lang = cli_args.target_lang.unwrap_or(config.target_lang.clone());

    // 执行翻译
    match crate::fycore::translate(
        &config.appid,
        &source_lang,
        &target_lang,
        &cli_args.text,
        config.clone(),
    ) {
        Ok(result) => {
            log_to_file!("翻译成功完成");
            println!("{}", result);
        }
        Err(error_msg) => {
            log_to_file!("翻译失败: {}", error_msg);
            eprintln!("翻译错误: {}", error_msg);
        }
    }
    log_to_file!("CLI模式结束");
}
