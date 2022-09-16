#[allow(unused_mut)]
#[allow(dead_code)]
enum State {
    Success,
    Fail,
    Unknown,
}

#[allow(unused_mut)]
pub fn main() {
    let state: State = State::Fail;
    let mut output: String = "State: ".to_string();

    match state {
        State::Success => output.push_str("success"),
        State::Fail => output.push_str("failure"),
        State::Unknown => output.push_str("unknown"),
    }

    println!("{}", output);
}
