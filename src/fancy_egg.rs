// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate base64;

use base64::{Engine as _, engine::general_purpose};

pub static EGG_CODE: &str = "U2Fs5Zac5qyiQ3lyYQ==";

pub fn decrypt(encoded_str: &str) -> String {
    // 使用base64 crate进行解码
    let decoded_bytes = general_purpose::STANDARD.decode(encoded_str).unwrap();

    // 将解码后的字节转换为字符串（如果已知原始数据是文本）
    String::from_utf8(decoded_bytes).unwrap()
}

use rand::Rng;

/// 获取一句随机祝福诗词
///
/// 该函数从预定义的25句祝福诗词中随机选择一句返回
/// 诗词内容涵盖自然意境、人生哲思、事业前程、情感祝福及健康生活五大类
///
/// # 返回值
///
/// 返回一个包含随机祝福诗词的字符串
pub fn get_random_blessing() -> String {
    // 定义25句不落俗套的祝福诗词
    const BLESSINGS: [&str; 25] = [
        // 自然意境类
        "星河入梦夜无尘，愿君心事皆成真",
        "松风拂面春常在，竹影摇窗月自明",
        "云卷千峰秀，水涵万顷清",
        "落花不扫春犹在，流水无声韵自长",
        "晨露润青苔，幽兰自吐芳",
        // 人生哲思类
        "行路不问归期远，心灯长照夜航船",
        "半卷诗书藏岁月，一壶清茶话平生",
        "不争朝夕争朝暮，但守初心守岁寒",
        "浮名散作千林雪，真意凝成一砚冰",
        "世事如棋局局新，心安即是旧时邻",
        // 事业前程类
        "笔底波澜生万象，胸中丘壑纳千峰",
        "云程发轫春山外，星轨初开夜海深",
        "织锦当从经纬始，凌云先自寸心谋",
        "商海浮沉持舵稳，文峰登览踏云轻",
        "创新不惧千山阻，破茧方知一羽轻",
        // 情感祝福类
        "相逢何必曾相识，一笑能消万古愁",
        "情若幽兰深谷里，香随明月到君前",
        "两心相照如明镜，半世同行胜管弦",
        "家书不寄千行字，灯火相依便是春",
        "故人如月常相伴，新友如星渐满天",
        // 健康与生活类
        "身似青松经岁寒，心如止水映天宽",
        "茶烟轻袅春常在，竹影婆娑病自消",
        "行路不辞山水远，看花长伴岁时新",
        "一枕清风眠自在，半窗明月梦无边",
        "食有粗茶淡饭味，居无闹市远尘喧",
    ];

    // 使用rand库生成随机索引
    let random_index = rand::rng().random_range(0..BLESSINGS.len());

    // 返回随机选择的祝福语
    BLESSINGS[random_index].to_string()
}
