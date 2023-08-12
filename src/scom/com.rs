use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    sql::{Sql_Util::*, sqlite3_util::getmoney_name_sqlite3},
    var_config::{def_Config::DefPlayer, yml_util::generate_random_key},
    CONFIG_VAR,
};

use chrono::{Datelike, Local, Timelike, Utc};
use rocket::{yansi::Paint, futures::future::ok};

pub fn com_mg(com_str: String) -> Result<(), String> {
    let mut params = com_str.trim().split_whitespace();
    if let Some(command) = params.next() {
        match command {
            "addplayer" => {
                // Extracting player name and permission level from the input
                if let (Some(player_name), Some(permission_level_str)) =
                    (params.next(), params.next())
                {
                    if let Ok(permission_level) = permission_level_str.parse::<i32>() {
                        println!("添加玩家: {} 权限等级为: {}", player_name, permission_level);
                        let config = CONFIG_VAR
                            .lock()
                            .unwrap()
                            .as_ref()
                            .expect("CONFIG_VAR not initialized")
                            .clone();
                        let key = generate_random_key(12);
                        let current_time = Local::now();
                        let formatted_time = format!(
                            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                            current_time.year(),
                            current_time.month(),
                            current_time.day(),
                            current_time.hour(),
                            current_time.minute(),
                            current_time.second()
                        );
                        let player = DefPlayer {
                            name: player_name.to_string(),
                            pw: key.clone(),
                            level: permission_level,
                            prefix: "null".to_owned(),
                            online: 0,
                            ip: "null".to_owned(),
                            time: formatted_time,
                            money: 0,
                        };
                        add_player(config, player);
                        println!(
                            "添加成功:\n玩家：{}\n初始密码为：{}\n等级为: {}",
                            Paint::green(player_name),
                            Paint::green(key),
                            Paint::green(permission_level)
                        );
                        return Ok(());
                    } else {
                        return Err(String::from("无效的权限等级，请输入一个整数。"));
                    }
                } else {
                    return Err(String::from(
                        "命令格式不正确，请输入：addplayer <玩家名字>  <权限等级>",
                    ));
                }
            }
            "delplayer" => {
                // Extracting player name from the input
                if let Some(player_name) = params.next() {
                    let config = CONFIG_VAR
                        .lock()
                        .unwrap()
                        .as_ref()
                        .expect("CONFIG_VAR not initialized")
                        .clone();
                    match getplayer_information(config.clone(), player_name.to_owned()) {
                        Ok(_def_player) => {
                            match deleteplayer_me_sql(config, _def_player.name) {
                                Ok(_) => Ok(()),
                                Err(_) => Err(String::from("小概率事件触发，几乎他妈的不可能触发")),
                            }
                        }
                        _ => Err(String::from("玩家不存在")),
                    }
                } else {
                    return Err(String::from("命令格式不正确，请输入：delplayer <玩家名字>"));
                }
            }
            "changeplevel" => {
                // Extracting player name and new permission level from the input
                if let (Some(player_name), Some(permission_level_str)) =
                    (params.next(), params.next())
                {
                    if let Ok(permission_level) = permission_level_str.parse::<i32>() {
                        let config = CONFIG_VAR
                            .lock()
                            .unwrap()
                            .as_ref()
                            .expect("CONFIG_VAR not initialized")
                            .clone();
                        match getplayer_information(config.clone(), player_name.to_owned()) {
                            Ok(mut _def_player) => {
                                _def_player.level = permission_level;
                                match update_player_level(
                                    config.clone(),
                                    player_name.clone().to_owned(),
                                    permission_level,
                                ) {
                                    Ok(_) => Ok(()),
                                    Err(_) => Err(String::from(
                                        "修改失败：小概率事件触发，几乎他妈的不可能触发",
                                    )),
                                }
                            }
                            _ => Err(String::from("玩家不存在")),
                        }
                    } else {
                        return Err(String::from("无效的权限等级，请输入一个整数。"));
                    }
                } else {
                    return Err(String::from(
                        "命令格式不正确，请输入：changeplevel <玩家名字>  <新权限等级>",
                    ));
                }
            }
            "test"=>
            {
                
                Ok(())
            }
            _ => {
                return Err(String::from("无效的命令，请输入有效的命令。"));
            }
        }
    } else {
        return Err(String::from("请输入命令。"));
    }
}
