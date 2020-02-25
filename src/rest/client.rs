

use super::model::*;
use crate::authenticate::Authenticate;

const URL:&'static str = "https://api.bithumb.com";




impl Client{
    pub fn new(api:&str, secret:&str)->Self{
        Client{
            url:reqwest::Url::parse(URL).unwrap(),
            auth:Authenticate::from(api, secret)
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
    pub fn request(&self, method:reqwest::Method, endpoint:&str)-> Result<BithResponse, RestError>{
        let ourl = self.url.clone().join(endpoint).expect("failed to join endpoint");

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

}

