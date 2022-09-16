// Experimented with fancy markdown comments in this one.
// I'm fairly sure that I violated some standards in doing so, but the result was more readable.
pub fn main() {
    // primitive array
    let mut array1 = [1, 2, 3];
    // Note: assignment here copies the data, as it does with all other primitive values
    let array2 = array1;
    array1[0] = 42;
    println!("Values: {:?}", (array1, array2));
    // ## Output: `Values: ([42, 2, 3], [1, 2, 3])`
    // Note that change does not propagate to `array2`.
    // If you're dealing with a non-primitive, you can expect the first one to no longer hold the value
    // This avoids unnecessary copying of large objects in memory, and also avoids accidental editing.
    // For instance, if you try to do this...
    // ```
    // let vector1 = vec![1, 2, 3];
    // let vector2 = vector1;
    // println!("Values: {:?}", (vector1, vector2));
    // ```
    // You get `use of moved value` error, here.
    // Since the object doesn't implement the copy trait, it will try to move it to the new variable.
    // (note that this doesn't necessarily imply a memory operation -- it's just changing the word your program has for that location in memory, AFAIK)
    // ---------------------------------------------------------------
    //
    // Aaand here's an example of read-only variable 'duplication' by using references.
    let vector1 = vec![1, 2, 3];
    let vector2 = &vector1;

    // Note: both of these are still in the same memory location. Both can be accessed.
    // There's probably a way to do the assignment thing, but I dunno how.
    // ```
    // vector1[0] = 42;
    //
    println!("Values: {:?}", (&vector1, vector2));
}
