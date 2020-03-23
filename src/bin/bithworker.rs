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
use bithapi::worker::*;

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
                                for i in 0..2{
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
                                        break;
                                    }else{
                                        //response.message = Some(format!("{}, {}", param.units, param.price));
                                        response.message = Some(String::from("Order Failed"));
                                    }
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
