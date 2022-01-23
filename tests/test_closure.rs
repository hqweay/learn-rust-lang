#[test]
fn test_1() {
  let add_3 = |item: i32| println!("{}", item + 3);
  add_3(1);
}

#[test]
fn test_2() {
  fn add_x(add: fn(i32) -> i32, item: i32) {
    println!("{}", add(item));
  }

  let add_4 = |item: i32| item + 4;

  add_x(add_4, 1);
}

// 返回值类型为方法的闭包
#[test]
fn test_3() {
  fn multify_add_x(add: Box<dyn Fn(i32) -> i32>, _item: i32) -> impl Fn(i32) -> i32 {
    Box::new(move |item| -> i32 {
      println!("{}", add(item * _item));
      add(item * _item)
    })
  }
  let add_3 = |item: i32| item + 3;
  let multify_2_add_3 = multify_add_x(Box::new(add_3), 2);
  multify_2_add_3(1);
}
