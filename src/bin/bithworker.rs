#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate anyhow;
extern crate bithapi;
extern crate chrono;
extern crate zmq;

use chrono::Timelike;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/*
 * # Todo
 *
 * 1. arguments from command line.
 * 2. connect zmq
 * 3. receive the message
 * 4. parse the request order
 * 5. request order api 
 * 6. return uid
 *
 * 7. testing
 *
 * */

#[derive(Clone, Debug, Deserialize)]
pub struct Param{
    pub api:String,
    pub secret:String,
    pub order_symbol:String,
    pub payment_symbol:String,
    pub units:f64,
    pub price:f64,
    pub order_type:bithapi::rest::OrderType,
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


fn main(){

    let _ = dotenv::dotenv();
    env_logger::init();

    let args:Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("endpoint is required as argument.");
    }

    let endpoint = &args[1];
    info!("{:?}", endpoint);

    let ctx = zmq::Context::new();
    let rep = ctx.socket(zmq::REP).unwrap();
    rep.connect(endpoint).unwrap();

    info!("Starting bithumb worker");

    loop{
        let mut msg = zmq::Message::new();
        rep.recv(&mut msg, 0).unwrap();
        if let Some(msgstr) = msg.as_str(){
            let mut response = WorkerResponse{
                success:false,
                uid:String::new(),
                message:None,
                arg:None,
            };

            if let Ok(req) = serde_json::from_str::<WorkerRequest>(msgstr){
                response.uid = req.uid.clone();
                
                match req.op {
                    RequestOp::Ping => {
                        response.success = true;
                        response.message = Some(String::from("Pong"));
                    },
                    RequestOp::OrderRequest =>{
                        // parse Arg
                        if let Some(reqarg) = req.arg{
                            if let Ok(param) = serde_json::from_value::<Param>(reqarg){
                                // response...
                                let client = bithapi::rest::Client::new(&param.api, &param.secret);
                                let orderres = client.trade_place(bithapi::rest::PlaceParam{
                                    order_currency:param.order_symbol.clone(),
                                    payment_currency:param.payment_symbol.clone(),
                                    units:param.units,
                                    price:param.price,
                                    order_type:param.order_type,
                                });

                                if orderres.is_ok(){
                                    response.success = true;
                                    let orderid = orderres.unwrap();
                                    response.arg = Some(orderid);
                                }else{
                                    response.message = Some(String::from("Order Failed"));
                                }
                            }else{
                                response.message = Some(String::from("Invalid parameter"));
                            }
                        }else{
                            response.message = Some(String::from("Invalid parameter"));
                        }
                    },
                    _=>{},
                }
                
            }else {
                response.message = Some(String::from("Invalid Structure."));
                error!("failed to parse a request : {}", msgstr);
            }

            // send reply
            if let Ok(repstr) = serde_json::to_string(&response){
                rep.send_str(&repstr, 0).unwrap();
            }
        }
    }
}
