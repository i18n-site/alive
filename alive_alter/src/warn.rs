use std::fmt::Display;

#[derive(Debug)]
pub struct Warn {
  pub watch_name: String,
  pub host: Box<str>,
  pub tag_li: Box<[String]>,
  pub duration: u64,
  pub err: aok::Error,
  pub times: u64,
  pub first_warn: u64,
}

impl Display for Warn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "❌")?;
    crate::display!(self, f);
    write!(f, "\n{}", self.err)?;
    Ok(())
  }
}
