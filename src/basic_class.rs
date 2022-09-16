// Unlike most C-like languages,
// In Rust, structs and classes are not fully separate things.
// Rather, a class is basically just a struct with added implementation
// (basically, object-agnostic methods)
// and traits
// (sort of like Java's interface objects)
// See: https://doc.rust-lang.org/std/keyword.impl.html
// and: https://doc.rust-lang.org/std/keyword.trait.html
// If you are a programming baby, see comments below for more in-depth explanations of each.

/** Structs store an instantiable object, each containing a bunch of named variables (called `fields`)

    `#[allow(dead_code)]` marker prevents unused objects from generating warnings
**/
#[allow(dead_code)]
struct Person {
    // These are private by default (that is to say, inaccessible from outside object)
    // Note that privacy does not affect things within this module.
    // If you want to make these public, just prepend `pub`.
    first_name: String,
    last_name: String,
    age: u128, // Life goals: cause overflow error here
}

/** `impl` allows you to add a method to an object type.

    This is how Rust is able to have callable methods attached to primitives.
**/
#[allow(dead_code)]
impl Person {
    fn new(first: &str, last: &str, age: u128) -> Person {
        // No semicolon for return
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            age,
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn get_age(&self) -> u128 {
        self.age
    }

    fn set_age(&mut self, age: u128) {
        self.age = age
    }

    fn increment_age(&mut self) {
        self.age += 1
    }
}

pub fn main() {
    let mut me: Person = Person::new("Alexandria", "Pettit", 26);
    me.first_name = "aa".to_string();
    println!("My first name: {}", me.first_name);
    println!("My full name: {}", me.full_name());
    println!("My age: {}", me.get_age());
    // Another year passes :(
    me.increment_age();
    println!("My age after another year: {}", me.get_age());
}
