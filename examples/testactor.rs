
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate actix;

use bithapi::*;

use actix::{
    Actor,
    Addr,
    System,
    Context,
    Handler,
    Message,
};


#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Param{
    pub api:String,
    pub secret:String,
    pub order_symbol:String,
    pub payment_symbol:String,
    pub units:f64,
    pub price:f64,
    pub order_type:bithapi::rest::OrderType,
}


struct BithReq;

impl Actor for BithReq{
    type Context = Context<Self>;

    fn started(&mut self, ctx:&mut Self::Context){
        info!("started actor..");
    }
}

impl Handler<Param> for BithReq{
    type Result = (); // <- Message response type

    fn handle(&mut self, param: Param, ctx: &mut Context<Self>) -> Self::Result {
        let client = bithapi::rest::Client::new(&param.api, &param.secret);
        let orderres = client.trade_place(bithapi::rest::PlaceParam{
            order_currency:param.order_symbol.clone(),
            payment_currency:param.payment_symbol.clone(),
            units:param.units,
            price:param.price,
            order_type:param.order_type,
        });

        if orderres.is_ok(){
            let orderid = orderres.unwrap();
            info!("order success : {:?}",orderid); 
        }else{
            error!("order failed");
        }
    }
}
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main(){
    let _ = dotenv::dotenv();
    env_logger::init();
    let api= String::from(std::env::var("BITHUMB_API").unwrap().as_str());
    let secret=  String::from(std::env::var("BITHUMB_SECRET").unwrap().as_str());

    let system = System::new("test");

    let mut actors:Vec<Addr<BithReq>> = Vec::new();

    for i in 0..30{
        let mut addr = BithReq.start();
        actors.push(addr);
    }
    let mut req = Param{
        api:api,
        secret:secret,
        order_symbol:String::from("XVG"),
        payment_symbol:String::from("KRW"),
        units:220.0,
        price:2.392,
        order_type:bithapi::rest::OrderType::Bid,
    };

    let mut req2 = req.clone();
    req2.order_type = bithapi::rest::OrderType::Ask;

    std::thread::sleep(std::time::Duration::from_secs(2));
    for i in 0..14{
        actors[i].do_send(req.clone());
        actors[i+1].do_send(req2.clone());
    }

    system.run();

}
