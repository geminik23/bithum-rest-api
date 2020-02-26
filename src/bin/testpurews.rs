
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
#[macro_use] extern crate serde_json;
extern crate ws;


fn main(){
    ws::connect("wss://pubwss.bithumb.com/pub/ws", |out|{
        move |msg|{
            println!("{:?}", msg);
            Ok(())
        }
    }).unwrap();

    

}
