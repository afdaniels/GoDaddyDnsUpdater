use serde::{Serialize,Deserialize};
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigInfo{
    key: String,
    secret: String,
    domain: String,
    record_type: String,
    subdomain: String
}

fn load_config<P: AsRef<Path>>(path: P) -> Result<ConfigInfo, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let x = serde_json::from_reader(reader)?;
    Ok(x)
}

#[tokio::main]
async fn update_ip() {
    if let Some(ip) = public_ip::addr().await {
        println!("public ip address: {:?}", ip);
        let config = load_config("config.json").unwrap();

        let my_ip = ip.to_string();
        let key = config.key;
        let secret = config.secret;
        let domain = config.domain;
        let record_type = config.record_type;
        let sub = config.subdomain;

        let _url = format!("https://api.godaddy.com/v1/domains/{}/records/{}/{}", domain,record_type,sub);
        let authorization_value: String = format!("sso-key {}:{}", key, secret);

        let body = json!([{
            "data": my_ip
        }]);

        println!("{}", body.to_string());

        let client = reqwest::Client::new();
        let res = client
            .put(_url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", authorization_value)
            .body(body.to_string())
            .send()
            .await
            .unwrap();

        println!("body = {:?}", res);
        
    } else {
        println!("couldn't get an IP address");
    }
}


fn main() {
    update_ip();
}

