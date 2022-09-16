pub fn main() {
    /////////////////////////////////////////////////////////////////////

    let mut nums = vec![1, 2, 3, 4, 5];
    println!("{:?}", nums); // Debug trait works the same
                            // Output: [1, 2, 3, 4, 5]

    // Accessing & assigning indexes works the same
    println!("{}", nums[0]);
    nums[0] = 5;
    println!("{}", nums[0]);
    // Output:
    // 1
    // 5

    /////////////////////////////////////////////////////////////////////

    // Getting length works the same
    println!("{}", nums.len());

    /////////////////////////////////////////////////////////////////////

    // As does measuring size in memory
    // Notice that size in memory reports 4 bytes larger than array with same length & element type
    println!("Size in memory: {}", std::mem::size_of_val(&nums));
    // Output:
    // Size in memory: 24

    // But now, we can add/remove, and change size
    nums.push(6);
    println!("{:?}", nums); // [5, 2, 3, 4, 5, 6]
    nums.remove(0);
    println!("{:?}", nums); // [2, 3, 4, 5, 6]

    /////////////////////////////////////////////////////////////////////

    // Iterating over vector
    // Examples from video I'm watching through say to use `nums.iter()`, but this seems to work too.
    // Note that you'll get errors if you don't use a ref for the array here,
    // you end up with this error in subsequent code:
    // error[E0382]: borrow of moved value: `nums`
    for i in &nums {
        println!("{}", i);
    }

    /////////////////////////////////////////////////////////////////////

    // This iterator, rather than giving us each variable as an immutable reference,
    // gives us a mutable reference we can use to edit values in the iterator
    for i in nums.iter_mut() {
        *i = 0;
    }

    println!("{:?}", nums);
}
