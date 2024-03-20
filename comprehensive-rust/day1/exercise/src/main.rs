struct Person{
    name: String,
    age: i32
}

fn describe(person: &Person) {
    println!("{} is {} years old.", person.name, person.age)
}

fn main() {
    let peter = Person{
        name: String::from("Peter"),
        age: 27
    };

    describe(&peter)
}
