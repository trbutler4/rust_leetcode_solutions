/*
You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.

Return the merged string.

 

Example 1:

Input: word1 = "abc", word2 = "pqr"
Output: "apbqcr"
Explanation: The merged string will be merged as so:
word1:  a   b   c
word2:    p   q   r
merged: a p b q c r
Example 2:

Input: word1 = "ab", word2 = "pqrs"
Output: "apbqrs"
Explanation: Notice that as word2 is longer, "rs" is appended to the end.
word1:  a   b 
word2:    p   q   r   s
merged: a p b q   r   s
Example 3:

Input: word1 = "abcd", word2 = "pq"
Output: "apbqcd"
Explanation: Notice that as word1 is longer, "cd" is appended to the end.
word1:  a   b   c   d
word2:    p   q 
merged: a p b q c   d
 

Constraints:

1 <= word1.length, word2.length <= 100
word1 and word2 consist of lowercase English letters.
*/

#[allow(dead_code)]
pub fn solution1(word1: String, word2: String) -> String {
    // basically just merge sort 

    // base case 
    if word1.is_empty() {
        return word2;
    }
    if word2.is_empty() {
        return word1;
    }

    // grab first character of each word 
    let c1 = &word1[0..1];
    let c2 = &word2[0..1];

    // recusively pass slices 
    let sliced_word1 = &word1[1..];
    let sliced_word2 = &word2[1..];

    format!("{}{}{}", c1, c2, solution1(sliced_word1.to_string(), sliced_word2.to_string()))
}