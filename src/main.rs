fn main() {
    let s = &"thiss is the sentence now";
    let (s, word) = first_word(&s[1..]);
    println!("the first word of '{}' is {}", s, word);
}

fn first_word(s: &str) -> (&str, &str) {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // iter() returns each element in a collection
        if item == b' ' {
            // b' ' is byte literal
            return (s, &s[..i]);
        }
    }
    (s, s)
}
