
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::authenticate::*;

// use chrono::{DateTime, Utc};


#[derive(Clone, Deserialize, Debug)]
pub struct BithResponse{
    pub status:String,
    pub data:Option<serde_json::Value>,
    pub message:Option<String>,
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
    Error(u16, Option<BithResponse>),
    Unknown(u16, Option<BithResponse>),
    ParameterError(u16, Option<BithResponse>), //400
    Unauthorized(u16, Option<BithResponse>), //401
    AccessDenied(u16, Option<BithResponse>), //403
    NotFound(u16, Option<BithResponse>), //404 .. if not found order when deleting
}

pub type RestTypedResult<T> = Result<T, RestError>;


//========================================= API args

#[derive(Clone, Debug, Serialize, Default)]
pub struct AccountParam{
    pub order_currency:String,
    pub payment_currency:Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AccountResponse{
    pub created:String, //integer
    pub account_id:String,
    pub order_currency:String,
    pub payment_currency:String,
    pub trade_fee:String, //number
    pub balance:String, //number
}
