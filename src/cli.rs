// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;

/// 命令行参数结构
#[derive(Debug, Clone)]
pub struct CliArgs {
    pub text: String,
    pub source_lang: Option<String>,
    pub target_lang: Option<String>,
    pub help: bool,
    pub version: bool,
}

impl CliArgs {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            source_lang: None,
            target_lang: None,
            help: false,
            version: false,
        }
    }
}

/// 解析命令行参数
pub fn parse_args(args: &[String]) -> CliArgs {
    let mut cli_args = CliArgs::new();
    let mut i = 0;
    
    while i < args.len() {
        let arg = &args[i];
        
        match arg.as_str() {
            "-h" | "--help" => {
                cli_args.help = true;
                i += 1;
            }
            "-v" | "--version" => {
                cli_args.version = true;
                i += 1;
            }
            "-s" | "--source" => {
                if i + 1 < args.len() {
                    cli_args.source_lang = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("错误: --source 需要指定源语言参数");
                    std::process::exit(1);
                }
            }
            "-t" | "--target" => {
                if i + 1 < args.len() {
                    cli_args.target_lang = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("错误: --target 需要指定目标语言参数");
                    std::process::exit(1);
                }
            }
            _ => {
                // 如果不是选项，则认为是翻译文本的一部分
                if cli_args.text.is_empty() {
                    cli_args.text = arg.clone();
                } else {
                    cli_args.text.push(' ');
                    cli_args.text.push_str(arg);
                }
                i += 1;
            }
        }
    }
    
    cli_args
}

/// 显示帮助信息
pub fn show_help() {
    println!(
        "btcli - 命令行翻译工具\n\n\
         用法: btcli [选项] <文本>\n\n\
         选项:\n\
         -s, --source LANG    指定源语言 (例如: en, zh)\n\
         -t, --target LANG    指定目标语言 (例如: en, zh)\n\
         -h, --help          显示此帮助信息\n\
         -v, --version       显示版本信息\n\n\
         示例:\n\
         btcli \"Hello world\"                 # 翻译文本\n\
         btcli -t zh \"Hello world\"          # 翻译为中文\n\
         btcli -s en -t zh \"Hello world\"   # 指定源语言和目标语言\n"
    );
}

/// 显示版本信息
pub fn show_version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("btcli version {}", version);
}