fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    let a = five();
    let b = plus_one(5);

    println!("main: The value of x is: {}", x);
    println!("main: The value of y is: {}", y);
    println!("main: The value of a is: {}", a);
    println!("main: The value of b is: {}", b);
}

fn another_function(x: i32, y: i32){
    println!("Another function.");
    println!("another_function: The value of x is: {}", x);
    println!("another_function: The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}