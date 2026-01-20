// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::Debug;

pub trait ResultExt<T> {
    fn show_error(self) -> T;
    fn handle_error<F>(self, handler: F) -> T
    where
        F: FnOnce(&str);
    fn show_error_in_ui(self, s: &mut cursive::Cursive) -> T;
}

impl<T, E: Debug> ResultExt<T> for Result<T, E> {
    fn show_error(self) -> T {
        match self {
            Ok(value) => value,
            Err(err) => {
                let error_msg = format!("Error: {:?}", err);
                eprintln!("{}", error_msg);
                std::process::exit(1);
            }
        }
    }

    fn handle_error<F>(self, handler: F) -> T
    where
        F: FnOnce(&str),
    {
        match self {
            Ok(value) => value,
            Err(err) => {
                let error_msg = format!("{:?}", err);
                handler(&error_msg);
                std::process::exit(1);
            }
        }
    }

    fn show_error_in_ui(self, s: &mut cursive::Cursive) -> T {
        match self {
            Ok(value) => value,
            Err(err) => {
                let error_msg = format!("Error: {:?}", err);
                crate::ui::messagebox::show_error(s, &error_msg);
                std::process::exit(1);
            }
        }
    }
}

pub trait OptionExt<T> {
    fn expect_or_exit(self, msg: &str) -> T;
    fn handle_none<F>(self, msg: &str, handler: F) -> T
    where
        F: FnOnce(&str);
    fn show_none_in_ui(self, s: &mut cursive::Cursive, msg: &str) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    fn expect_or_exit(self, msg: &str) -> T {
        match self {
            Some(value) => value,
            None => {
                eprintln!("{}", msg);
                std::process::exit(1);
            }
        }
    }

    fn handle_none<F>(self, msg: &str, handler: F) -> T
    where
        F: FnOnce(&str),
    {
        match self {
            Some(value) => value,
            None => {
                handler(msg);
                std::process::exit(1);
            }
        }
    }

    fn show_none_in_ui(self, s: &mut cursive::Cursive, msg: &str) -> T {
        match self {
            Some(value) => value,
            None => {
                crate::ui::messagebox::show_error(s, msg);
                std::process::exit(1);
            }
        }
    }
}
