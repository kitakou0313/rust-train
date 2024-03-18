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

struct Foo{
    a: i32,
    b: i32
}

fn func(foo:Foo) {
    let Foo {a, b} = foo;
}

fn main() {
    let foo: Foo
}
