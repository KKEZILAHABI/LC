//Palindrome Number
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {

        let str_1 = x.to_string();
        let str_2: String = str_1.chars().rev().collect();

        str_1 == str_2        
    }

}