
extern crate base64;
extern crate ring;
extern crate hex;

use ring::{digest, hmac};
use hex::encode as hexify;
 
#[derive(Debug, Clone)]
pub struct Authenticate{
    pub api:String,
    pub secret:String,
}


impl Authenticate{
    pub fn from(api:&str, secret:&str)->Self{
        Authenticate{
            api:String::from(api),
            secret:String::from(secret),
        }
    }


    pub fn signature(&self, endpoint:&str, nonce:i64, query:&str)->Option<String>{
        let signed_key = hmac::SigningKey::new(&digest::SHA512, self.secret.as_bytes());
        let splinter = 0 as char;
        let sign_message = format!("{}{}{}{}{}", endpoint, splinter, query, splinter, nonce);
        // let signature = hexify(hmac::sign(&signed_key, sign_message.as_bytes()));
        let signature = base64::encode(hexify(hmac::sign(&signed_key, sign_message.as_bytes())).as_str());
        Some(signature)
    }


}
