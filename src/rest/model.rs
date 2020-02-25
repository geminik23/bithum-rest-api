
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::authenticate::*;

// use chrono::{DateTime, Utc};


#[derive(Clone, Deserialize, Debug)]
pub struct BithResponse{
    pub status:String,
    pub data:Option<serde_json::Value>,
    pub message:Option<String>,
    pub order_id:Option<String>,
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
    BithumbError(u16, Option<BithResponse>),
    Unknown(u16, Option<BithResponse>),
    ParameterError(u16, Option<BithResponse>), //400
    Unauthorized(u16, Option<BithResponse>), //401
    AccessDenied(u16, Option<BithResponse>), //403
    NotFound(u16, Option<BithResponse>), //404 .. if not found order when deleting
}

pub type RestTypedResult<T> = Result<T, RestError>;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum OrderType {
    #[serde(rename = "bid")]
    Bid,
    #[serde(rename = "ask")]
    Ask,
}
impl Default for OrderType{
    fn default() -> Self{ OrderType::Bid }
}


//========================================= API args


//
// INFO

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


#[derive(Clone, Debug, Serialize, Default)]
pub struct OrdersParam{
    pub order_currency:String,
    pub order_id:Option<String>,
    #[serde(rename = "type")]
    pub order_type:Option<OrderType>,
    pub count:Option<u16>, // 1~1000(default 100)
    pub after:Option<i64>,
    pub payment_currency:Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OrderResponse{
    pub order_currency:String,
    pub payment_currency:String,
    pub order_id:String,
    pub order_date:String, // integer
    #[serde(rename = "type")]
    pub order_type:OrderType,
    pub units:String,
    pub units_remaining:String, //number
    pub price:String, //number
}



pub type OrdersResponse = Vec<OrderResponse>;



//
// TRADE


#[derive(Clone, Debug, Serialize, Default)]
pub struct PlaceParam{
    pub order_currency:String,
    pub payment_currency:String,
    pub units:f64,
    pub price:u64,
    #[serde(rename = "type")]
    pub order_type:OrderType,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct CancelParam{
    #[serde(rename = "type")]
    pub order_type:OrderType,
    pub order_id:String,
    pub order_currency:String,
    pub payment_currency:String,
}