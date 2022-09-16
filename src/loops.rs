fn fizzbuzz(x: i32) {
    let fizz = x % 3 == 0;
    let buzz = x % 5 == 0;

    if fizz && buzz {
        println!("FizzBuzz");
    } else if fizz {
        println!("Fizz");
    } else if buzz {
        println!("Buzz");
    } else {
        println!("{}", x);
    }
}

pub fn main() {
    let mut count = 0;

    // Infinite loops are very easy! You don't even need a dummy check
    loop {
        count += 1;
        println!("Count: {}", count);
        if count >= 100 {
            break;
        }
    }

    ////////////////////////////////////////////////////////////////////
    // While loop fizzbuzz example
    count = 0;
    while count <= 100 {
        fizzbuzz(count);
        count += 1;
    }

    ////////////////////////////////////////////////////////////////////
    // For loop fizzbuzz example
    for x in 0..100 {
        fizzbuzz(x);
    }
}
