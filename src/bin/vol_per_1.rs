#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate anyhow;
extern crate bithapi;
use bithapi::ws::*;
extern crate chrono;
extern crate zmq;

use chrono::Timelike;
use serde::Serialize;


#[derive(Clone, Debug, Serialize, Copy)]
struct Param{
    pub volumes:f64,
    pub hour:u32,
    pub min:u32,
    pub sec:u32,
    pub nano:u32,
}



#[derive(Clone, Copy)]
struct VolCalculator{ 
    info:Param,
}

impl VolCalculator{
    pub fn new()->Self{
        VolCalculator{
            info:Param{
                volumes:0.0,
                hour:0u32,
                min:0u32,
                sec:0u32,
                nano:0u32,
            }
        }
    }
}

impl bithapi::ws::Listener for VolCalculator{

    fn on_opened(&mut self, bith:&mut BithumbHandler) {
        debug!("opened");

        let socket = bith.ctx.socket(zmq::PUB).unwrap();
        socket.bind("tcp://*:54321").unwrap();
        bith.sockets.insert(String::from("publisher"), socket);

        // register
        //bith.subscribe_ticker(vec![String::from("BTC_KRW")], None);
        //bith.subscribe_transaction(vec![String::from("QBZ_KRW")]);
        // all symbols
        let mut arg:Vec<String> = Vec::new();
        if let Ok(tickers) = bithapi::rest::Client::tickers(){
            for key in tickers.tickers.keys(){
                if key =="date"{
                    continue;
                }
                arg.push(format!("{}_KRW", key));
            }
        }
        bith.subscribe_transaction(arg);
    }

    fn on_error(&mut self, err:&ws::Error){
        info!("error {:?}", err);
    }

    fn on_close(&mut self, bith:&mut BithumbHandler){
        debug!("closed");
    }

    fn on_request_resut(&mut self, bith:&mut BithumbHandler, res:String){
        info!("{:?}", res);
    }
    fn on_ticker(&mut self, bith:&mut BithumbHandler, res:TickRes){
        info!("{:?}", res);
    }
    fn on_transaction(&mut self, bith:&mut BithumbHandler, res:TransactionRes){
        // cumulate the all volumes
        for l in res.list.iter(){
            if let Ok(amount)= res.list[0].cont_amt.parse::<f64>(){
                self.info.volumes += amount;
            }
            let dt = chrono::NaiveDateTime::parse_from_str(l.cont_dtm.as_str(), "%Y-%m-%d %H:%M:%S.%6f").unwrap();
            let mut cur = dt.hour();
            if cur == 0{ cur = 24;}

            // comparing the time
            if self.info.hour < cur{
                self.info.volumes = 0.0;
                self.info.hour = if cur==24 {0} else {cur};
            }
        
            if self.info.hour == dt.hour(){
                self.info.hour = dt.hour();
                self.info.min = dt.minute();
                self.info.sec = dt.second();
                self.info.nano = dt.nanosecond();
                info!("volume : {}, Time : {}:{}:{}.{}", self.info.volumes,self.info.hour, self.info.min, self.info.sec, self.info.nano);
                if let Some(socket) = bith.sockets.get("publisher"){
                    if let Ok(msg) = serde_json::to_string(&self.info){
                        socket.send(&msg, 0).unwrap();
                    }else{
                        error!("Serialize Error");
                    }
                }else{
                    error!("no sockets");
                }
            }
        }
    }

    fn on_orderbook(&mut self, bith:&mut BithumbHandler, res:OrderbookdepthRes){
        info!("{:?}", res);
    }
}



fn main(){
    let _ = dotenv::dotenv();
    env_logger::init();


    let vol = VolCalculator::new();
    let _ = bithapi::ws::Connector::connect_and_run(vol);
}
