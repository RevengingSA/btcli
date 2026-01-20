// Copyright (C) 2026 S.A. (@snoware)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::conf::AppConfig;
use crate::fyerrcodes::query_msg;
use md5;
use rand;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;
use ureq;

/// 提高程序可读性，避免硬编码host
const HOST: &str = "https://fanyi-api.baidu.com/api/trans/vip/translate";

/// 定义响应体数据结构
// 成功
#[derive(Deserialize, Debug)]
struct TranslationResponse {
    from: String,
    to: String,
    trans_result: Vec<TranslationItem>,
}

#[derive(Deserialize, Debug)]
struct TranslationItem {
    src: String,
    dst: String,
}

//失败
#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error_code: serde_json::Value,
    error_msg: String,
}

fn calculate_sign(appid: &str, q: &str, salt: &str, secret_key: &str) -> String {
    let sign_str = format!("{}{}{}{}", appid, q, salt, secret_key);
    format!("{:x}", md5::compute(sign_str.as_bytes()))
}

fn send_response(
    appid: &str,
    from: &str,
    to: &str,
    q: &str,
    app_config: &AppConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let salt = rand::random::<u32>().to_string();
    let sign = calculate_sign(appid, q, &salt, &app_config.secret_key);

    let mut params = HashMap::new();
    params.insert("appid", appid);
    params.insert("from", from);
    params.insert("to", to);
    params.insert("q", q);
    params.insert("salt", &salt);
    params.insert("sign", &sign);

    let response = ureq::post(HOST)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send_form(&params)?;

    let response_text = response.into_body().read_to_string()?;
    Ok(response_text)
}

///已经确定成功，从body里获取返回结果
fn patch_raw(content: String) -> Result<String, String> {
    let parsed_response = serde_json::from_str::<TranslationResponse>(&content)
        .map_err(|e| format!(":( Failed to parse successful response: {}", e))?;

    if !parsed_response.trans_result.is_empty() {
        Ok(parsed_response.trans_result[0].dst.clone())
    } else {
        Err(":( Response contains no translation results".to_string())
    }
}

///解析错误响应
fn patch_error(body_content: String) -> Result<ErrorResponse, String> {
    serde_json::from_str::<ErrorResponse>(&body_content)
        .map_err(|e| format!(":( Failed to parse error response: {}", e))
}

/// 核心翻译函数 - 单次翻译
pub fn translate(
    appid: &str,
    from: &str,
    to: &str,
    q: &str,
    app_config: AppConfig,
) -> Result<String, String> {
    // 控制请求频率
    std::thread::sleep(std::time::Duration::from_millis(100));

    // 发送请求并获取响应
    match send_response(appid, from, to, q, &app_config) {
        Err(e) => {
            return Err(format!(":( error sending request: {}", e.to_string()));
        }
        Ok(response_body) => {
            // 检查是否包含错误信息
            if response_body.contains("error_msg") {
                // 解析错误响应
                match patch_error(response_body) {
                    Ok(err_resp) => {
                        // 处理 error_code 可能是字符串或数字的情况
                        let errcode = match err_resp.error_code.as_u64() {
                            Some(code) => code as usize,
                            None => {
                                // 如果是字符串，则尝试转换
                                match err_resp.error_code.as_str() {
                                    Some(code_str) => match usize::from_str(code_str) {
                                        Ok(code) => code,
                                        Err(_) => {
                                            return Err(":( failed to parse error code".to_string());
                                        }
                                    },
                                    None => return Err(":( failed to parse error code".to_string()),
                                }
                            }
                        };

                        // 返回错误信息
                        return Err(format!(
                            ":( We asked, but server said: {}",
                            query_msg(errcode)
                        ));
                    }
                    Err(e) => return Err(e),
                }
            } else {
                // 翻译成功，返回解析后的结果
                return patch_raw(response_body);
            }
        }
    }
}
