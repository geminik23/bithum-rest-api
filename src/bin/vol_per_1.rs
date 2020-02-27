#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate anyhow;
extern crate bithapi;
use bithapi::ws::*;
extern crate chrono;
extern crate zmq;



#[derive(Clone, Copy)]
struct VolCalculator{

}

impl VolCalculator{
    pub fn new()->Self{
        // setup for zmq

        const ctx:zmq::Context = zmq::Context::new();
        let socket = ctx.socket(zmq::PUB).unwrap();



        VolCalculator{
            //ctx:ctx.clone(),
            //socket:Box::from(socket),
        }
    }
}

impl bithapi::ws::Listener for VolCalculator{

    fn on_opened(&mut self, bith:&BithumbHandler) {
        debug!("opened");
        // register
        //bith.subscribe_ticker(vec![String::from("BTC_KRW")], None);
        //bith.subscribe_orderbook(vec![String::from("BTC_KRW"), String::from("ETH_KRW")]);
        bith.subscribe_transaction(vec![String::from("BTC_KRW"), String::from("ETH_KRW")]);
    }

    fn on_error(&mut self, err:&ws::Error){
        info!("error {:?}", err);
    }

    fn on_close(&mut self, bith:&BithumbHandler){
        debug!("closed");
    }

    fn on_request_resut(&mut self, bith:&BithumbHandler, res:String){
        info!("{:?}", res);
    }
    fn on_ticker(&mut self, bith:&BithumbHandler, res:TickRes){
        info!("{:?}", res);
    }
    fn on_transaction(&mut self, bith:&BithumbHandler, res:TransactionRes){
        println!("{}", res.list[0].cont_dtm);
        let no_timezone = chrono::NaiveDateTime::parse_from_str(res.list[0].cont_dtm.as_str(), "%Y-%m-%d %H:%M:%S.%6f").unwrap();
        println!("{}", no_timezone);
        //info!("{:?}", res);
    }
    fn on_orderbook(&mut self, bith:&BithumbHandler, res:OrderbookdepthRes){
        info!("{:?}", res);
    }
}


fn main(){
    let _ = dotenv::dotenv();
    env_logger::init();

    let _ = bithapi::ws::Connector::connect_and_run(VolCalculator::new());
}
