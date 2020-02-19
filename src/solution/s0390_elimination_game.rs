/**
 * [390] Elimination Game
 *
 *
 * There is a list of sorted integers from 1 to n. Starting from left to right, remove the first number and every other number afterward until you reach the end of the list.
 *
 * Repeat the previous step again, but this time from right to left, remove the right most number and every other number from the remaining numbers.
 *
 * We keep repeating the steps again, alternating left to right and right to left, until a single number remains.
 *
 * Find the last number that remains starting with a list of length n.
 *
 * Example:
 *
 * Input:
 * n = 9,
 * <u>1</u> 2 <u>3</u> 4 <u>5</u> 6 <u>7</u> 8 <u>9</u>
 * 2 <u>4</u> 6 <u>8</u>
 * <u>2</u> 6
 * 6
 *
 * Output:
 * 6
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/elimination-game/
// discuss: https://leetcode.com/problems/elimination-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        Self::left(n)
    }

    pub fn left(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n % 2 == 1 {
            2 * Self::right((n - 1) / 2)
        } else {
            2 * Self::right(n / 2)
        }
    }

    pub fn right(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n % 2 == 1 {
            2 * Self::left((n - 1) / 2)
        } else {
            // add 1 to all elements, then minus 1 from the result
            2 * Self::left(n / 2) - 1
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_390() {
        println!("{}", Solution::last_remaining(9));
    }
}
