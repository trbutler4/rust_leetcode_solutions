mod roman_to_int;
fn main() {
    check_roman_to_int();
}

fn check_roman_to_int() {
    let s: String = String::from("III");
    println!("Example 1");
    println!("{}", roman_to_int::solution1(s));
}