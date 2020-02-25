

use super::model::*;
use crate::authenticate::Authenticate;

const URL:&'static str = "https://api.bithumb.com";

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, from_slice, to_string, to_value, to_vec};




impl Client{
    pub fn new(api:&str, secret:&str)->Self{
        Client{
            url:reqwest::Url::parse(URL).unwrap(),
            auth:Authenticate::from(api, secret)
        }
    }

    fn to_params<T:Serialize>(&self, req:T) -> Vec<(String, String)> {
        let v = to_value(req).unwrap();
        let v = v.as_object().unwrap();
        let mut vec = vec![];

        for (key,value) in v.into_iter() {
            if value.is_null() {
                continue;
            } else if value.is_string() {
                vec.push((key.clone(), value.as_str().unwrap().to_string()))
            } else {
                vec.push((key.clone(), to_string(value).unwrap()))
            }
        }
        vec
    }

    pub fn request_pub(method:reqwest::Method, endpoint:&str)-> Result<BithResponse, RestError>{
        let ourl = reqwest::Url::parse(URL).unwrap().join(endpoint).expect("failed to join endpoint");

        let client  = reqwest::blocking::Client::new();

        if let Ok(response) = client.request(method, ourl)
        // .body()
        .send(){
            let code = response.status().as_u16();
            let mut body :Option<BithResponse> = None;
            if let Ok(res) = response.json::<serde_json::Value>(){
                let result = serde_json::from_value::<BithResponse>(res);
                if result.is_ok(){
                    body = Some(result.unwrap());
                }else{
                    return Err(RestError::ModelParseError);
                }
            }else{
                return Err(RestError::JsonParseError);
            }
            match code{
                code if code >=400 =>{
                    match code{
                        400 => { return Err(RestError::ParameterError(code, body));},
                        401 => { return Err(RestError::Unauthorized(code, body));},
                        403 => { return Err(RestError::AccessDenied(code, body));},
                        404 => { return Err(RestError::NotFound(code, body));},
                        _=>{ return Err(RestError::Error(code, body));},
                    }
                },
                200=>{
                    if let Some(res) = body{
                        return Ok(res);
                    }
                },
                _=>{
                    return Err(RestError::Unknown(code, body));
                }
            }

        }
        // Connection Error
        Err(RestError::ConnectionError)
    }


    pub fn post_typed<T>(&self, endpoint:&str, params:Option<serde_json::Value>)-> RestTypedResult<T> where T:DeserializeOwned {
        return self.request_typed::<T>(reqwest::Method::POST, endpoint, params);
    }

    pub fn request_typed<T>(&self, method:reqwest::Method, endpoint:&str, params:Option<serde_json::Value>)-> RestTypedResult<T> where T:DeserializeOwned {
        let result = self.request(method, endpoint, params);
        match result {
            Ok(res)=>{
                if let Some(data) = res.data{
                    if let Ok(result_) = serde_json::from_value::<T>(data){
                        return Ok(result_);
                    }
                }
                error!("no data in success response");
                return Err(RestError::JsonParseError);
            },
            Err(why)=>{ return Err(why)},
        }
    }



    pub fn request(&self, method:reqwest::Method, endpoint:&str, params:Option<serde_json::Value>)-> Result<BithResponse, RestError> {
        let mut ourl = self.url.clone().join(endpoint).expect("failed to join endpoint");
        let _url = ourl.clone();

        if let Some(mut param) = params {
            param["endpoint"] = serde_json::to_value(endpoint).unwrap();
            let p = self.to_params(param);
            let mut query = ourl.query_pairs_mut();
            for (key,value) in p.iter() {
                query.append_pair(&key, &value);
            }
        }

        let client  = reqwest::blocking::Client::new();
        // self.auth.signature();

        let nonce = chrono::Utc::now().timestamp_millis();
        // let nonce = chrono::Local::now().timestamp_millis();
        let mut query:String = String::from("");
        if let Some(q) = ourl.query(){
            query.push_str(q);
        }

        let sign = self.auth.signature(endpoint, nonce, &query).unwrap();
        info!("signature {:?}", sign);



        if let Ok(response) = client.request(method, _url)
        .body(reqwest::blocking::Body::from(query))
        .header("Api-Key", self.auth.api.as_str())
        .header("Api-Sign", sign.as_str())
        .header("Api-Nonce", format!("{}",nonce).as_str())
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        // .header("user-agent", "bithapi-rs")
        .send(){
            let code = response.status().as_u16();
            let mut body :Option<BithResponse> = None;
            if let Ok(res) = response.json::<serde_json::Value>(){
                let result = serde_json::from_value::<BithResponse>(res);
                if result.is_ok(){
                    body = Some(result.unwrap());
                }else{
                    return Err(RestError::ModelParseError);
                }
            }else{
                return Err(RestError::JsonParseError);
            }
            match code{
                code if code >=400 =>{
                    match code{
                        400 => { return Err(RestError::ParameterError(code, body));},
                        401 => { return Err(RestError::Unauthorized(code, body));},
                        403 => { return Err(RestError::AccessDenied(code, body));},
                        404 => { return Err(RestError::NotFound(code, body));},
                        _=>{ return Err(RestError::Error(code, body));},
                    }
                },
                200=>{
                    if let Some(res) = body{
                        return Ok(res);
                    }
                },
                _=>{
                    return Err(RestError::Unknown(code, body));
                }
            } 

        }
        // Connection Error
        Err(RestError::ConnectionError)
    }


    /// PRIVATE APIs
    /// 
    pub fn account(&self, param:AccountParam)->RestTypedResult<AccountResponse>{
        self.post_typed("/info/account", Some(serde_json::to_value(param).unwrap()))
    }
    


}

