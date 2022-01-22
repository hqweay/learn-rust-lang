use std::{error::Error, fs};

use super::bean::Config;

// Box 用于装箱；dyn 用于强调返回的值是动态分配的。
// 因为 rust 需要知道每个函数返回的类型需要多大空间，然而不同类型的不同实现需要不同的内存量，因此可以考虑使用 Box 来封装类型，
// 因为 Box 是个引用，引用的大小是静态已知的，它指向的对象保存于堆。由于 rust 会尽量明确在堆上需要使用多少内存，因此使用 dyn 告诉编译器这里需要动态计算。
pub fn find(config: Config) -> Result<(), Box<dyn Error>> {
  println!("Searching for {} in {}", config.query, config.filename);

  // ? 会从函数中返回错误值并让调用者来处理
  let contents = fs::read_to_string(config.filename)?;

  println!("With text:\n{}", contents);

  for line in contents.lines() {
    if line.contains(&config.query) {
      println!("{}", line);
      println!("{}", "====");
    }
  }
  Ok(()) //没有返回值，只是为了消除编译器的警告
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn run_test() {
    let config = Config::new(&[
      "grep".to_string(),
      "hello".to_string(),
      "testlib.rs".to_string(),
    ])
    .unwrap();
    if let Err(e) = find(config) {
      eprintln!("Application error: {}", e)
    }
  }
}
