// fn double(n: i32) -> i32 {
//     // expressions are implicitly returned
//     n * 2
// }

// // take a string in and the move it back
// fn greet(s: String) -> String {
//     println!("Hello {s}!");
//     s
// }

// // does not take a string, but takes _access_ to a string
// fn greet_borrow(s: &String) {
//     println!("Hello {s}!");
// }

// fn greet_borrow_mut(s: &mut String) {
//     *s += "!!!"
// }

fn read_line() -> String {
    let mut input = String::new();
    // throws away the error
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string() // trim returns str, which is immutable
}
fn main() {
    println!("Enter your name:");
    let input = read_line();
    println!("Hello {input}!")
}
