mod roman_to_int;
mod merge_alternately;

fn main() {
    let word1: String = String::from("abc");
    let word2: String = String::from("pqr");
    println!("{}", merge_alternately::solution1(word1, word2));
}