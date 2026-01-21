// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

//太机械化了，用了(ai)
const ERRORS_KEYPAIR: [(usize, &str); 15] = [
    (52000, "成功"),
    (
        52001,
        "请求超时，检查传入的 q 参数是否是正常文本，以及 from 或 to 参数是否在支持的语种列表中",
    ),
    (52002, "系统错误，请重试"),
    (
        52003,
        "未授权用户，请检查appid是否正确，或是否已开通对应服务服务是否开通",
    ),
    (54000, "必填参数为空，请检查是否漏传、误传参数"),
    (54001, "签名错误，请检查签名生成方法是否有误"),
    (
        54003,
        "访问频率受限，请降低您的调用频率，或在管理控制台进行身份认证后切换为高级版/尊享版",
    ),
    (
        54004,
        "账户余额不足，请前往管理控制台为账户充值。如后台显示还有余额，说明当天用量计费金额已超过账户余额",
    ),
    (
        54005,
        "长query请求频繁，请降低长度大于1万字节query的发送频率，3s后再试",
    ),
    (
        58000,
        "客户端IP非法，检查开发者信息页面填写的对应服务器IP地址是否正确，如服务器为动态IP，建议留空不填",
    ),
    (
        58001,
        "译文语言方向不支持，检查译文语言是否在语言列表里，个人标准版和高级版支持28个常见语种，企业尊享版支持全部语种",
    ),
    (58002, "服务当前已关闭，请前往管理控制台开启服务"),
    (
        58003,
        "此IP已被封禁，同一IP当日使用多个APPID发送翻译请求，则该IP将被封禁当日请求权限，次日解封。请勿将APPID和密钥填写到第三方软件中。",
    ),
    (90107, "认证未通过或未生效，请前往我的认证查看认证进度"),
    (
        20003,
        "请求内容存在安全风险，请检查请求文本是否涉及反动，暴力等相关内容",
    ),
];

pub fn gen_err_map() -> HashMap<usize, &'static str> {
    HashMap::from(ERRORS_KEYPAIR)
}

pub fn query_msg(code: usize) -> &'static str {
    *gen_err_map().get(&code).unwrap_or(&"未知错误")
}
