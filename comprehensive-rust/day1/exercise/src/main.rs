fn collatz_length(mut n: i32) -> u32 {
    if n == 0 {
        panic!("n = 0")
    }

    let mut length = 1;
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2
        }else {
            n = 3 * n + 1
        }
        length += 1;
    }
    return length;
}
  
  fn main() {
    dbg!(collatz_length(3));
  }
