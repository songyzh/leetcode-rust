/**
 * [393] UTF-8 Validation
 *
 * A character in UTF8 can be from 1 to 4 bytes long, subjected to the following rules:
 * <ol>
 * For 1-byte character, the first bit is a 0, followed by its unicode code.
 * For n-bytes character, the first n-bits are all one's, the n+1 bit is 0, followed by n-1 bytes with most significant 2 bits being 10.
 * </ol>
 * This is how the UTF-8 encoding would work:
 *
 *    Char. number range  |        UTF-8 octet sequence
 *       (hexadecimal)    |              (binary)
 *    --------------------+---------------------------------------------
 *    0000 0000-0000 007F | 0xxxxxxx
 *    0000 0080-0000 07FF | 110xxxxx 10xxxxxx
 *    0000 0800-0000 FFFF | 1110xxxx 10xxxxxx 10xxxxxx
 *    0001 0000-0010 FFFF | 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
 *
 *
 * Given an array of integers representing the data, return whether it is a valid utf-8 encoding.
 *
 *
 * Note:<br />
 * The input is an array of integers. Only the least significant 8 bits of each integer is used to store the data. This means each integer represents only 1 byte of data.
 *
 *
 *
 * Example 1:
 *
 * data = [197, 130, 1], which represents the octet sequence: 11000101 10000010 00000001.
 *
 * Return true.
 * It is a valid utf-8 encoding for a 2-bytes character followed by a 1-byte character.
 *
 *
 *
 *
 * Example 2:
 *
 * data = [235, 140, 4], which represented the octet sequence: 11101011 10001100 00000100.
 *
 * Return false.
 * The first 3 bits are all one's and the 4th bit is 0 means it is a 3-bytes character.
 * The next byte is a continuation byte which starts with 10 and that's correct.
 * But the second continuation byte does not start with 10, so it is invalid.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/utf-8-validation/
// discuss: https://leetcode.com/problems/utf-8-validation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut mask_last_8 = 0b1111_1111;
        let ascii_limit = 0b1000_0000;
        let mut utf8_folling_bytes = 0;
        for num in data.iter().rev() {
            let num = *num & mask_last_8;
            if num < ascii_limit {
                continue;
            }
            // if gets here, the 8th bit must be 1
            if num & 0b0100_0000 == 0 {
                // the 7th bit is 0, utf8 following byte
                utf8_folling_bytes += 1;
                if utf8_folling_bytes == 4 {
                    return false;
                }
                continue;
            }
            // utf8 starting byte
            let mut ones_before_zero = 0;
            for i in (0..8).rev() {
                if num & (1 << i) > 0 {
                    ones_before_zero += 1;
                } else {
                    break;
                }
            }
            if ones_before_zero - 1 != utf8_folling_bytes {
                return false;
            }
            utf8_folling_bytes = 0;
        }
        utf8_folling_bytes == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_393() {
        println!("{}", Solution::valid_utf8(vec![145]));
    }
}
