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
use std::process::{Command, Stdio, Child};

// reserved pot 
// 39991
// 39010 39011


// TODO
// 1. Worker -> seperate Model
// 2. adding ops -> WorkerCount
// 3. response
// 4. matching msg
// 5. 



fn main(){
    let _ = dotenv::dotenv();
    env_logger::init();

    info!("starting bithserver..");
    // TODO
    // zmq::REP initialize
    let ctx = zmq::Context::new();
    let rep = ctx.socket(zmq::REP).unwrap();
    let ep = "tcp://*:39991";
    rep.bind(ep).expect("Failed to binding endpoint");

    // broker
    let wport = 39011;
    let wep_server = format!("tcp//*:{}", wport);
    let wep_client = format!("tcp://127.0.0.1:{}", wport);
    let mut bw = BrokerOfWorkers::new("tcp://*:39010",&wep_server);
    let w = std::thread::spawn(move||{
        bw.run();
    });

    
    // workers
    
    let mut children:Vec<Child> = Vec::new();

    // loop for REP
    loop{
        let mut items = [
            rep.as_poll_item(zmq::POLLIN),
        ];
        zmq::poll(&mut items, -1).unwrap();

        if items[0].is_readable() {
            if let Ok(message) = rep.recv_msg(0){
                // rep
                // TODO check ops
                //
                //
                //
                let count = 1;
                { // set new count
                    for child in children.iter_mut(){
                        child.kill().expect("!kill");
                    }
                    std::thread::sleep(std::time::Duration::from_secs_f64(1.5));
                    children.clear();
                    for i in 0..count{
                        let child = Command::new("./bithworker")
                            .arg(&wep_client)
                            .spawn()
                            .expect("Failed to execute worker");
                        children.push(child);
                    }

                }
                
            }else{
                // rep error
            }
        }
    }
    
}




#[derive(Default)]
pub struct BrokerOfWorkers{
    front_ep:String,
    back_ep:String,
}

impl BrokerOfWorkers{
    pub fn new(front_ep:&str, back_ep:&str)->Self{
        BrokerOfWorkers{
            front_ep:String::from(front_ep),
            back_ep:String::from(back_ep),
        }
    }

    pub fn run(&mut self){
        info!("initialize broker of workers...");
        info!("frontend : {}, backend : {}", self.front_ep, self.back_ep);
        let context = zmq::Context::new();

        let frontend = context.socket(zmq::ROUTER).unwrap();
        let backend = context.socket(zmq::DEALER).unwrap();

        frontend
            .bind(self.front_ep.as_str())
            .expect("[BROKER] failed binding frontend");
        backend
            .bind(self.back_ep.as_str())
            .expect("[BROKER] failed binding backend");

        info!("running broker of workers..");

        loop{
            let mut items = [
                frontend.as_poll_item(zmq::POLLIN),
                backend.as_poll_item(zmq::POLLIN),
            ];
            zmq::poll(&mut items, -1).unwrap();

            if items[0].is_readable() {
                loop {
                    let message = frontend.recv_msg(0).unwrap();
                    let more = message.get_more();
                    info!("frontend : received");
                    backend
                        .send(message, if more { zmq::SNDMORE } else { 0 })
                        .unwrap();
                    if !more {
                        break;
                    }
                }
            }
            if items[1].is_readable() {
                loop {
                    let message = backend.recv_msg(0).unwrap();
                    let more = message.get_more();
                    info!("backend : received");
                    frontend
                        .send(message, if more { zmq::SNDMORE } else { 0 })
                        .unwrap();
                    if !more {
                        break;
                    }
                }
            }   
        }
    }
}
