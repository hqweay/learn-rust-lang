mod math_trait {
  // trait 类似于其它语言的接口
  trait Math {
    fn add(&self, other: i32) -> Number;
    fn sub(&self, other: i32) -> Number {
      self.add(other * -1)
    }
    fn divide(&mut self, other: i32) -> &Number;
    fn multiply(&mut self, other: i32);
  }
  struct Number {
    value: i32,
  }
  // 为 Point 实现 Math trait
  impl Math for Number {
    fn add(&self, other: i32) -> Number {
      Number {
        value: self.value + other,
      }
    }
    // 把自己当作可变引用传入，操作一番
    fn divide(&mut self, other: i32) -> &Number {
      self.value /= other;
      self
    }
    fn multiply(&mut self, other: i32) {
      self.value *= other;
    }
  }

  #[test]
  fn test_multiply() {
    let mut number = Number { value: 10 };
    number.multiply(2); // 可变引用直接计算
    println!("{}", number.value);
  }

  #[test]
  fn test_divide() {
    let mut item = Number { value: 10 };
    // 可变引用操作的同时方法也有返回值，这里就相当于 Java 传了个引用进方法，直接操作一番又把原引用返回
    let haha = item.divide(2);
    println!("{}", item.value);
    // println!("{}", haha.value);
  }

  #[test]
  fn test_add() {
    let item = Number { value: 10 };
    println!("{}", item.add(2).value);
  }
  #[test]
  fn test_trait_bound() {
    fn math_add_2<T: Math>(item: T) -> Number {
      item.add(2)
    }
    fn math_sub_2<T: Math>(item: T) -> Number {
      item.sub(2)
    }

    let mut item = Number { value: 1 };
    item = math_add_2(item);
    println!("{}", item.value);

    let mut item = Number { value: 1 };
    item = math_sub_2(item);
    println!("{}", item.value);
  }
}
