

// NOTE: this is not efficient at all
pub fn solution(s: String) -> i32 {
    // replace specific cases, and then just add digits 
    let s_flat = s
        .replace("IV", "IIII")
        .replace("IX", "VIIII")
        .replace("XL", "XXXX")
        .replace("XC", "LXXXX")
        .replace("CD", "CCCC")
        .replace("CM", "DCCCC");

    // sum the values without the edge cases 
    s_flat.chars().map(|c| {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }).sum()

}

pub fn example1() {
    let s: String = String::from("III");
    println!("Example 1");
    println!("{}", solution(s));
}