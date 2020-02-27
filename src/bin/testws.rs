//#[macro_use] extern crate log;
//extern crate env_logger;
//extern crate dotenv;
//#[macro_use] extern crate serde_json;
//extern crate anyhow;


//extern crate ws;

//extern crate bithapi;
//use bithapi::ws::*;



//#[derive(Clone, Copy)]
//struct A{

//}

//extern crate chrono;

//impl bithapi::ws::Listener for A{

    //fn on_opened(&mut self, bith:&BithumbHandler) {
        //debug!("opened");
        //// register
        ////bith.subscribe_ticker(vec![String::from("BTC_KRW")], None);
        ////bith.subscribe_orderbook(vec![String::from("BTC_KRW"), String::from("ETH_KRW")]);
        //bith.subscribe_transaction(vec![String::from("BTC_KRW"), String::from("ETH_KRW")]);
    //}

    //fn on_error(&mut self, err:&ws::Error){
        //info!("error {:?}", err);
    //}

    //fn on_close(&mut self, bith:&BithumbHandler){
        //debug!("closed");
    //}

    //fn on_request_resut(&mut self, bith:&BithumbHandler, res:String){
        //info!("{:?}", res);
    //}
    //fn on_ticker(&mut self, bith:&BithumbHandler, res:TickRes){
        //info!("{:?}", res);
    //}
    //fn on_transaction(&mut self, bith:&BithumbHandler, res:TransactionRes){
        //println!("{}", res.list[0].cont_dtm);
        //let no_timezone = chrono::NaiveDateTime::parse_from_str(res.list[0].cont_dtm.as_str(), "%Y-%m-%d %H:%M:%S.%6f").unwrap();
        //println!("{}", no_timezone);
        ////info!("{:?}", res);
    //}
    //fn on_orderbook(&mut self, bith:&BithumbHandler, res:OrderbookdepthRes){
        //info!("{:?}", res);
    //}
//}


//fn main(){
    //let _ = dotenv::dotenv();
    //env_logger::init();

    //let listener = A{};
    //let _ = bithapi::ws::Connector::connect_and_run(listener);
//}
