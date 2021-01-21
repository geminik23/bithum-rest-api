use super::model::*;
use crate::authenticate::Authenticate;

const URL: &'static str = "https://api.bithumb.com";

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_slice, to_string, to_value, to_vec, Value};

impl Client {
    pub fn new(api: &str, secret: &str) -> Self {
        Client {
            url: reqwest::Url::parse(URL).unwrap(),
            auth: Authenticate::from(api, secret),
        }
    }

    fn to_params<T: Serialize>(&self, req: T) -> Vec<(String, String)> {
        let v = to_value(req).unwrap();
        let v = v.as_object().unwrap();
        let mut vec = vec![];

        for (key, value) in v.into_iter() {
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

    pub fn request_pub(method: reqwest::Method, endpoint: &str) -> Result<BithResponse, RestError> {
        let ourl = reqwest::Url::parse(URL)
            .unwrap()
            .join(endpoint)
            .expect("failed to join endpoint");

        let client = reqwest::Client::new();

        if let Ok(mut response) = client
            .request(method, ourl)
            // .body()
            .send()
        {
            let mut body: Option<BithResponse> = None;
            //if let Ok(res) = response.json::<serde_json::Value>(){
            if let Ok(res) = response.json::<serde_json::Value>() {
                let result = serde_json::from_value::<BithResponse>(res);
                if result.is_ok() {
                    body = Some(result.unwrap());
                } else {
                    return Err(RestError::ModelParseError);
                }
            } else {
                return Err(RestError::JsonParseError);
            }
            let code = response.status().as_u16();
            match code {
                code if code >= 400 => match code {
                    400 => {
                        return Err(RestError::ParameterError(code, body));
                    }
                    401 => {
                        return Err(RestError::Unauthorized(code, body));
                    }
                    403 => {
                        return Err(RestError::AccessDenied(code, body));
                    }
                    404 => {
                        return Err(RestError::NotFound(code, body));
                    }
                    _ => {
                        return Err(RestError::Error(code, body));
                    }
                },
                200 => {
                    let arg = body.clone();
                    if let Some(res) = body {
                        if res.message.is_some() {
                            return Err(RestError::BithumbError(0, arg));
                        }
                        return Ok(res);
                    }
                }
                _ => {
                    return Err(RestError::Unknown(code, body));
                }
            }
        }
        // Connection Error
        Err(RestError::ConnectionError)
    }

    fn post_typed_data<T>(
        &self,
        endpoint: &str,
        params: Option<serde_json::Value>,
    ) -> RestTypedResult<T>
    where
        T: DeserializeOwned,
    {
        return self.request_typed_data::<T>(reqwest::Method::POST, endpoint, params);
    }

    fn post_typed_order_id(
        &self,
        endpoint: &str,
        params: Option<serde_json::Value>,
    ) -> RestTypedResult<String> {
        return self.request_typed_order_id(reqwest::Method::POST, endpoint, params);
    }

    fn post_typed_empty(
        &self,
        endpoint: &str,
        params: Option<serde_json::Value>,
    ) -> RestTypedResult<()> {
        return self.request_typed_empty(reqwest::Method::POST, endpoint, params);
    }

    fn request_typed_data<T>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        params: Option<serde_json::Value>,
    ) -> RestTypedResult<T>
    where
        T: DeserializeOwned,
    {
        let result = self.request(method, endpoint, params);
        match result {
            Ok(res) => {
                if let Some(data) = res.data {
                    if let Ok(result_) = serde_json::from_value::<T>(data) {
                        return Ok(result_);
                    }
                }
                error!("no data in success response");
                return Err(RestError::JsonParseError);
            }
            Err(why) => return Err(why),
        }
    }

    fn request_typed_order_id(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        params: Option<serde_json::Value>,
    ) -> RestTypedResult<String> {
        let result = self.request(method, endpoint, params);
        match result {
            Ok(res) => {
                if let Some(data) = res.order_id {
                    return Ok(data);
                }
                error!("no data in success response");
                return Err(RestError::JsonParseError);
            }
            Err(why) => return Err(why),
        }
    }

    fn request_typed_empty(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        params: Option<serde_json::Value>,
    ) -> RestTypedResult<()> {
        let result = self.request(method, endpoint, params);
        match result {
            Ok(res) => {
                return Ok(());
            }
            Err(why) => return Err(why),
        }
    }

    fn request(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        params: Option<serde_json::Value>,
    ) -> Result<BithResponse, RestError> {
        let mut ourl = self
            .url
            .clone()
            .join(endpoint)
            .expect("failed to join endpoint");
        let _url = ourl.clone();

        if let Some(mut param) = params {
            param["endpoint"] = serde_json::to_value(endpoint).unwrap();
            let p = self.to_params(param);
            let mut query = ourl.query_pairs_mut();
            for (key, value) in p.iter() {
                query.append_pair(&key, &value);
            }
        }

        let client = reqwest::Client::new();
        // self.auth.signature();

        let nonce = chrono::Utc::now().timestamp_millis() + 10 * 1000;
        // let nonce = chrono::Local::now().timestamp_millis();
        let mut query: String = String::from("");
        if let Some(q) = ourl.query() {
            query.push_str(q);
        }

        let sign = self.auth.signature(endpoint, nonce, &query).unwrap();

        if let Ok(mut response) = client
            .request(method, _url)
            .body(reqwest::Body::from(query))
            .header("Api-Key", self.auth.api.as_str())
            .header("Api-Sign", sign.as_str())
            .header("Api-Nonce", format!("{}", nonce).as_str())
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .header("user-agent", "bithapi-rs")
            .send()
        {
            let code = response.status().as_u16();
            let mut body: Option<BithResponse> = None;
            //if let Ok(res) = response.json::<serde_json::Value>(){
            if let Ok(res) = response.json::<serde_json::Value>() {
                let result = serde_json::from_value::<BithResponse>(res);
                if result.is_ok() {
                    body = Some(result.unwrap());
                } else {
                    return Err(RestError::ModelParseError);
                }
            } else {
                return Err(RestError::JsonParseError);
            }
            match code {
                code if code >= 400 => match code {
                    400 => {
                        return Err(RestError::ParameterError(code, body));
                    }
                    401 => {
                        return Err(RestError::Unauthorized(code, body));
                    }
                    403 => {
                        return Err(RestError::AccessDenied(code, body));
                    }
                    404 => {
                        return Err(RestError::NotFound(code, body));
                    }
                    _ => {
                        return Err(RestError::Error(code, body));
                    }
                },
                200 => {
                    let arg = body.clone();
                    if let Some(res) = body {
                        if res.message.is_some() {
                            return Err(RestError::BithumbError(0, arg));
                        }
                        return Ok(res);
                    }
                }
                _ => {
                    return Err(RestError::Unknown(code, body));
                }
            }
        }
        // Connection Error
        Err(RestError::ConnectionError)
    }

    fn result_request_pub<T>(result: Result<BithResponse, RestError>) -> RestTypedResult<T>
    where
        T: DeserializeOwned,
    {
        match result {
            Ok(res) => {
                if let Some(data) = res.data {
                    if let Ok(result_) = serde_json::from_value::<T>(data) {
                        return Ok(result_);
                    }
                }
                error!("no data in success response");
                return Err(RestError::JsonParseError);
            }
            Err(why) => return Err(why),
        }
    }

    /// PUBLIC APIs
    ///
    pub fn ticker(order_currency: &str) -> RestTypedResult<TickerResponse> {
        let ep = format!("/public/ticker/{}_KRW", order_currency);
        let result = Self::request_pub(reqwest::Method::GET, ep.as_str());
        Self::result_request_pub(result)
    }

    pub fn tickers() -> RestTypedResult<TickersResponse> {
        let result = Self::request_pub(reqwest::Method::GET, "/public/ticker/ALL_KRW");
        match result {
            Ok(res) => {
                if let Some(data) = res.data {
                    let mut retval = TickersResponse {
                        tickers: TickerMap::new(),
                        date: String::new(),
                    };

                    let map = data.as_object().unwrap();

                    for (k, v) in map.into_iter() {
                        if k == "date" {
                            retval.date = String::from(v.as_str().unwrap());
                            continue;
                        }
                        if let Ok(value) = serde_json::from_value::<TickerResponse>(v.clone()) {
                            retval.tickers.insert(String::from(k), value);
                        }
                    }
                    return Ok(retval);
                }
                error!("no data in success response");
                return Err(RestError::JsonParseError);
            }
            Err(why) => return Err(why),
        }
    }

    /// PRIVATE APIs
    ///
    pub fn account(&self, param: AccountParam) -> RestTypedResult<AccountResponse> {
        self.post_typed_data("/info/account", Some(serde_json::to_value(param).unwrap()))
    }

    pub fn orders(&self, param: OrdersParam) -> RestTypedResult<OrdersResponse> {
        self.post_typed_data("/info/orders", Some(serde_json::to_value(param).unwrap()))
    }

    pub fn trade_place(&self, param: PlaceParam) -> RestTypedResult<String> {
        let mut v: Value = Value::Null;
        if param.price.fract() == 0.0 {
            let iparam = PlaceParamStr {
                order_currency: param.order_currency,
                payment_currency: param.payment_currency,
                units: param.units.to_string(),
                price: (param.price as i64).to_string(),
                order_type: param.order_type,
            };
            v = serde_json::to_value(iparam).unwrap();
        } else {
            let iparam = PlaceParamStr {
                order_currency: param.order_currency,
                payment_currency: param.payment_currency,
                units: param.units.to_string(),
                price: param.price.to_string(),
                order_type: param.order_type,
            };
            v = serde_json::to_value(iparam).unwrap();
            // v = serde_json::to_value(param).unwrap();
        }
        self.post_typed_order_id("/trade/place", Some(v))
    }

    pub fn trade_cancel(&self, param: CancelParam) -> RestTypedResult<()> {
        self.post_typed_empty("/trade/cancel", Some(serde_json::to_value(param).unwrap()))
    }
}
