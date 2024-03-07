fn take_u32(i: u32) {
    println!("{} is u32", i)
}

fn take_i8(i: i8) {
    println!("{} is i8", i)
}

fn main() {
    // 型を指定しない場合，関数への渡され方，参照され方から型を推論する
    let x = 10;
    let y = 2;

    take_u32(x);
    take_u32(y);
    take_i8(y);
}
