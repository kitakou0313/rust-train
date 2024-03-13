// macro
// コンパイル時に展開される
// 末尾に!をつけて呼び出す
// 可変長の引数を与えられる
fn functional(n: u32) -> u32 {
    let mut product = 1;

    for i in 1..n {
        product *= dbg!(i)
    }

    return product;
}

fn main() {
    println!("{}", functional(10))
}
