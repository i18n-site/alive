use aok::Result;
use reqwest::Client;
use sonic_rs::{from_str, Deserialize};
use str2ts::str2ts;
use whois::WhoIs;
use whois::WhoIsLookupOptions;

pub const NOT_FREE: [&str; 3] = ["ua", "top", "org"];

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

    async fn expire_whois_lookup(&self, host: &str) -> Result<u64> {
        // Perform a native WHOIS lookup
        let whois = WhoIs::from_string("")?;
        let result = whois.lookup(WhoIsLookupOptions::from_string(host))?;

        // Attempt to find an expiration line in the result.
        // We'll look for a line starting with 'Expiration' or 'Expiry' and parse it.
        let mut expiration_line = None;
        for line in result.lines() {
            if line.to_lowercase().contains("expir") {
                expiration_line = Some(line.trim().to_string());
                break;
            }
        }

        // If we found an expiration line, try to extract a date
        // We'll attempt to parse the whole line with str2ts directly,
        // assuming it can handle common date formats. If not, you'll need to
        // parse/format the date string more carefully.
        if let Some(line) = expiration_line {
            // Often WHOIS expiration lines look like: "Expiration Date: YYYY-MM-DD"
            // Let's try to extract the date portion.
            // We'll split by spaces and try each token with str2ts.
            for token in line.split_whitespace() {
                if let Ok(ts) = str2ts(token) {
                    return Ok(ts);
                }
            }
        }

        // If we cannot find or parse expiration date, return an error
        Err("Could not parse expiration date".into())
    }

    pub async fn expire_as93(&self, host: &str) -> Result<u64> {
        // Previously called external API, now replaced with local WHOIS lookup
        self.expire_whois_lookup(host).await
    }

    pub async fn expire_whoisjson(&self, host: &str) -> Result<u64> {
        // Previously called external API, now replaced with local WHOIS lookup
        self.expire_whois_lookup(host).await
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
        !NOT_FREE.contains(&&host[pos + 1..])
      } else {
        true
      },
      host,
    }
  }
}
