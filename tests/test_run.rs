use std::{path::PathBuf, process::Command};

use tiny_grep::run;

// 通过 lib.rs 文件中的 run 函数测试
#[test]
fn test() {
  run(
    [
      "tiny_grep".to_string(),
      "foo".to_string(),
      "test.txt".to_string(),
    ]
    .as_ref(),
  );
}

// 通过 main.rs 编译的二进制文件测试
#[test]
fn test_cli2() {
  let mut path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
  path.push("target/debug/tiny-grep");
  let output = Command::new(path)
    .arg("H /Users/hqweay/Public/Code/rust/grep/poem.txt")
    .output()
    .unwrap();
  let out = String::from_utf8(output.stdout).unwrap();
  println!("{}", out);

  assert!(out.contains("H"));
}
