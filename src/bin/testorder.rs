
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate reqwest;
extern crate serde_json;
extern crate anyhow;
//https://apidocs.bithumb.com/docs/ticker

use serde::{
    Serialize,
    Deserialize
};

const URL:&'static str = "https://api.bithumb.com";


#[derive(Clone, Deserialize, Debug)]
pub  struct BithResponse{
    pub status:String,
    pub data:serde_json::Value,
}

pub struct BithumbOrder{
    pub url:reqwest::Url,
    pub auth:Authenticate,
}

pub struct Authenticate{
    pub api:String,
    pub secret:String,
}
#[derive(Debug, Clone)]
pub enum BithumbError{
    
}

#[derive(Debug, Clone)]
pub enum RestError {
    ConnectionError,
    JsonParseError,
    ModelParseError,
    Error(u16),
    Unknown(u16),
    ParameterError(u16), //400
    Unauthorized(u16), //401
    AccessDenied(u16), //403
    NotFound(u16), //404 .. if not found order when deleting
}



impl BithumbOrder{
    pub fn new(api:&str, secret:&str)->Self{
        BithumbOrder{
            url:reqwest::Url::parse(URL).unwrap(),
            auth:Authenticate{
                api:String::from(api),
                secret:String::from(secret),
            }
        }
    }
    pub fn request_pub(method:reqwest::Method, endpoint:&str)-> Result<BithResponse, RestError>{
        let ourl = reqwest::Url::parse(URL).unwrap().join(endpoint).expect("failed to join endpoint");

        let client  = reqwest::blocking::Client::new();

        if let Ok(response) = client.request(method, ourl)
        // .body()
        .send(){
            let code = response.status().as_u16();
            match code{
                code if code >=400 =>{
                    match code{
                        400 => { return Err(RestError::ParameterError(code));},
                        401 => { return Err(RestError::Unauthorized(code));},
                        403 => { return Err(RestError::AccessDenied(code));},
                        404 => { return Err(RestError::NotFound(code));},
                        _=>{ return Err(RestError::Error(code));},
                    }
                },
                200=>{
                    if let Ok(res) = response.json::<serde_json::Value>(){
                        //TODO
                        let result = serde_json::from_value::<BithResponse>(res);
                        if result.is_ok(){
                            return Ok(result.unwrap());
                        }
                        return Err(RestError::ModelParseError);
                    }else{
                        // Json Parse Error
                        return Err(RestError::JsonParseError);
                    }
                },
                _=>{
                    return Err(RestError::Unknown(code));
                }
            }

        }
        // Connection Error
        Err(RestError::ConnectionError)
    }

    pub fn request(&self, method:reqwest::Method, endpoint:&str)-> Result<serde_json::Value, anyhow::Error>{
        let ourl = self.url.clone().join(endpoint).expect("failed to join endpoint");

        let client  = reqwest::blocking::Client::new();

        if let Ok(response) = client.request(method, ourl)
        // .body()
        .send(){
            let code = response.status().as_u16();
            match code{
                code if code >=400 =>{
                    // 400=>{ return Err(RestError::ParameterError(err));},
                    // 401=>{ return Err(RestError::Unauthorized(err));},
                    // 403=>{ return Err(RestError::AccessDenied(err));},
                    // 404=>{ return Err(RestError::NotFound(err));},
                    // 429=>{ return Err(RestError::RateLimitWithResetTime(err, resettime)); }, 
                    // 503=>{ return Err(RestError::Downtime(err)); },
                    // _=>{ return Err(RestError::Error(code));},
                },
                200=>{
                    // TODO
                    if let Ok(res) = response.json::<serde_json::Value>(){
                        return Ok(res)
                    }
                    // Json Parse Error
                    // return Err(RestError::JsonParseError);
                },
                _=>{
                    // return Err(RestError::Unknown(code));
                }
            }
            

        }
        // Connection Error
        // Err(RestError::ConnectionError)
        Ok(serde_json::Value::Null)
    }

    pub fn test_order(&self){

    }

}



fn main(){
    let _ = dotenv::dotenv();
    env_logger::init();
    

    // let res = order_client.request(reqwest::Method::GET, "public/ticker/ALL_KRW");
    let res = BithumbOrder::request_pub(reqwest::Method::GET, "public/ticker/ALL_KRW");
    if res.is_ok(){
        info!("PUBLIC API the status code of response : {:?}", res.unwrap().status);
    }

    
    let client = BithumbOrder::new( std::env::var("BITHUMB_API").unwrap().as_str(), std::env::var("BITHUMB_SECRET").unwrap().as_str());
    //client.request(

    // threadpool
    



    

}
