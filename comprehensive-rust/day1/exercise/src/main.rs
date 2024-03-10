fn main() {
    let y = 3;
    let x = {
        let z = 10;
        y - z
    };

    println!("{}", x)
}
