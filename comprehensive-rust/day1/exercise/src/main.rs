fn gcd(a: u32, b:u32) -> u32 {
    // 最後のexpressionが自動的に返り値になる
    // ;をつけると返らない
    if b > 0 {
        {gcd(b, a % b)}
    }else {
        a
    }
}

fn main() {
    println!("{}", gcd(101, 10))
}
