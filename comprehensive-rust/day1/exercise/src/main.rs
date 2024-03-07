fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }

    return fib(n-1) + fib(n-2);
}

fn main() {
    let x = 10;
    println!("fib:{}", fib(x))
}
