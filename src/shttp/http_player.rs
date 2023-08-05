use std::ptr::null;

use rocket::{catch, get, http::Status, post, Request, Responder};
// 自定义状态码并返回数据
use crate::{shttp::httpGetResponder_tools::HttpGetResponder, sql::Sql_Util::*, CONFIG_VAR}; // 添加引用

use serde_json::from_str;

use super::{httpGetResponder_tools::*, http_player_config::AddPlayer};

// 访问位置路径返回
#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("抱歉, \n'{}' 这是一个未知路径", req.uri())
}

//初始化玩家数据
#[post("/", format = "application/json", data = "<player_str>")]
pub fn addplayer(player_str: String) -> HttpGetResponder {
    match from_str::<AddPlayer>(&player_str) {
        Ok(_addplayer) => {
            let config = CONFIG_VAR
                .lock()
                .unwrap()
                .as_ref()
                .expect("CONFIG_VAR not initialized")
                .clone();
            match getplayer_information(config.clone(), _addplayer.name) {
                Ok(player) => {
                    if player.player.pw == _addplayer.pw
                        && player.player.level > config.addplayer
                        && _addplayer.player.level < player.player.level
                    {
                        //管理员权限进行手动添加白名单
                        add_player(config, _addplayer.player);
                        null_200_http_get_responder()
                    } else if !config.whitelist {
                        //当白名单关闭的时候允许玩家自己注册
                        add_player(config, _addplayer.player);
                        null_403_http_get_responder()
                    } else {
                        null_403_http_get_responder()
                    }
                }
                _ => null_403_http_get_responder(),
            }
        }
        Err(_) => null_403_http_get_responder(),
    }
}

// 删除玩家数据
#[get("/?<name>&<pw>&<pl_name>")]
pub fn deleteplayer(
    name: Option<String>,
    pw: Option<String>,
    pl_name: Option<String>,
) -> HttpGetResponder {
    let name = name.unwrap_or_default();
    let pw = pw.unwrap_or_default();
    let pl_name = pl_name.unwrap_or_default();
    println!("name: {}", name);
    println!("pw: {}", pw);
    println!("pl_name: {}", pl_name);

    let config = CONFIG_VAR
        .lock()
        .unwrap()
        .as_ref()
        .expect("CONFIG_VAR not initialized")
        .clone();
    match getplayer_information(config.clone(), name) {
        Ok(player) => {
            if player.player.pw == pw && player.player.level > config.delplayer {
                //管理员权限进行手动删除白名单
                null_200_http_get_responder()
            } else {
                null_403_http_get_responder()
            }
        }
        _ => null_403_http_get_responder(),
    }
}

// 玩家自主注销
#[get("/?<name>&<pw>")]
pub fn deleteplayer_me(name: Option<String>, pw: Option<String>) -> HttpGetResponder {
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
    match getplayer_information(config.clone(), name.clone()) {
        Ok(player) => {
            if player.player.pw == pw && config.delplme {
                let _ = deleteplayer_me_sql(config, player.player.name);
                null_200_http_get_responder()
            } else {
                null_403_http_get_responder()
            }
        }
        _ => null_403_http_get_responder(),
    }
}

// 获取全部玩家
#[get("/?<name>&<pw>&<moeny_name>")]
pub fn getinformation_all(
    name: Option<String>,
    pw: Option<String>,
    moeny_name: Option<String>,
) -> HttpGetResponder {
    let name = name.unwrap_or_default();
    let pw = pw.unwrap_or_default();
    let moeny_name = moeny_name.unwrap_or_default();
    println!("name: {}", name);
    println!("pw: {}", pw);
    println!("moeny_name: {}", moeny_name);

    let config = CONFIG_VAR
        .lock()
        .unwrap()
        .as_ref()
        .expect("CONFIG_VAR not initialized")
        .clone();

    match getplayer_information(config.clone(), name.clone()) {
        Ok(player) => {
            if player.player.pw == pw && player.player.level > config.getplall {
                match get_player_all(config, moeny_name) {
                    Ok(Some(players)) => {
                        let json_data = serde_json::to_value(&players).unwrap();
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
            } else {
                null_403_http_get_responder()
            }
        }
        Err(_) => null_403_http_get_responder(), // Return 404 if player not found
    }
}

// 登录以获取个人信息
#[get("/?<name>&<pw>&<moeny_name>")]
pub fn getpllogin(
    name: Option<String>,
    pw: Option<String>,
    moeny_name: Option<String>,
) -> HttpGetResponder {
    let name = name.unwrap_or_default();
    let pw = pw.unwrap_or_default();
    let moeny_name = moeny_name.unwrap_or_default();
    println!("name: {}", name);
    println!("pw: {}", pw);
    println!("moeny_name: {}", moeny_name);

    let config = CONFIG_VAR
        .lock()
        .unwrap()
        .as_ref()
        .expect("CONFIG_VAR not initialized")
        .clone();

    match getplayer_information_money(config, name, moeny_name) {
        Ok(player) => {
            if player.player.pw == pw {
                let json_data = serde_json::to_value(&player).unwrap();
                let status = Status::Ok;
                let message = serde_json::to_string(&Response {
                    code: 200,
                    message: json_data,
                })
                .unwrap();
                HttpGetResponder((status, message))
            } else {
                null_403_http_get_responder()
            }
        }
        _ => null_403_http_get_responder(),
    }
}

// 验证密码
#[get("/?<name>&<pw>")]
pub fn login(
    name: Option<String>,
    pw: Option<String>,
) -> HttpGetResponder {
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
            println!("{}",player.player.pw);
            println!("{}",pw);
            if player.player.pw == pw {
                null_200_http_get_responder()
            } else {
                null_403_http_get_responder()
            }
        }
        _ => null_403_http_get_responder(),
    }
}
