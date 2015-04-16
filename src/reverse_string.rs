// Need to comment this out because its unstable and cannot run on beta (only nightly)
// The feature unicode is required for graphemes
//#![feature(unicode)]

pub fn reverse_string(string_to_reverse: String) -> String {
    let chars = string_to_reverse.chars();
    let mut char_stack: Vec<_> = string_to_reverse.chars().collect();

    let mut answer = String::new();
    
    // Reversing a string without using a built in reverse function
    loop {
        let top = match char_stack.pop() {
            None => break, // empty
            Some(c) => c,
        };
        answer.push(top);
    }

    return answer;
}

// pub fn reverse_string_graphemes(string_to_reverse: String) -> String {
//     string_to_reverse.graphemes(true).rev().collect()
// }