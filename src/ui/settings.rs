use crate::ui::messagebox;
use cursive::Cursive;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{Button, Dialog, EditView, LinearLayout, TextView};

pub fn build_settings_view() -> Dialog {
    let settings_layout = LinearLayout::vertical()
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("AppID: ").fixed_width(10))
                .child(EditView::new().with_name("appid").fixed_width(20)),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("API Key: ").fixed_width(10))
                .child(EditView::new().with_name("key").fixed_width(20)),
        )
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("默认源语言: ").fixed_width(10))
                .child(EditView::new().with_name("source_lang").fixed_width(20)),
        )
        .child(
            LinearLayout::horizontal()
                .child(Button::new("保存", |s| save_settings(s)))
                .child(Button::new("返回", |s| {
                    let _ = s.pop_layer();
                })),
        );

    Dialog::around(settings_layout).title("设置")
}

fn save_settings(s: &mut Cursive) {
    // 获取输入并保存
    let appid = s
        .call_on_name("appid", |view: &mut EditView| view.get_content())
        .unwrap_or_default();
    let key = s
        .call_on_name("key", |view: &mut EditView| view.get_content())
        .unwrap_or_default();
    let source_lang = s
        .call_on_name("source_lang", |view: &mut EditView| view.get_content())
        .unwrap_or_default();

    // 验证输入
    if appid.is_empty() || key.is_empty() || source_lang.is_empty() {
        messagebox::show_error(s, "请填写所有字段");
        return;
    }

    // 保存到配置
    match crate::conf::save_conf(&appid, &key, &source_lang) {
        Ok(_) => {
            messagebox::show_error(s, "配置保存成功");
        }
        Err(e) => {
            messagebox::show_error(s, &format!("保存配置失败: {}", e));
        }
    }
}
