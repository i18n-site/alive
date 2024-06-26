#[derive(Debug, Clone)]
pub struct TaskMeta {
  pub host: Box<str>,
  pub tag_li: Box<[String]>,
  pub interval: u64,
  pub first_warn: u64,
}

#[derive(Debug, Clone)]
pub struct Task<Arg> {
  pub arg: Arg,
  pub meta: TaskMeta,
}

impl<Arg> Task<Arg> {
  pub fn new_with_first_warn(
    host: impl AsRef<str>,
    arg: Arg,
    tag_li: impl Into<Box<[String]>>,
    interval: u64,
    first_warn: u64,
  ) -> Self {
    Self {
      arg,
      meta: TaskMeta {
        interval,
        host: host.as_ref().into(),
        tag_li: tag_li.into(),
        first_warn,
      },
    }
  }

  pub fn new(
    host: impl AsRef<str>,
    arg: Arg,
    tag_li: impl Into<Box<[String]>>,
    interval: u64,
  ) -> Self {
    Self::new_with_first_warn(host, arg, tag_li, interval, 3)
  }
}
