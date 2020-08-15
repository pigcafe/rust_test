fn main() {
    let mut s = String::from("hello world");

    //let hello = &s[0..5];
    //let world = &s[6..11];
    //println!("{}{}", hello, world);

    let word = first_word(&s[..]);

    println!("{}", word);

    // NOTE: No errors occurs.
    //   In the doc of rust, an error should be happened.
    //   However, if anyone no use it after here, no error happened at this line.
    //   Environment: rustc 1.45.0 (5c1f21c3b 2020-07-13) running on Ubuntu-20.04
    s.clear();

    println!("{}", s);
    //println!("{}", word);     <-- Error.
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
