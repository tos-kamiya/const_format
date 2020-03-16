// run with 'cargo run --example foo_usage

use const_format::const_format;

const USAGE: &str = const_format!(a0 = "foo", a1 = "42";
"Tool `", a0, "`, a super-duper fantastic one.

Usage:
  ", a0, " -h
  ", a0, " [-w NUM] <file>

Option:
  -w NUM    Secret number of the world [default: ", a1, "]
");

fn main() {
    println!("{}", USAGE);
}
