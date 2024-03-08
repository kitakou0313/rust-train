fn fib(n: u32) -> u32 {
    return if n == 0 || n == 1 {1} else {fib(n-1) + fib(n-2)};
}

fn main() {
    let x = 10;
    println!("fib:{}", fib(x))
}
