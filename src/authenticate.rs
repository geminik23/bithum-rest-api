


pub struct Authenticate{
    pub api:String,
    pub secret:String,
    pub encoded_api:String,
    pub encoded_secret:String,
}


impl Authenticate{
    pub fn from(api:&str, secret:&str)->Self{
        Authenticate{
            api:String::from(api),
            secret:String::from(secret),
            encoded_api: String::from(""),
            encoded_secret: String::from(""),
        }
    }


}
