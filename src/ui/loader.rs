// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use cursive::{Cursive, CursiveExt};

pub fn ui_main() {
    let mut siv = Cursive::default();

    let mut theme = siv.current_theme().clone();
    theme
        .palette
        .set_color("Background", cursive::theme::Color::Rgb(200, 230, 230));
    theme
        .palette
        .set_color("View", cursive::theme::Color::Rgb(220, 250, 250));
    siv.set_theme(theme);

    siv.add_layer(crate::ui::index::build_main_view());

    siv.set_global_callback('t', |s| {
        crate::ui::index::translate_with_ask(s);
    });
    siv.set_global_callback('c', |s| {
        crate::ui::index::clear_texts(s);
    });
    siv.set_global_callback('s', |s| {
        s.add_layer(crate::ui::settings::build_settings_view());
    });
    siv.set_global_callback('h', |s| {
        s.add_layer(crate::ui::help::build_help_view());
    });
    siv.set_global_callback('a', |s| {
        s.add_layer(crate::ui::about::build_about_view());
    });
    siv.set_global_callback('q', |s| {
        s.quit();
    });

    siv.run();
}
