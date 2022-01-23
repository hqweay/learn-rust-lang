use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

#[test]
fn test_match() {
  let f = File::open("hello.txt");

  match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => panic!("Problem opening the file: {:?}", other_error),
    },
  };
}

#[test]
fn test_error() {
  File::open("hello.txt").unwrap_or_else(|err| {
    if err.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|err| {
        panic!("Problem creating the file: {:?}", err);
      })
    } else {
      panic!("Problem opening the file: {:?}", err);
    }
  });
}

#[test]
fn test_unwrap() {
  // 失败时 panic 的简写：unwrap 和 expect
  File::open("hello.txt").unwrap();
  File::open("hello.txt").expect("Failed to open hello.txt");
}

#[test]
fn test() {
  test_throw_error().unwrap();
}

// ? 会将错误返回给调用者
// 相当于条件判断，如果正常返回 Ok，否则返回 Err
fn test_throw_error() -> Result<String, io::Error> {
  let mut s = String::new();

  File::open("hello.txt")?.read_to_string(&mut s)?;

  Ok(s)
}
