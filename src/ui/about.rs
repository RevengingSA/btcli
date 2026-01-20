use cursive::views::{Dialog, TextView};

pub fn build_about_view() -> Dialog {
    let about_text =
        "btcli - 百度翻译命令行工具\n\n版本: 0.1.0\n作者: S.A. (@snoware)\n许可证: MPL-2.0";

    Dialog::around(TextView::new(about_text))
        .title("关于")
        .button("关闭", |s| {
            let _ = s.pop_layer();
        })
}
