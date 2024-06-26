use std::fmt::Display;

#[derive(Debug)]
pub struct Recover {
  pub watch_name: String,
  pub host: Box<str>,
  pub tag_li: Box<[String]>,
  pub duration: u64,
  pub first_warn: u64,
  pub err: u64,
}

impl Display for Recover {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "✅")?;
    crate::display!(self, f);
    Ok(())
  }
}
