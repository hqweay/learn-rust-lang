impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    // config 应该保留一份自己的属性，而不是直接拷贝 args；从而避免所有权的问题。
    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}

pub struct Config {
  pub query: String,
  pub filename: String,
}
