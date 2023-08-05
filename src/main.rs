use mysql::Pool;
use rocket::catchers;
//rocket框架配置
use rocket::routes;
use rocket::Config as OtherConfig;

use colored::Colorize;
use rocket::yansi;
use scom::com::com_mg;
use sql::Sql_Util::create_table;
use var_config::def_Config::inti_config;

// 结构体引用
mod scom;
mod var_config;
use var_config::def_Config::Config;
use ws::listen;

mod sql;

use std::io;
// 异步
use std::{
    sync::{Arc, Mutex},
    thread,
};

// http服务端
mod shttp;
use crate::shttp::http_money::delmoney;
use crate::shttp::http_money::getplmoney;
use crate::shttp::http_player::*;

// ws服务端
mod sws;
use crate::sws::ws_server::ServerHandler;

use yansi::Paint; //转换u数组为字符串

//MySQL的Pool连接-全局变量
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref POOL: Mutex<Option<Pool>> = Mutex::new(None);
    static ref CONFIG_VAR: Mutex<Option<Config>> = Mutex::new(None);
}
// 加密测试
use rand::Rng;
// 异步启动
#[tokio::main]
async fn main() {
    inti_play().await
    
}

async fn inti_play() {
    match inti_config() {
        Ok(config) => {
            *CONFIG_VAR.lock().unwrap() = Some(config.clone());
            create_table(config.clone()).await; //创建数据库
            let ws_server_task: tokio::task::JoinHandle<tokio::task::JoinHandle<()>> =
                tokio::spawn(start_ws_server(config.clone()));
            let http_server_task: tokio::task::JoinHandle<tokio::task::JoinHandle<()>> =
                tokio::spawn(start_http_server(config));
            let _handle_input: () = handle_input(ws_server_task, http_server_task)
                .await
                .to_owned();
        }
        Err(err) => {
            println!("读取配置文件失败：{}", err);
            println!(
                "{}",
                Paint::yellow("新旧配置文件冲突，请备份原有数据并重新运行")
            );
        }
    }
    // 等待线程完成
    thread::sleep(std::time::Duration::from_secs(1));
}
async fn handle_input(
    ws_server_task: tokio::task::JoinHandle<tokio::task::JoinHandle<()>>,
    http_server_task: tokio::task::JoinHandle<tokio::task::JoinHandle<()>>,
) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("请输入正确指令");
        // 去除输入字符串两端的空格和换行符
        let command = input.trim();
        match command {
            "stop" => {
                ws_server_task.abort();
                http_server_task.abort();
                println!("{}", Paint::red("Rust_Syn服务端: 已经停止"));
                std::process::exit(0);
            }
            "help" => {
                let mut text = format!("{}: 帮助\n", Paint::yellow("help"));
                text += &format!("{}: 停止服务端\n", Paint::yellow("stop"));
                text += &format!(
                    "{}: 添加玩家数据（格式：addplayer <玩家名字>  <权限等级>）\n",
                    Paint::yellow("addplayer")
                );
                text += &format!(
                    "{}: 删除玩家数据（格式：delplayer <玩家名字>）\n",
                    Paint::yellow("delplayer")
                );
                text += &format!(
                    "{}: 修改玩家权限等级（格式：changeplevel <玩家名字>  <权限等级>）\n",
                    Paint::yellow("changeplevel")
                );
                println!("{}", Paint::green(text));
            }
            _ => match com_mg(input) {
                Ok(_) => {
                    println!("{}", Paint::green("执行成功"));
                }
                Err(_str) => {
                    println!("{}", Paint::red(_str))
                }
            },
        }
    }
}

// 启动ws端
async fn start_ws_server(config: Config) -> tokio::task::JoinHandle<()> {
    // 启动 WebSocket 服务器
    let ws_server_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        let connections = Arc::new(Mutex::new(Vec::new()));
        if let Err(error) = listen(format!("0.0.0.0:{}", config.ws_port), |out| ServerHandler {
            out,
            connections: connections.clone(),
        }) {
            // 通知用户故障
            println!("创建 WebSocket 失败，原因: {:?}", error);
        }
    });

    println!(
        "{} {}",
        Paint::yellow("通信服务端-端口:"),
        Paint::green(config.ws_port)
    );
    ws_server_task
}

// 启动http端
async fn start_http_server(config: Config) -> tokio::task::JoinHandle<()> {
    // 启动 HTTP 服务器
    let http_server_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        let config = OtherConfig::figment()
            .merge(("address", "0.0.0.0"))
            .merge(("port", config.http_port));
        let _ = rocket::custom(config)
            .mount("/addplayer", routes![addplayer])
            .mount("/deleteplayer", routes![deleteplayer])
            .mount("/deleteplayer_me", routes![deleteplayer_me])
            .mount("/getinformation_all", routes![getinformation_all])
            .mount("/getpllogin", routes![getpllogin])
            .mount("/delmoney", routes![delmoney])
            .mount("/getplmoney", routes![getplmoney])
            .mount("/login", routes![login])
            .register("/", catchers![not_found])
            // .mount("/", routes![index])
            .launch()
            .await;
    });
    println!(
        "{} {}",
        Paint::yellow("请求服务端-端口:"),
        Paint::green(config.http_port)
    );
    http_server_task
}
