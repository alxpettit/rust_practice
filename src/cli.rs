pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    // println!("Args: {:?}", args);
    if args.len() >= 2 {
        let cmd = &args[1];
        if cmd.contains("hello") {
            // We print this iff a command is passed containing 'hello'
            println!("Hello world!");
        }
    }
}
