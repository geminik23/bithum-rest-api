//https://github.com/geminik23/bm-tracker

use serde::{Deserialize, Serialize};
use std::sync::{Arc,Mutex};


pub struct BithumbHandler{
    pub out: Arc<Mutex<ws::Sender>>,
}

impl BithumbHandler{
    pub fn subscribe_ticker(&self, symbols:Vec<String>, tick_types:Option<Vec<String>>){
        
        let req = WSRequest{
            filter_type:FilterType::Ticker,
            symbols:symbols,
            tick_types:tick_types
        };
        if let Ok(r) = serde_json::to_string(&req){
            debug!("subscribe ticker {:?}", r);
            self.out.lock().unwrap().send(r).unwrap();
        }
    }

    pub fn subscribe_transaction(&self, symbols:Vec<String>){
        let req = WSRequest{
            filter_type:FilterType::Transaction,
            symbols:symbols,
            tick_types:None
        };

        if let Ok(r) = serde_json::to_string(&req){
            self.out.lock().unwrap().send(r).unwrap();
        }
    }

    pub fn subscribe_orderbook(&self, symbols:Vec<String>){
        let req = WSRequest{
            filter_type:FilterType::Orderbookdepth,
            symbols:symbols,
            tick_types:None
        };

        if let Ok(r) = serde_json::to_string(&req){
            self.out.lock().unwrap().send(r).unwrap();
        }
    }
}

pub struct Connector{}

pub struct InnerHandler<T: Listener>{
    pub listener:T,
    pub out: Arc<Mutex<ws::Sender>>,
    pub bith_handler:BithumbHandler,
}


const RES_MSG_CONNECTED:&'static str="Connected Successfully";
const RES_MSG_SUCCESS_FILTER:&'static str="Filter Registered Successfully";
const RES_MSG_FAILED_FILTER:&'static str="Invalid Filter Syntax";

#[derive(Clone, Debug)]
pub enum ReqResult{
    Connected,
    SuccessfullyRegistered,
    InvalidFilterSyntax,
}

#[derive(Clone, Deserialize, Debug)]
pub struct WSResultResponseT{
    pub status:String,
    pub resmsg:String,
}

#[derive(Clone, Debug)]
pub struct WSResultResponse{
    pub status:String,
    pub res:ReqResult,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub enum FilterType{
    #[serde(rename = "ticker")]
    Ticker,
    #[serde(rename = "transaction")]
    Transaction,
    #[serde(rename = "orderbookdepth")]
    Orderbookdepth,
}

#[derive(Clone, Deserialize, Debug)]
pub struct WSResponse{
    #[serde(rename = "type")]
    pub filter_type:FilterType,
    pub content:serde_json::Value,
}


#[derive(Clone, Serialize, Debug)]
pub struct WSRequest{
    #[serde(rename = "type")]
    pub filter_type:FilterType,
    pub symbols:Vec<String>,
    #[serde(skip_serializing)]
    #[serde(rename = "tickTypes")]
    pub tick_types:Option<Vec<String>>,
}


// RESPONSES


#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionElement{
    pub symbol:String,
    pub buy_sell_gb:String,
    pub cont_price:String,
    pub cont_qty:String,
    pub cont_amt:String,
    pub cont_dtm:String,
    pub updn:String,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookElement{
    pub symbol:String,
    pub order_type:String,
    pub price:String,
    pub quantity:String,
    pub total:String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct TickRes{

}

#[derive(Clone, Deserialize, Debug)]
pub struct TransactionRes{
    pub list:Vec<TransactionElement>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct OrderbookdepthRes{
    pub list:Vec<OrderbookElement>,
    pub datetime:String,
}


pub trait Listener{
    fn on_opened(&mut self, bith:&BithumbHandler);
    fn on_error(&mut self, err:&ws::Error);
    fn on_close(&mut self, bith:&BithumbHandler);
    fn on_request_resut(&mut self, bith:&BithumbHandler, res:String);
    fn on_ticker(&mut self, bith:&BithumbHandler, res:TickRes);
    fn on_transaction(&mut self, bith:&BithumbHandler, res:TransactionRes);
    fn on_orderbook(&mut self, bith:&BithumbHandler, res:OrderbookdepthRes);
}

