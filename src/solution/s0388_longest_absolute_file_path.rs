/**
 * [388] Longest Absolute File Path
 *
 * Suppose we abstract our file system by a string in the following manner:
 *
 * The string "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext" represents:
 *
 * dir
 *     subdir1
 *     subdir2
 *         file.ext
 *
 *
 * The directory dir contains an empty sub-directory subdir1 and a sub-directory subdir2 containing a file file.ext.
 *
 * The string "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext" represents:
 *
 * dir
 *     subdir1
 *         file1.ext
 *         subsubdir1
 *     subdir2
 *         subsubdir2
 *             file2.ext
 *
 *
 * The directory dir contains two sub-directories subdir1 and subdir2. subdir1 contains a file file1.ext and an empty second-level sub-directory subsubdir1. subdir2 contains a second-level sub-directory subsubdir2 containing a file file2.ext.
 *
 * We are interested in finding the longest (number of characters) absolute path to a file within our file system. For example, in the second example above, the longest absolute path is "dir/subdir2/subsubdir2/file2.ext", and its length is 32 (not including the double quotes).
 *
 * Given a string representing the file system in the above format, return the length of the longest absolute path to file in the abstracted file system. If there is no file in the system, return 0.
 *
 * Note:<br />
 *
 * The name of a file contains at least a . and an extension.
 * The name of a directory or sub-directory will not contain a ..
 *
 *
 *
 * Time complexity required: O(n) where n is the size of the input string.
 *
 * Notice that a/aa/aaa/file1.txt is not the longest file path, if there is another path aaaaaaaaaaaaaaaaaaaaa/sth.png.
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-absolute-file-path/
// discuss: https://leetcode.com/problems/longest-absolute-file-path/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut max_len = 0_usize;
        // store previous parts' lengths
        let mut stack = vec![];
        for path in input.split("\n") {
            let level = path.rfind('\t');
            let level = if level.is_none() {
                0
            } else {
                level.unwrap() + 1
            };
            let curr_len;
            if level == 0 {
                // root
                curr_len = path.len();
                // remove previous roots
                stack.clear();
            } else {
                while stack.len() > level {
                    // remove unrelated previous parts
                    stack.pop();
                }
                // prev length + curr_total_len - tabs + '/'
                curr_len = stack.last().unwrap() + path.len() - level + 1;
            }
            stack.push(curr_len);
            if path.contains('.') {
                max_len = max_len.max(curr_len);
            }
        }
        max_len as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_388() {}
}
