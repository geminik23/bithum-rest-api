
use super::model::*;
use std::sync::{Arc, Mutex};

const URL:&'static str = "wss://pubwss.bithumb.com/pub/ws";


impl Connector{
    pub fn connect_and_run<T:Listener+Copy>(listener:T){
        debug!("connecting...");

        ws::connect(URL, move |out| {
            // sender
            let sender = Arc::new(Mutex::new(out));

            // initilaize the bithumb handler
            let bhandler = BithumbHandler{
                out : sender.clone(),
            };

            // return handler
            InnerHandler{
                listener:listener,
                out: sender.clone(),
                bith_handler:bhandler,
            }

        }).unwrap();
    }
}


