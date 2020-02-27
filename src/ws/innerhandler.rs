
use std::str::from_utf8;
use ws::{CloseCode, OpCode, Sender, Frame, Handler, Handshake, Message, Result, Error, ErrorKind};
use super::model::*;
use ws::util::{Token, Timeout};

//impl<T:Listener> BithumbHandler for InnerHandler<T>{
    //fn subscribe_ticker(&mut self, symbols:Vec<String>, tick_types:Option<Vec<String>>){
    //}
    //fn subscribe_transaction(&mut self, symbols:Vec<String>){
    //}
    //fn subscribe_orderbook(&mut self, symbols:Vec<String>){
    //}
//}

impl<T:Listener+Copy> Handler for InnerHandler<T>{

    fn on_open(&mut self, hs: Handshake) -> Result<()> {
        // pass msg to listeners
        debug!("on open method in InnerHandler");
        self.listener.on_opened(&self.bith_handler);
        DefaultHandler.on_open(hs)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        debug!("on open method in InnerHandler : {:?}", msg);
        // pass msg to listeners
        
        if let Ok(strmsg) = msg.as_text(){
            if let Ok(res_) = serde_json::from_str::<WSResponse>(strmsg){
                match res_.filter_type{
                    FilterType::Ticker=>{
                        if let Ok(res) = serde_json::from_value::<TickRes>(res_.content){
                            self.listener.on_ticker(&self.bith_handler, res);
                        }else{
                            debug!("failed to parse tick");
                        }
                    },
                    FilterType::Transaction=>{
                        if let Ok(res) = serde_json::from_value::<TransactionRes>(res_.content){
                            self.listener.on_transaction(&self.bith_handler, res);
                        }else{
                            debug!("failed to parse transaction");
                        }
                    },
                    FilterType::Orderbookdepth=>{
                        if let Ok(res) = serde_json::from_value::<OrderbookdepthRes>(res_.content){
                            self.listener.on_orderbook(&self.bith_handler, res);
                        }else{
                            debug!("failed to parse orderbook");
                        }
                    },
                }
                return Ok(());
            }else {
                self.listener.on_request_resut(&self.bith_handler, String::from(strmsg));
            }
        }
        DefaultHandler.on_message(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // pass msg to listeners
        self.listener.on_close(&self.bith_handler);
        
        let sender = self.out.lock().unwrap();
        debug!("closed {:?} reason: {}", code, reason);

        match code {
            CloseCode::Normal => {}
            CloseCode::Away => {}
            _ => {}
        }
        sender.shutdown().unwrap();
    }

    fn on_error(&mut self, err: Error) {
        // pass msg to listeners
        self.listener.on_error(&err);
        
        {// if error, shutdown
            let sender = self.out.lock().unwrap();
            sender.shutdown().unwrap();
        }
    }

        
    fn on_frame(&mut self, frame: Frame) -> Result<Option<Frame>> {
        if frame.opcode() == OpCode::Pong {
            //if let Ok(pong) = (from_utf8(frame.payload())?).parse::<u64>() {
            if let Ok(_) = (from_utf8(frame.payload())?).parse::<u64>() {
                debug!("received pong");
            } else {
                error!("received bad pong");
            }
        }
        DefaultHandler.on_frame(frame)
    }
    


}

struct DefaultHandler;
impl Handler for DefaultHandler {}
