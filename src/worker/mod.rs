use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Clone, Debug, Deserialize)]
pub struct Param{
    pub api:String,
    pub secret:String,
    pub order_symbol:String,
    pub payment_symbol:String,
    pub units:f64,
    pub price:f64,
    pub order_type:super::rest::OrderType,
}


#[derive(Clone, Debug, Deserialize)]
pub enum RequestOp{
    Ping,
    OrderRequest,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WorkerRequest{
    pub op:RequestOp,
    pub uid:String,
    pub arg:Option<Value>,
}

#[derive(Clone, Debug, Serialize)]
pub struct WorkerResponse{
    pub success:bool,
    pub uid:String,
    pub message:Option<String>,
    pub arg:Option<String>,
}


