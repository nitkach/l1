/// Make a trait `Action` with the `say` method,
/// which should output a message to the console.
///
/// Make a `Person` structure that contains a string name.
///
/// Make an implementation of the `Action` trait for the `Person` structure,
/// in which the `say` method will output the text "Hello, NAME" to the console
/// (where NAME is the name stored in the structure).

trait Action {
    fn say(&self);
}

struct Person {
    name: String,
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let person = Person {
        name: "John".to_owned(),
    };

    person.say();
}
