use rocket::{catch, get, http::Status, post, Request, Responder};
// 自定义状态码并返回数据
use crate::{shttp::httpGetResponder_tools::HttpGetResponder, sql::Sql_Util::*, CONFIG_VAR}; // 添加引用

use serde_json::from_str;

use super::{httpGetResponder_tools::*, http_player_config::AddPlayer};

// 删除经济体
#[get("/?<moneysName>&<key>")]
pub fn delmoney(moneysName: Option<String>, key: Option<String>) -> HttpGetResponder {
    let moeny_name = moneysName.unwrap_or_default();
    let key = key.unwrap_or_default();
    println!("moeny_name: {}", moeny_name);
    println!("key: {}", key);

    let config = CONFIG_VAR
        .lock()
        .unwrap()
        .as_ref()
        .expect("CONFIG_VAR not initialized")
        .clone();

    match deletemoney(config.clone(), moeny_name.clone(), key.clone()) {
        Ok(_) => null_200_http_get_responder(),
        Err(_) => null_403_http_get_responder(), // Return 404 if player not found
    }
}

// 获取个人信息经济余额
#[get("/?<name>&<moeny_name>")]
pub fn getplmoney(name: Option<String>, moeny_name: Option<String>) -> HttpGetResponder {
    let name = name.unwrap_or_default();
    let moeny_name = moeny_name.unwrap_or_default();
    println!("name: {}", name);
    println!("moeny_name: {}", moeny_name);

    let config = CONFIG_VAR
        .lock()
        .unwrap()
        .as_ref()
        .expect("CONFIG_VAR not initialized")
        .clone();

    match getplayer_information_money(config, name, moeny_name) {
        Ok(player) => {
            let json_data = serde_json::to_value(&player.money).unwrap();
            let status = Status::Ok;
            let message = serde_json::to_string(&Response {
                code: 200,
                message: json_data,
            })
            .unwrap();
            HttpGetResponder((status, message))
        }
        _ => null_403_http_get_responder(),
    }
}

//查询所有经济体以及密钥

// 查询经济体
#[get("/?<name>&<pw>")]
pub fn getmoney(name: Option<String>, pw: Option<String>) -> HttpGetResponder {
    let name = name.unwrap_or_default();
    let pw = pw.unwrap_or_default();
    println!("name: {}", name);
    println!("pw: {}", pw);

    let config = CONFIG_VAR
        .lock()
        .unwrap()
        .as_ref()
        .expect("CONFIG_VAR not initialized")
        .clone();

    match getplayer_information(config.clone(), name) {
        Ok(player) => {
            if player.pw == pw {
                if player.level >= config.getmoney {
                    match getmoney_name(config) {
                            Ok(str) =>
                            {
                                let json_data = serde_json::to_value(str).unwrap();
                                let status = Status::Ok;
                                let message = serde_json::to_string(&Response {
                                    code: 200,
                                    message: json_data,
                                })
                                .unwrap();
                                HttpGetResponder((status, message))
                            },
                            Err(_) =>
                            {
                                null_403_http_get_responder()
                            },
                        
                    }
                } else {
                    match getmoney_name_pl(config) {
                        Ok(str) =>
                        {
                            let json_data = serde_json::to_value(str).unwrap();
                            let status = Status::Ok;
                            let message = serde_json::to_string(&Response {
                                code: 200,
                                message: json_data,
                            })
                            .unwrap();
                            HttpGetResponder((status, message))
                        },
                        Err(_) =>
                        {
                            null_403_http_get_responder()
                        },
                    
                }
                }
            } else {
                null_403_http_get_responder()
            }
        }
        _ => null_403_http_get_responder(),
    }
}
