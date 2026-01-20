// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod conf;
mod expect_react;
mod extract_help;
mod fycore;
mod fyerrcodes;
mod ui;

fn main() {
    crate::ui::loader::ui_main();
}
