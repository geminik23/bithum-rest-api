

#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate zmq;

use bithapi::*;



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


use std::process::{Command, Stdio, Child};
use std::i32;

use std::io::{self, BufRead};
fn main(){
    let _ = dotenv::dotenv();
    env_logger::init();

    let ctx = zmq::Context::new();
    let router = ctx.socket(zmq::ROUTER).unwrap();
    let wport = 39011;
    let wep = "tcp://*:39011";

    let mut children:Vec<Child> = Vec::new();
    let mut bw = BrokerOfWorkers::new("tcp://*:39010",wep);
    let w = std::thread::spawn(move||{
        bw.run();
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let count:i32 = line.unwrap().parse().unwrap();

        for child in children.iter_mut(){
            child.kill().expect("!kill");
        }
        std::thread::sleep(std::time::Duration::from_secs_f64(2.0));

        children.clear();

        for i in 0..count{
            let child = Command::new("./bithworker").arg(format!("tcp://127.0.0.1:{}", wport).as_str()).spawn().expect("Failed to execute worker");
            children.push(child);
        }
    }

}
