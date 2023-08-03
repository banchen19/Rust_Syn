// 自定义状态码并返回数据
#[derive(Responder)]
#[response(content_type = "json")]
pub struct HttpGetResponder(pub (rocket::http::Status, String));
use serde::{Deserialize, Serialize};
use rocket::{catch, get, http::Status, post, Request, Responder};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub(crate) code: u32,
    pub(crate) message: serde_json::Value,
}

/// 生成 404 Not Found 响应器，返回状态码 404 和消息体为 {"code": 404, "message": "null"} 的 JSON 数据。
fn null_404_http_get_responder() -> HttpGetResponder {
    let status = Status::NotFound;
    let message = serde_json::to_string(&Response {
        code: 404,
        message: serde_json::Value::String("null".to_string()),
    })
    .unwrap();
    HttpGetResponder((status, message))
}

/// 生成 400 Bad Request 响应器，返回状态码 400 和消息体为 {"code": 400, "message": "Bad Request"} 的 JSON 数据。
fn null_400_http_get_responder() -> HttpGetResponder {
    let status = Status::BadRequest;
    let message = serde_json::to_string(&Response {
        code: 400,
        message: serde_json::Value::String("Bad Request".to_string()),
    })
    .unwrap();
    HttpGetResponder((status, message))
}

/// 生成 403 Forbidden 响应器，返回状态码 403 和消息体为 {"code": 403, "message": "false"} 的 JSON 数据。
pub fn null_403_http_get_responder() -> HttpGetResponder {
    let status = Status::Forbidden;
    let message = serde_json::to_string(&Response {
        code: 403,
        message: serde_json::Value::String("false".to_string()),
    })
    .unwrap();
    HttpGetResponder((status, message))
}

/// 生成 409 Conflict 响应器，返回状态码 409 和消息体为 {"code": 409, "message": "Conflict"} 的 JSON 数据。
pub fn null_409_http_get_responder() -> HttpGetResponder {
    let status = Status::Conflict;
    let message = serde_json::to_string(&Response {
        code: 409,
        message: serde_json::Value::String("Conflict".to_string()),
    })
    .unwrap();
    HttpGetResponder((status, message))
}

/// 生成 200 OK 响应器，返回状态码 200 和消息体为 {"code": 200, "message": "true"} 的 JSON 数据。
pub fn null_200_http_get_responder() -> HttpGetResponder {
    let status = Status::Ok;
    let message = serde_json::to_string(&Response {
        code: 200,
        message: serde_json::Value::String("true".to_string()),
    })
    .unwrap();
    HttpGetResponder((status, message))
}

/// 生成 500 Internal Server Error 响应器，返回状态码 500 和消息体为 {"code": 500, "message": "error"} 的 JSON 数据。
pub fn null_500_http_get_responder() -> HttpGetResponder {
    let status = Status::InternalServerError;
    let message = serde_json::to_string(&Response {
        code: 500,
        message: serde_json::Value::String("error".to_string()),
    })
    .unwrap();
    HttpGetResponder((status, message))
}

/// 生成 401 Unauthorized 响应器，返回状态码 401 和消息体为 {"code": 401, "message": "unauthorized"} 的 JSON 数据。
pub fn null_401_http_get_responder() -> HttpGetResponder {
    let status = Status::Unauthorized;
    let message = serde_json::to_string(&Response {
        code: 401,
        message: serde_json::Value::String("unauthorized".to_string()),
    })
    .unwrap();
    HttpGetResponder((status, message))
}