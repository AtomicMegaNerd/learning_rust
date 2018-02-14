pub fn add_nums(a: i64, b: i64) -> i64 {
  // Add two numbers together and return the result
  return a + b;
}

pub fn func_ptrs() {
  const ROFL: i64 = 99;
  let t = ROFL + 1;
  let xray = add_nums(32, 41);
  let alias1 = add_nums;
  let alias2: fn(i64, i64) -> i64 = add_nums;
  let t2 = alias1(3, 4);

  println!("{0} {1} {2} {3}", t, xray, t2, alias2(3, 4));
}
