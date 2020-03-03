#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
#[macro_use] extern crate serde_json;


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

    // manual ***** removed pub
    // let res = client.request(reqwest::Method::POST, "/info/balance", Some(json!({"currency":"XMR"})));
    // if res.is_ok(){
    //     let response = res.unwrap();
    //     info!("account info : {:?}", response.data);
    // }else{
    //     error!("{:?}", res);
    // }

    //::::::::::::PUBLIC
    // ticker
    // let res = bithapi::rest::Client::ticker("XMR");
    // info!("{:?}", res.unwrap());

    // ticker all
    // let res = bithapi::rest::Client::tickers();
    // info!("{:?}", res.unwrap());

    {/// ticker loop
        for i in 0..5{
                println!("start utc {}",chrono::Utc::now().timestamp_millis());
            let res = bithapi::rest::Client::ticker("BTC");
            if let Ok(tick) = res{
                println!("{:?}",tick.date);
                println!("end utc {}\n",chrono::Utc::now().timestamp_millis());
            }
        }
    }

    //::::::::::::PRIVATE

    /////////////INFO
    // account info
    // let res = client.account(rest::AccountParam{order_currency:String::from("XMR"), ..Default::default()});
    // info!("{:?}", res);

    // orders
    // let res = client.orders(rest::OrdersParam{order_currency:String::from("XMR"), ..Default::default()});
    // info!("{:?}", res);

    
    //////////////TRADE
    // place
    // let order_type = rest::OrderType::Bid;
    // let res = client.trade_place(rest::PlaceParam{
    //     order_currency:String::from("XMR"),
    //     payment_currency:String::from("KRW"),
    //     units:0.008,
    //     price:91000.0,
    //     order_type:order_type,
    // });
    // if let Ok(id) = res{ // cancel
    //     info!("ordered : {}", id);
    //     let res = client.trade_cancel(rest::CancelParam{
    //         order_currency:String::from("XMR"),
    //         payment_currency:String::from("KRW"),
    //         order_type:order_type,
    //         order_id:id,
    //     });
    //     info!("{:?}", res);
    // }
}
