fn main() {
    let a = 10;

    {
        let a = 20;
        println!("{}", a);

        // shadowing
        let a = true;
        println!("{}", a)
    }

    println!("{}", a)
}
