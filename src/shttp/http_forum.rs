use crate::{sql::Sql_Util::*, CONFIG_VAR, shttp::httpGetResponder_tools::Response};

use super::{
    httpGetResponder_tools::{
        null_200_http_get_responder, null_403_http_get_responder, HttpGetResponder,
    },
    http_forum_config::Forum,
    http_player_config::AddForum,
};

use rocket::{catch, get, http::Status, post, Request, Responder, data::{Limits, ToByteUnit}};
use serde_json::from_str;

//初始化论坛数据
#[post("/", format = "application/json", data = "<str>")]
pub fn addforum(str: String) -> HttpGetResponder {
    match from_str::<AddForum>(&str) {
        Ok(_addforum) => {
            let config = CONFIG_VAR
                .lock()
                .unwrap()
                .as_ref()
                .expect("CONFIG_VAR not initialized")
                .clone();

            match getplayer_information(config.clone(), _addforum.name) {
                Ok(player) => match serde_json::from_value::<Forum>(_addforum.forum) {
                    Ok(forum) => {
                        match getplayer_information(config.clone(), forum.clone().sender) {
                            Ok(_player) => {
                                if player.pw == _addforum.pw {
                                    //TODO 初始化论坛数据
                                    add_forum(config,forum);
                                    null_200_http_get_responder()
                                } else {
                                    null_403_http_get_responder()
                                }
                            }
                            _ => null_403_http_get_responder(),
                        }
                    }
                    _ => null_403_http_get_responder(),
                },
                _ => null_403_http_get_responder(),
            }
        }
        Err(_) => null_403_http_get_responder(),
    }
}


// 获取全部玩家
#[get("/?<name>&<pw>")]
pub fn getinformation_all_forum(
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

    match getplayer_information(config.clone(), name.clone()) {
        Ok(player) => {
            if player.pw == pw{
                match getforumall(config) {
                    Ok(forums) => {
                        let json_data = serde_json::to_value(&forums).unwrap();
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
