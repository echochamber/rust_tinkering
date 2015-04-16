
pub fn reverse_string(stringToReverse: String) -> String {
    let mut chars = stringToReverse.chars();
    let mut reversed: Vec<char> = Vec::new();

    for c in chars {
        reversed.push(c);
    }

    let mut answer = String::new();
    loop {
        let top = match reversed.pop() {
            None => break, // empty
            Some(c) => c,
        };
        answer.push(top);
    }

    return answer;
}