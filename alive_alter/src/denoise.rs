pub fn should_send(n: u64, first_warn: u64) -> bool {
  if n < first_warn {
    return false;
  }
  let n = n - first_warn;
  match n {
    0..=9 => n == 0,
    10..=70 => (n - 10) % 20 == 0,
    71..=1540 => (n - 70) % (4 * 60) == 0,
    _ => n % (24 * 60) == 0,
  }
}
