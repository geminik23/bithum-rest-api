#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
#[macro_use] extern crate serde_json;
extern crate anyhow;


extern crate ws;
use serde::{Deserialize, Serialize};


const RES_MSG_CONNECTED:&'static str="Connected Successfully";
const RES_MSG_SUCCESS_FILTER:&'static str="Filter Registered Successfully";
const RES_MSG_FAILED_FILTER:&'static str="Invalid Filter Syntax";


#[derive(Clone, Deserialize, Debug)]
pub struct WSResponse{
    pub status:String,
    pub resmsg:String,
}

#[derive(Clone, Serialize, Debug)]
pub enum FilterType{
    #[serde(rename = "ticker")]
    Ticker,
    #[serde(rename = "transaction")]
    Transaction,
    #[serde(rename = "orderbookdepth")]
    Orderbookdepth,
}

#[derive(Clone, Serialize, Debug)]
pub struct WSRequest{
    #[serde(rename = "type")]
    pub filter_type:FilterType,
    pub symbols:Vec<String>,
    #[serde(rename = "tickTypes")]
    pub tick_types:Option<Vec<String>>,
}






pub trait Client{
    fn is_connected(&self)->bool;
    fn close(&self);
    fn subscribe_ticker(&self, symbols:Vec<String>, tick_types:Option<Vec<String>>);
    fn subscribe_transaction(&self, symbols:Vec<String>);
    fn subscribe_orderbook(&self, symbols:Vec<String>);
}

const URL:&'static str = "wss://pubwss.bithumb.com/pub/ws";

pub struct Connector{
    listeners:Vec<Box<dyn Listener>>,
}


pub trait Listener{
    fn on_opened(&self);
    fn on_error(&self);
    fn on_close(&self);
    fn on_message(&self, msg:ws::Message);
}

struct A{

}

impl Listener for A{
    fn on_opened(&self){}
    fn on_error(&self){}
    fn on_close(&self){}
    fn on_message(&self, msg:ws::Message){}
}


// 1. Outer object
// 2. Inside object
// 3. 

impl Connector{
    pub fn new()->Self{ Connector{ listeners:Vec::new() } }

    pub fn add_handler(mut self, listener:Box<dyn Listener>)-> Self{ self.listeners.push(listener); self }

    pub fn connect_and_run(&mut self){

        // TODO initialize the inner object which implemted WSClient trait

        //ws::connect(URL, |out| {
            //// 1. out with mutex 
            //let sender = Arc::new(Mutex::new(out));
            //// 2. 



        //}).unwrap();

        // TODO return this inner object, this will be trait

    }
}

pub fn connect(listeners:Vec<Box<dyn Listener>>)-> Result<(), anyhow::Error>{
    let mut conn = Connector::new();
    for l in listeners.into_iter(){
        conn = conn.add_handler(l);
    }
    conn.connect_and_run();
    Ok(())
}


fn main(){
    let _ = dotenv::dotenv();
    env_logger::init();

    let listener = Box::from(A{});
    let _ = Connector::new().add_handler(listener).connect_and_run();
}
