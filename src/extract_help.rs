// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Help information module for btcli
//! Contains language lists, error codes and special messages

const HELP_INFO: &str = r#"=== btcli - 百度翻译命令行工具帮助信息 ===

【常见语种列表】
名称        代码    名称        代码    名称        代码
自动检测      auto   中文        zh      英语        en
粤语        yue    文言文       wyw     日语        jp
韩语        kor    法语        fra     西班牙语      spa
泰语        th     阿拉伯语      ara     俄语        ru
葡萄牙语      pt     德语        de      意大利语      it
希腊语       el     荷兰语       nl      波兰语       pl
保加利亚语     bul    爱沙尼亚语     est     丹麦语       dan
芬兰语       fin    捷克语       cs      罗马尼亚语     rom
斯洛文尼亚语    slo    瑞典语       swe     匈牙利语      hu
繁体中文      cht    越南语       vie     

【错误码列表】
错误码    含义              解决方案
52000    成功              
52001    请求超时            检查传入的 q 参数是否是正常文本，以及 from 或 to 参数是否在支持的语种列表中
52002    系统错误            请重试
52003    未授权用户           请检查appid是否正确，或是否已开通对应服务服务是否开通
54000    必填参数为空          请检查是否漏传、误传参数
54001    签名错误            请检查签名生成方法是否有误
54003    访问频率受限          请降低您的调用频率，或在管理控制台进行身份认证后切换为高级版/尊享版
54004    账户余额不足          请前往管理控制台为账户充值。如后台显示还有余额，说明当天用量计费金额已超过账户余额
54005    长query请求频繁       请降低长度大于1万字节query的发送频率，3s后再试
58000    客户端IP非法         检查开发者信息页面填写的对应服务器IP地址是否正确，如服务器为动态IP，建议留空不填
58001    译文语言方向不支持      检查译文语言是否在语言列表里，个人标准版和高级版支持28个常见语种，企业尊享版支持全部语种
58002    服务当前已关闭         请前往管理控制台开启服务
58003    此IP已被封禁          同一IP当日使用多个APPID发送翻译请求，则该IP将被封禁当日请求权限，次日解封。请勿将APPID和密钥填写到第三方软件中。
90107    认证未通过或未生效       请前往我的认证查看认证进度
20003    请求内容存在安全风险     请检查请求文本是否涉及反动，暴力等相关内容

【其他信息】
- API可通过百度翻译开放平台申请
- 文本、cli翻译放在寒假哦
- 代码本人全部重看
- S.A. 2026-1-20 与你同往
"#;

static ENCRYPTED_MESSAGE: &[u8] = &[
    0xBC, 0xD2, 0xCB, 0xBF, 0xCC, 0xC6, 0xBC, 0xF6, 0xF8, 0xB3, 0xFB, 0xE4, 0xBF, 0xD7, 0xD9, 0xBF,
    0xF5, 0xE1,
];

fn decrypt_message(encrypted: &[u8]) -> String {
    let decrypted_bytes: Vec<u8> = encrypted.iter().map(|&b| b ^ 0x5A).collect();
    String::from_utf8_lossy(&decrypted_bytes).to_string()
}

/// 获取帮助信息
pub fn get_help_info() -> &'static str {
    HELP_INFO
}

/// 获取解密后的特殊消息
pub fn get_secret_message() -> String {
    decrypt_message(ENCRYPTED_MESSAGE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_message() {
        let decrypted = get_secret_message();
        assert!(!decrypted.is_empty());
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let original = "我喜欢顾千寻";
        let encrypted = original.bytes().map(|b| b ^ 0x5A).collect::<Vec<u8>>();
        let decrypted = decrypt_message(&encrypted);
        assert_eq!(original, decrypted);
    }
}
