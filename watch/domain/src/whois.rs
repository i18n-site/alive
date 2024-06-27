use aok::Result;
use reqwest::Client;
use sonic_rs::{from_str, Deserialize};
use str2ts::str2ts;

#[derive(Deserialize, Debug)]
pub struct WhoisAs93Domain {
  expiration_date_in_time: String,
}

#[derive(Deserialize, Debug)]
pub struct WhoisAs93 {
  domain: WhoisAs93Domain,
}

#[derive(Deserialize, Debug)]
pub struct WhoisJson {
  expires: String,
}

#[derive(Debug)]
pub struct Whois {
  token: String,
}

impl Whois {
  pub fn new(token: &str) -> Self {
    let token = format!("Token={token}");
    Whois { token }
  }

  pub async fn expire_as93(&self, host: &str) -> Result<u64> {
    let response = reqwest::get("https://who-dat.as93.net/".to_owned() + host)
      .await?
      .text()
      .await?;

    let json: WhoisAs93 = sonic_rs::from_str(&response)?;
    Ok(str2ts(&json.domain.expiration_date_in_time)?)
  }

  pub async fn expire_whoisjson(&self, host: &str) -> Result<u64> {
    let url = "https://whoisjson.com/api/v1/whois?domain=".to_owned() + host;

    let client = Client::new();
    let response = client
      .get(url)
      .header("Authorization", &self.token)
      .send()
      .await?
      .text()
      .await?;

    let whois_response: WhoisJson = from_str(&response)?;

    Ok(str2ts(&whois_response.expires)?)
  }
}

#[derive(Debug)]
pub struct Domain {
  pub free: bool,
  pub host: String,
}

impl Domain {
  pub async fn expire(&self, whois: &Whois) -> Result<u64> {
    if self.free {
      whois.expire_as93(&self.host).await
    } else {
      whois.expire_whoisjson(&self.host).await
    }
  }

  pub fn new(host: impl Into<String>) -> Self {
    let host = host.into();
    Domain {
      free: if let Some(pos) = host.rfind('.') {
        !["top"].contains(&&host[pos + 1..])
      } else {
        true
      },
      host,
    }
  }
}
