// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(feature = "ui")]
use crate::extract_help;
#[cfg(feature = "ui")]
use cursive::traits::{Resizable, Scrollable};
#[cfg(feature = "ui")]
use cursive::views::{Dialog, ScrollView, TextView};

#[cfg(feature = "ui")]
pub fn build_help_view() -> Dialog {
    let help_text = extract_help::get_help_info();

    let scrollable_text = ScrollView::new(TextView::new(help_text))
        .scrollable()
        .fixed_height(20)
        .fixed_width(60);

    Dialog::around(scrollable_text)
        .title("帮助")
        .button("关闭", |s| {
            let _ = s.pop_layer();
        })
} //没事不要用简写的闭包，头疼的要死，难看随他难看