/* Arrays are:
   - fixed length
   - homogeneously typed (all items have the same type -- tho not sure about subtyping
*/

pub fn main() {
    // Notice the semicolon delimiter is used to spec array length in the type
    // Notice also that type is literally represented as square brackets, with the type name inside.
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", nums);
    // output: [1, 2, 3, 4, 5]
    ///////////////////////////////////////////////////////////////////
    // Implicit typing works too
    // Note: cargo doesn't warn of unused variables when prefixed with underscore
    let nums2 = [1, 2, 3, 4, 5];
    assert_eq!(nums2, nums);
    ///////////////////////////////////////////////////////////////////
    // Retrieving values by index works as you'd expect
    println!("{}", nums[0]);
    // Output: 1

    ///////////////////////////////////////////////////////////////////
    // Comparison seems to work as expected
    if nums == nums2 {
        println!("`nums` and `nums2` are equal");
    }

    ///////////////////////////////////////////////////////////////////

    // Immutability applies to elements inside array, as well as the array itself
    let mut nums3 = [1, 2, 3, 4, 5];

    // So, we can overwrite specific elements
    nums3[0] = 4;

    println!("{:?}", nums3);
    // Output: [4, 2, 3, 4, 5]

    ///////////////////////////////////////////////////////////////////

    // We can also reassign the whole thing, but it MUST be an array of length 5 -- the length is part of the type

    nums3 = [5, 4, 3, 2, 1];
    println!("{:?}", nums3);

    ///////////////////////////////////////////////////////////////////

    // Can retrieve length via member, like Python:
    let length = nums3.len();
    // I wonder if len() actually translates to a function call in the binary...
    // I suspect it's functioning like a macro, given that it's `const` and has a single line of code, in impl.
    println!("{}", length);

    ///////////////////////////////////////////////////////////////////

    // Getting the amount of memory the array takes up.
    // Note that you have to pass a reference, not the array itself.
    // Note also that you don't need to use any import or include statements for this.
    // `std` namespace is accessible globally without that.
    println!(
        "Array takes up {} bytes in memory",
        std::mem::size_of_val(&nums3)
    );
    // Output: Array takes up 20 bytes of memory

    ///////////////////////////////////////////////////////////////////

    // Can also replace array with array ref. Debug trait doesn't care...
    let num_ref = &nums3;
    println!("Slice: {:?}", num_ref);

    ///////////////////////////////////////////////////////////////////

    // Can use array ref to slice -- implicitly changes where we're pointing in memory
    let slice_of_first_three = &nums3[0..2];
    println!("Slice: {:?}", slice_of_first_three);
}
