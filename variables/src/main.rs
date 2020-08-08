const MAX_POINTS: u32 = 100_000;
fn main() {

    // 変数は、デフォルトで不変
    // mut を付与することで、可変になる
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 再代入の場合は、mut 不要
    // シャドーイング、という
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // シャドーイングは、型違いも可能
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is : {}", spaces)
}
