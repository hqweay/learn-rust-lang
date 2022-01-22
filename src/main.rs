use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("开始执行：{}", args.join(" ").as_str());
  tiny_grep::run(&args);
}
