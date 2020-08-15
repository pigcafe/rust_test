fn main() {
    let mut s = String::from("hello");
    
    s.push_str(", world!");

    println!("{}", s);

    // No happen any memory corruption
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);

    // Clone
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    // For function
    let s5 = String::from("hello");

    let s6 = takes_ownership(s5);
    println!("{}", s6);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership (mut some_srting: String) -> String {
    println!("{}", some_srting);
    some_srting.push_str(", world!!");
    some_srting
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
