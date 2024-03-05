fn interproduct(a: i16, b: i16, c:i16) -> i16{
    return a * b + b * c + c * a;
}

fn main() {
   let greetings: &str = "Hello, ";
   let mut sentence = String::new();

   sentence.push_str(greetings);
   sentence.push_str("World");

   println!("{}",sentence);
}
