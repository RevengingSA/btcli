// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use cursive::{
    Cursive,
    views::{Dialog, TextView},
};

/// 构建错误窗口对话框
///
/// # 参数
/// - `error_message`: 要显示的错误信息
///
/// # 返回
/// 返回一个Dialog实例，可直接添加到Cursive界面中
pub fn messagebox(title: &str, message: &str) -> Dialog {
    Dialog::around(TextView::new(message))
        .title(title)
        .button("确定", |s| {
            s.pop_layer();
        })
}

/// 显示错误窗口的便捷函数
///
/// # 参数
/// - `s`: Cursive实例引用
/// - `error_message`: 要显示的错误信息
pub fn show_error(s: &mut Cursive, error_message: &str) {
    s.add_layer(messagebox("错误", error_message));
}
