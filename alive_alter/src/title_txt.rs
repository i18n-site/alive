pub fn title_txt(msg: &str) -> (&str, &str) {
  if let Some(pos) = msg.find('\n') {
    (&msg[..pos], &msg[pos + 1..])
  } else {
    (msg, "")
  }
}
