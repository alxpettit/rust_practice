// Tuples group together values of different types
// I wonder what binary code this translates to...

pub fn main() {
    let person: (&str, &str, i8) = ("Alexandria", "The Internet", 26);

    // Note that tuple indexing syntax here seems to mean that we can't determine index during runtime
    // - I suspect this syntax is a deliberate choice to intuitively convey this fact.
    // - Heterogeneous typing on this object + static typing = dynamic index does not really make sense during runtime.
    println!("{} is from {} and is {}", person.0, person.1, person.2);

    let max_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
    println!("{}, {}", max_tuple.0, max_tuple.13);
    // The video I'm watching while writing these examples, claimed 12 to be the max for tuples
    // This seems to be untrue, though.
    // Output: 1, 14

    // Initializing a tuple with a single element works just like in Python -- just need the comma,
    // to distinguish it from scope forcing operators.
    let single_tuple_element = (1,);
    assert_eq!(single_tuple_element.0, 1);

}