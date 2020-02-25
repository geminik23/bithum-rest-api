
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate reqwest;
#[macro_use]
extern crate serde_json;
extern crate anyhow;
//https://apidocs.bithumb.com/docs/ticker


use bithapi::*;



fn main(){
    let _ = dotenv::dotenv();
    env_logger::init();
    

    // let res = order_client.request(reqwest::Method::GET, "public/ticker/ALL_KRW");
    let res = bithapi::rest::Client::request_pub(reqwest::Method::GET, "public/ticker/ALL_KRW");
    if res.is_ok(){
        info!("PUBLIC API the status code of response : {:?}", res.unwrap().status);
    }

    
    let client = bithapi::rest::Client::new( std::env::var("BITHUMB_API").unwrap().as_str(), std::env::var("BITHUMB_SECRET").unwrap().as_str());


    // info!("{:?}",chrono::Utc::now().timestamp_millis());

    let res = client.request(reqwest::Method::POST, "/info/balance", Some(json!({"currency":"XMR"})));
    if res.is_ok(){
        let response = res.unwrap();
        info!("account info : {:?}", response.data);
    }else{
        error!("{:?}", res);
    }

    // threadpool
    

    

}
