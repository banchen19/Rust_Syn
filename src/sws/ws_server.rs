use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use rocket::yansi::Paint;
use serde_json;
use ws::{Handler, Handshake, Message, Request, Result};

use crate::sws::{
    msg_tools::{message_amin, MessageResult},
    ws_key::{decrypt, encrypt, generate_md5_key},
};

pub(crate) struct ServerHandler {
    pub(crate) out: ws::Sender,
    pub(crate) connections: Arc<Mutex<Vec<(ws::Sender, SystemTime)>>>,
}
impl Handler for ServerHandler {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        // 获取客户端地址
        let client_addr = shake.peer_addr;
        println!(
            "{}:  {}",
            Paint::yellow("通信端接受新连接,来自"),
            Paint::green(client_addr.unwrap())
        );
        let mut connections = self.connections.lock().unwrap();
        let current_time = SystemTime::now();
        connections.push((self.out.clone(), current_time));
        println!(
            "{}:  {}",
            Paint::yellow("当前已接受连接数量："),
            Paint::green(connections.len())
        );

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Text(text) => msgdecrypt(text.as_bytes().to_vec(), self),
            Message::Binary(bytes) => msgdecrypt(bytes, self),
        }

        // 在收到客户端的消息时，更新连接的时间戳
        let mut connections = self.connections.lock().unwrap();
        let current_time = SystemTime::now();
        // 查找并更新对应的连接时间戳
        for (sender, time) in connections.iter_mut() {
            if sender == &self.out {
                *time = current_time;
                // 获取当前时间
                let now = SystemTime::now();
                // 检查是否超过10分钟
                if now
                    .duration_since(*time)
                    .unwrap_or(Duration::new(0, 0))
                    .as_secs()
                    >= 600
                {
                    println!("超过10分钟，断开连接");
                    let _ = sender.close(ws::CloseCode::Other(404));
                }
                break;
            }
        }
        Ok(())
    }

    fn on_close(&mut self, _code: ws::CloseCode, _reason: &str) {
        let mut connections: std::sync::MutexGuard<'_, Vec<(ws::Sender, SystemTime)>> =
            self.connections.lock().unwrap();
        connections.retain(|sender: &(ws::Sender, SystemTime)| {
            sender.0.connection_id() != self.out.connection_id()
        });
        println!(
            "{}: {}",
            Paint::yellow("检测到通信客户端连接断开,更新连接数"),
            Paint::green(connections.len())
        );
    }
}

// 消息解密
fn msgdecrypt(msg_string: Vec<u8>, _server_handler: &mut ServerHandler) {
    match decrypt(&msg_string) {
        Ok(pt) => {
            let decrypted_string = String::from_utf8(pt.to_vec()).unwrap();
            println!("解密结果（字符串）：{}", decrypted_string);
            // 转交给消息处理
            match message_amin(decrypted_string) {
                MessageResult::Success(data,update_sw) => {
                    if update_sw {
                        send_key(data, _server_handler);//送去加密消息
                    }
                }
                MessageResult::UnknownType => {
                    let _ = _server_handler.out.close(ws::CloseCode::Other(404));
                }
            }
        }
        Err(_) => {
            // 无法解密断开
            let _ = _server_handler.out.close(ws::CloseCode::Other(2));
        }
    }
}

// 加密数据——发送消息
fn send_key(byte_msg: Vec<u8>, _server_handler: &mut ServerHandler) {
    match encrypt(&byte_msg) {
        Ok(ct) => {
            // 发送消息
            to_send_chat_bds(_server_handler, ct.clone());
        }
        Err(_) => {
            let _ = _server_handler.out.close(ws::CloseCode::Other(2));
        }
    }
}

// 发送消息：为二进制
fn to_send_chat_bds(_server_handler: &mut ServerHandler, data: Vec<u8>) {
    let binding = _server_handler.connections.clone();
    let connections_arr: std::sync::MutexGuard<'_, Vec<(ws::Sender, SystemTime)>> =
        binding.lock().unwrap();

    for sender in connections_arr.iter() {
        if _server_handler.out != sender.0 {
            if let Err(err) = sender.0.send(data.clone()) {
                println!("玩家发送消息发送消息失败: {:?}", err);
                let _ = sender.0.close(ws::CloseCode::Other(404));
            }
        };
    }
}
