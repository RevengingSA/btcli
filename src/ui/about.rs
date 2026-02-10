// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(feature = "ui")]
use cursive::views::{Dialog, TextView};

#[cfg(feature = "ui")]
pub fn build_about_view() -> Dialog {
    let about_text =
        "btcli - 百度翻译命令行工具\n\n版本: 0.5.7\n作者: S.A. (@snoware)\n许可证: MPL-2.0";

    Dialog::around(TextView::new(about_text))
        .title("关于")
        .button("关闭", |s| {
            let _ = s.pop_layer();
        })
}