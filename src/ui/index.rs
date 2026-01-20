use crate::conf;
use crate::fycore;
use crate::ui::*;
use cursive::traits::{Nameable, Resizable};
use cursive::{
    Cursive,
    views::{Button, Dialog, EditView, LinearLayout, TextView},
};

static mut IS_Q_PRESSED: bool = false;
static mut ASK_ABOUT_SETTINGS: bool = true;

pub fn build_main_view() -> LinearLayout {
    let mut layout = LinearLayout::vertical();

    let input_edit = EditView::new().with_name("input_edit").fixed_height(10);

    let output_edit = TextView::new("").with_name("output_edit").fixed_height(10);

    let button_row = LinearLayout::horizontal()
        .child(Button::new("[翻译(T)]", |s| translate_with_ask(s)))
        .child(Button::new("[清空(C)]", |s| clear_texts(s)))
        .child(Button::new("[设置(S)]", |s| {
            s.add_layer(settings::build_settings_view())
        }))
        .child(Button::new("[帮助(H)]", |s| {
            s.add_layer(help::build_help_view())
        }))
        .child(Button::new("[关于(A)]", |s| {
            s.add_layer(about::build_about_view())
        }))
        .child(Button::new("[退出(Q)]", |s| s.quit()));

    layout.add_child(input_edit);
    layout.add_child(output_edit);
    layout.add_child(button_row);

    layout
}

fn translate(s: &mut Cursive) {
    let input_text = s
        .call_on_name("input_edit", |view: &mut EditView| view.get_content())
        .unwrap_or_default();

    if input_text.is_empty() {
        messagebox::show_error(s, "请输入要翻译的文本");
        return;
    }

    let config = match conf::try_init_conf() {
        Ok(config) => config,
        Err(error_msg) => {
            messagebox::show_error(s, &error_msg);
            return;
        }
    };

    match fycore::translate(
        &config.appid,
        &config.source_lang,
        &config.target_lang,
        &input_text,
        config.clone(),
    ) {
        Ok(result) => {
            s.call_on_name("output_edit", |view: &mut TextView| {
                view.set_content(&result);
            });
        }
        Err(error_msg) => {
            messagebox::show_error(s, &error_msg);
        }
    }
}

pub fn translate_with_ask(s: &mut Cursive) {
    if unsafe { ASK_ABOUT_SETTINGS } {
        let dialog = Dialog::new()
            .title("翻译设置")
            .content(cursive::views::TextView::new("是否需要修改翻译设置？"))
            .button("是", |s| {
                s.add_layer(settings::build_settings_view());
                s.pop_layer();
            })
            .button("否，本次不再询问", |s| {
                unsafe {
                    ASK_ABOUT_SETTINGS = false;
                }
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
    s.call_on_name("input_edit", |view: &mut EditView| view.set_content(""));
    s.call_on_name("output_edit", |view: &mut TextView| {
        view.set_content("");
    });
}

pub fn handle_q_a_combo(s: &mut Cursive) {
    use crate::extract_help;
    let secret_message = extract_help::get_secret_message();
    messagebox::show_error(s, &secret_message);
}

pub fn check_q_a_combo_for_q(s: &mut Cursive) {
    unsafe {
        IS_Q_PRESSED = true;
    }
}

pub fn check_q_a_combo_for_a(s: &mut Cursive) {
    if unsafe { IS_Q_PRESSED } {
        unsafe {
            IS_Q_PRESSED = false;
        }
        handle_q_a_combo(s);
    } else {
        unsafe {
            IS_Q_PRESSED = false;
        }
    }
}
