use super::ws_config::{SendMsg, WsChat};

// Define an enum to represent the three possible states
pub enum MessageResult {
    Success(Vec<u8>, bool),
    UnknownType,
}

// 消息接受，类型处理
pub fn message_amin(text: String) -> MessageResult {
    match serde_json::from_str::<WsChat>(&text.clone()) {
        Ok(ws_chat) => {
            // 处理消息类型
            match ws_chat.r#type.as_str() {
                "chat" => {
                    // 消息转发
                    match serde_json::from_value::<SendMsg>(ws_chat.data.clone()) {
                        Ok(str) => {
                            let chat = WsChat {
                                r#type: "chat".to_owned(),
                                data: ws_chat.data,
                            };
                            println!("发送者：{},消息内容：{}",str.name,str.msg);
                            let json_string: String = serde_json::to_string(&chat).unwrap();
                            MessageResult::Success(rebyte(json_string), true)
                        }
                        Err(_) => MessageResult::UnknownType,
                    }
                }
                "pl_join" => MessageResult::Success(rebyte("".to_owned()), false),
                "pl_update" => MessageResult::Success(rebyte("".to_owned()), true),
                _ => {
                    println!("未知通信类型");
                    MessageResult::UnknownType
                }
            }
        }
        Err(err) => {
            println!("解析错误");
            MessageResult::UnknownType
        }
    }
}

fn rebyte(str: String) -> Vec<u8> {
    str.as_bytes().to_vec()
}
