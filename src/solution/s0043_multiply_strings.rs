/**
 * [43] Multiply Strings
 *
 * Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
 *
 * Example 1:
 *
 *
 * Input: num1 = "2", num2 = "3"
 * Output: "6"
 *
 * Example 2:
 *
 *
 * Input: num1 = "123", num2 = "456"
 * Output: "56088"
 *
 *
 * Note:
 *
 * <ol>
 * 	The length of both num1 and num2 is < 110.
 * 	Both num1 and num2 contain only digits 0-9.
 * 	Both num1 and num2 do not contain any leading zero, except the number 0 itself.
 * 	You must not use any built-in BigInteger library or convert the inputs to integer directly.
 * </ol>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/multiply-strings/
// discuss: https://leetcode.com/problems/multiply-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// TODO: optimize
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1.starts_with("0") || num2.starts_with("0") {
            return "0".to_string();
        }
        let num1: Vec<char> = num1.chars().collect();
        let num2: Vec<char> = num2.chars().collect();
        let mut ret = vec![];
        for i in 0..num1.len() {
            if num1[i] == '0' {
                continue;
            }
            for j in 0..num2.len() {
                if num2[j] == '0' {
                    continue;
                }
                let zeros = num1.len() - 1 - i + num2.len() - 1 - j;
                let mut curr = Self::multiply_char_char(num1[i], num2[j]);
                Self::pad_zeros(&mut curr, zeros);
                ret = Self::add_string(ret, curr.chars().collect());
            }
        }
        ret.iter().collect()
    }

    fn multiply_char_char(c1: char, c2: char) -> String {
        (c1.to_digit(10).unwrap() * c2.to_digit(10).unwrap()).to_string()
    }

    fn add_string(num1: Vec<char>, num2: Vec<char>) -> Vec<char> {
        let mut ret = vec!['0'; num1.len().max(num2.len())];
        let mut index1 = num1.len() as i32 - 1;
        let mut index2 = num2.len() as i32 - 1;
        let mut carrier = 0;
        while index1 >= 0 || index2 >= 0 {
            let mut v1 = 0;
            if index1 >= 0 {
                v1 = num1[index1 as usize].to_digit(10).unwrap();
            }
            let mut v2 = 0;
            if index2 >= 0 {
                v2 = num2[index2 as usize].to_digit(10).unwrap();
            }
            let sum = v1 + v2 + carrier;
            let curr = sum % 10;
            carrier = sum / 10;
            ret[index1.max(index2) as usize] = std::char::from_digit(curr, 10).unwrap();
            if index1 >= 0 {
                index1 -= 1;
            }
            if index2 >= 0 {
                index2 -= 1;
            }
        }
        if carrier > 0 {
            ret.insert(0, std::char::from_digit(carrier, 10).unwrap());
        }
        ret
    }

    fn pad_zeros(s: &mut String, zeros: usize) {
        (0..zeros).map(|_| s.push('0')).count();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43() {}
}
