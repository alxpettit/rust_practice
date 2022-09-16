// Variables hold primitive data or references to data
// Variables are immutable by default
// Variables defined in a block are inaccessible outside of that block ("block-scoped")

pub fn main() {
    let name = "Alexandria";
    let mut age = 25;

    println!("My name is {name}, and I am {age}", name=name, age=age);
    age = 26; // I'm getting old :(
    println!("My name is {name}, and I am {age}", name=name, age=age);
    // aaagh I'm 26 already I belong in a nursing home >.<

    // Example of defining constant
    const ID: i32 = 8092457;
    println!("ID: {id}", id=ID);

    // Assigning multiple variables at once
    let (my_name, my_age) = ("Alexandria", 22);
    // Notice that this syntax is also just like Python! Except the `let` bit >.> & semicolon

    // turn back the clock... it's the only way to survive
    println!("{} is {}", my_name, my_age);
    // Something tells me that age doesn't work that way
    // Oh well... there's always cryopreservation and life-extension technologies, right?
}

// pub fn run() {
//     let name = "Alexandria";
//     let age = 25;
//
//     age = 26; // I'm getting old :(
//     // You get: "cannot assign twice to immutable variable"
//     println!("My name is {name}, and I am {age}", name=name, age=age);
// }