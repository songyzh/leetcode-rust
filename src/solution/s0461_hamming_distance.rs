/**
 * [461] Hamming Distance
 *
 * The <a href="https://en.wikipedia.org/wiki/Hamming_distance" target="_blank">Hamming distance</a> between two integers is the number of positions at which the corresponding bits are different.
 *
 * Given two integers x and y, calculate the Hamming distance.
 *
 * Note:<br />
 * 0 &le; x, y < 2^31.
 *
 *
 * Example:
 *
 * Input: x = 1, y = 4
 *
 * Output: 2
 *
 * Explanation:
 * 1   (0 0 0 1)
 * 4   (0 1 0 0)
 *        &uarr;   &uarr;
 *
 * The above arrows point to positions where the corresponding bits are different.
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut ret = 0;
        let mut x = x;
        let mut y = y;
        while x != 0 || y != 0 {
            if x & 1 != y & 1 {
                ret += 1;
            }
            x >>= 1;
            y >>= 1;
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_461() {}
}
