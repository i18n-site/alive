use alive_alter::{denoise, title_txt, Recover, Warn};
use aok::{Result, OK};
use sonic_rs::{json, to_string, Value};
use tracing::warn;
use xstr::cut;

genv::s!(LARK_BOT);

#[derive(Debug, Default)]
pub struct Alter {}

impl Alter {
  async fn _send(&self, title: &str, txt: &str) -> Result<()> {
    let title = cut(title, 255);
    let txt = cut(txt, 10000);
    let li: [Value; 2] = [
      json!({"tag":"text","text":txt}),
      json!({
        "tag": "at",
        "user_id": "all", //取值使用"all"来at所有人
      }),
    ];
    let msg =
      json!({"msg_type":"post","content":{"post":{"zh_cn":{"title":title,"content":[li]}}}});
    let r = ireq::post(&*LARK_BOT, to_string(&msg)?).await?;
    // "{\"StatusCode\":0,\"StatusMessage\":\"success\",\"code\":0,\"data\":{},\"msg\":\"success\"}"
    if !r.contains("\"StatusCode\":0,") {
      warn!(r);
    }
    OK
  }
}

impl alive_alter::Alter for Alter {
  async fn warn(&self, warn: &Warn) -> Result<()> {
    if denoise::should_send(warn.times, warn.first_warn) {
      let warn = warn.to_string();
      let (title, txt) = title_txt(&warn);
      self._send(title, txt).await?;
      self._send(&warn.to_string(), "").await?;
    }
    OK
  }

  async fn recover(&self, recover: &Recover) -> Result<()> {
    if recover.err >= recover.first_warn {
      self._send(&recover.to_string(), "").await?;
    }
    OK
  }
}

// use aok::OK;
//
//
// pub async fn send(
//   title: impl AsRef<str>,
//   txt: impl AsRef<str>,
//   url: impl AsRef<str>,
// ) -> aok::Result<()> {
//   let title = cut(title.as_ref(), 255);
//   let txt = cut(txt.as_ref(), 10000);
//   let url = url.as_ref();
//
//   let mut li: Vec<Value> = Vec::with_capacity(2);
//
//   let txt = txt.to_owned() + "\n";
//
//   li.push(json!({"tag":"text","text":txt}));
//
//   li.push(json!({
//     "tag": "at",
//     "user_id": "all", //取值使用"all"来at所有人
//   }));
//
//   if !url.is_empty() {
//     li.push(json!({"tag":"a","text":url,"href":url}));
//   };
//
//   let msg = json!({"msg_type":"post","content":{"post":{"zh_cn":{"title":title,"content":[li]}}}});
//
//   let url: &str = LARK_BOT.as_ref();
//   ireq::post(url, to_string(&msg)?).await?;
//   OK
// }
