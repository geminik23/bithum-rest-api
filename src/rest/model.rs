
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::authenticate::*;

// use chrono::{DateTime, Utc};


#[derive(Clone, Deserialize, Debug)]
pub  struct BithResponse{
    pub status:String,
    pub data:serde_json::Value,
}

pub struct Client{
    pub url:reqwest::Url,
    pub auth:Authenticate,
}


#[derive(Debug, Clone)]
pub enum RestError {
    ConnectionError,
    JsonParseError,
    ModelParseError,
    Error(u16),
    Unknown(u16),
    ParameterError(u16), //400
    Unauthorized(u16), //401
    AccessDenied(u16), //403
    NotFound(u16), //404 .. if not found order when deleting
}



