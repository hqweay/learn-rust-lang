mod util;
use std::process;

use util::application::find;
use util::bean::Config;

pub fn run(args: &[String]) {
  // unwrap_or_else 定义于标准库 Result<T, E>，可以进行一些自定义的非panic!的错误处理
  // 方法内的参数是一个匿名函数，用于处理错误
  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
  // 如果 find 函数返回 Err，则退出程序
  if let Err(e) = find(config) {
    eprintln!("Application error: {}", e);
    process::exit(1);
  }
}
