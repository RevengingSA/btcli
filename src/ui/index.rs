// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::ui::*;
use cursive::traits::{Nameable, Resizable};
use cursive::{
    Cursive,
    views::{Button, Dialog, LinearLayout, TextArea, TextView},
};

use std::cell::Cell;

// 使用 Cell 来安全地存储可变状态
thread_local! {
    static ASK_ABOUT_SETTINGS: Cell<bool> = Cell::new(true);
}

pub fn build_main_view() -> LinearLayout {
    let mut layout = LinearLayout::vertical();

    // 创建带标签的输入区域
    let input_layout = LinearLayout::horizontal()
        .child(TextView::new("源文字: ").fixed_width(10))
        .child(
            TextArea::new()
                .with_name("input_textarea")
                .min_size((30, 5)),
        );

    // 创建带标签的输出区域
    let output_layout = LinearLayout::horizontal()
        .child(TextView::new("下面翻译结果: ").fixed_width(10))
        .child(
            TextView::new("")
                .with_name("output_textview")
                .min_size((30, 5)),
        );

    let button_row = LinearLayout::horizontal()
        .child(Button::new("[翻译(T)]", |s| translate_with_ask(s)))
        .child(Button::new("[清空(C)]", |s| clear_texts(s)))
        .child(Button::new("[查看设置(V)]", |s| {
            s.add_layer(settings::build_view_only_settings_view());
            // 延迟填充设置，确保UI控件已完全加载
            s.cb_sink()
                .send(Box::new(|s| {
                    settings::populate_view_only_settings_view(s);
                }))
                .unwrap_or(());
        }))
        .child(Button::new("[帮助(H)]", |s| {
            s.add_layer(help::build_help_view())
        }))
        .child(Button::new("[关于(A)]", |s| {
            s.add_layer(about::build_about_view())
        }))
        .child(Button::new("[退出(Q)]", |s| s.quit()));

    layout.add_child(input_layout);
    layout.add_child(output_layout);
    layout.add_child(button_row);

    layout
}

fn translate(s: &mut Cursive) {
    // 先获取输入内容，避免生命周期问题
    let input_content_opt = s.call_on_name("input_textarea", |view: &mut TextArea| {
        view.get_content().to_string() // 转换为拥有所有权的String
    });

    // 检查是否有内容
    let input_content = match input_content_opt {
        Some(content) => {
            if content.trim().is_empty() {
                lovely_items::show_error(s, "请输入要翻译的文本");
                return;
            }
            content
        }
        None => {
            lovely_items::show_error(s, "无法获取输入文本");
            return;
        }
    };

    // 获取配置
    let config = match crate::conf::try_init_conf() {
        Ok(config) => config,
        Err(error_msg) => {
            lovely_items::show_error(s, &error_msg);
            return;
        }
    };

    // 执行翻译
    match crate::fycore::translate(
        &config.appid,
        &config.source_lang,
        &config.target_lang,
        &input_content,
        config.clone(),
    ) {
        Ok(result) => {
            s.call_on_name("output_textview", |view: &mut TextView| {
                view.set_content(&result);
            });
        }
        Err(error_msg) => {
            lovely_items::show_error(s, &error_msg);
        }
    }
}

pub fn translate_with_ask(s: &mut Cursive) {
    if ASK_ABOUT_SETTINGS.with(|ask| ask.get()) {
        let dialog = Dialog::new()
            .title("翻译设置")
            .content(cursive::views::TextView::new("是否需要修改翻译设置？"))
            .button("是", |s| {
                s.add_layer(settings::build_view_only_settings_view());
                // 延迟填充设置，确保UI控件已完全加载
                s.cb_sink()
                    .send(Box::new(|s| {
                        settings::populate_view_only_settings_view(s);
                    }))
                    .unwrap_or(());
            })
            .button("否，本次不再询问", |s| {
                ASK_ABOUT_SETTINGS.with(|ask| ask.set(false));
                s.pop_layer();
                translate(s);
            })
            .button("否", |s| {
                s.pop_layer();
                translate(s);
            });

        s.add_layer(dialog);
    } else {
        translate(s);
    }
}

pub fn clear_texts(s: &mut Cursive) {
    s.call_on_name("input_textarea", |view: &mut TextArea| view.set_content(""));
    s.call_on_name("output_textview", |view: &mut TextView| {
        view.set_content("");
    });
}
