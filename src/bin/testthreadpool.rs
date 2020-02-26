
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate threadpool;

use threadpool::ThreadPool;
use bithapi::*;

fn main(){
    let _ = dotenv::dotenv();
    env_logger::init();

    let client = bithapi::rest::Client::new( std::env::var("BITHUMB_API").unwrap().as_str(), std::env::var("BITHUMB_SECRET").unwrap().as_str());

    let n_workers = 25;
    let n_jobs = 30;
    let price = 62.8;
    let pool = ThreadPool::new(n_workers);


    //==========command line
    let mut s=String::new();
    print!("Please enter some text: ");
    std::io::stdin().read_line(&mut s).expect("Did not enter a correct string");


    println!("{:?}", chrono::Utc::now());
    for _ in 0..n_jobs{
        let c = client.clone();
        let side = rest::OrderType::Ask;
        pool.execute(move ||{
            let res = c.trade_place(rest::PlaceParam{
                order_currency:String::from("XEM"),
                payment_currency:String::from("KRW"),
                units:9.0,
                price:price,
                order_type:side,
            });
            if res.is_ok(){
                info!("success! {}", res.unwrap());
            }
        });

        let c = client.clone();
        let side = rest::OrderType::Bid;
        pool.execute(move ||{
            let res = c.trade_place(rest::PlaceParam{
                order_currency:String::from("XEM"),
                payment_currency:String::from("KRW"),
                units:9.0,
                price:price,
                order_type:side,
            });
            if res.is_ok(){
                info!("success! {}", res.unwrap());
            }
        });
    }

    std::thread::sleep(std::time::Duration::from_secs(10));
    info!("end");
}