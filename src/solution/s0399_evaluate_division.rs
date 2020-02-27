/**
 * [399] Evaluate Division
 *
 * Equations are given in the format A / B = k, where A and B are variables represented as strings, and k is a real number (floating point number). Given some queries, return the answers. If the answer does not exist, return -1.0.
 *
 * Example:<br />
 * Given  a / b = 2.0, b / c = 3.0.<br />
 * queries are:  a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ? .<br />
 * return  [6.0, 0.5, -1.0, 1.0, -1.0 ].
 *
 * The input is:  vector<pair<string, string>> equations, vector<double>&amp; values, vector<pair<string, string>> queries , where equations.size() == values.size(), and the values are positive. This represents the equations. Return  vector<double>.
 *
 * According to the example above:
 *
 *
 * equations = [ ["a", "b"], ["b", "c"] ],
 * values = [2.0, 3.0],
 * queries = [ ["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"] ].
 *
 *
 *
 * The input is always valid. You may assume that evaluating the queries will result in no division by zero and there is no contradiction.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/evaluate-division/
// discuss: https://leetcode.com/problems/evaluate-division/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        // TODO reduce lines...
        // evaluate all values
        let mut graph = HashMap::new();
        for (i, pair) in equations.iter().enumerate() {
            let first = &pair[0];
            let second = &pair[1];
            graph.entry(first).or_insert(vec![]).push(i);
            graph.entry(second).or_insert(vec![]).push(i);
        }
        let mut map = HashMap::new();
        let mut tree_no = HashMap::new();
        let mut curr_tree_no = 0;
        for i in 0..equations.len() {
            Self::helper(
                &equations,
                &values,
                &graph,
                &mut map,
                &mut tree_no,
                &mut curr_tree_no,
                &mut HashSet::new(),
                i,
            );
        }
        let mut ret = vec![];
        for pair in queries {
            let first = &pair[0];
            let second = &pair[1];
            if !map.contains_key(first)
                || !map.contains_key(second)
                || (tree_no.get(first).unwrap() != tree_no.get(second).unwrap())
            {
                ret.push(-1_f64);
                continue;
            }
            ret.push(*map.get(first).unwrap() / *map.get(second).unwrap());
        }
        ret
    }

    fn helper<'a>(
        equations: &'a Vec<Vec<String>>,
        values: &Vec<f64>,
        graph: &HashMap<&String, Vec<usize>>,
        map: &mut HashMap<&'a String, f64>,
        tree_no: &mut HashMap<&'a String, i32>,
        curr_tree_no: &mut i32,
        evaled_equation_indices: &mut HashSet<usize>,
        curr_equation_index: usize,
    ) {
        if evaled_equation_indices.contains(&curr_equation_index) {
            return;
        }
        evaled_equation_indices.insert(curr_equation_index);
        let pair = &equations[curr_equation_index];
        let first = &pair[0];
        let second = &pair[1];
        let q = values[curr_equation_index];
        if !map.contains_key(first) && !map.contains_key(second) {
            // new graph
            *curr_tree_no += 1;
            map.insert(first, 1_f64);
            tree_no.insert(first, *curr_tree_no);
        }
        if map.contains_key(first) && !map.contains_key(second) {
            map.insert(second, *map.get(first).unwrap() / q);
            tree_no.insert(second, *curr_tree_no);
        }
        if map.contains_key(second) && !map.contains_key(first) {
            map.insert(first, *map.get(second).unwrap() * q);
            tree_no.insert(first, *curr_tree_no);
        }
        for i in graph.get(first).unwrap() {
            Self::helper(
                equations,
                values,
                graph,
                map,
                tree_no,
                curr_tree_no,
                evaled_equation_indices,
                *i,
            );
        }
        for i in graph.get(second).unwrap() {
            Self::helper(
                equations,
                values,
                graph,
                map,
                tree_no,
                curr_tree_no,
                evaled_equation_indices,
                *i,
            );
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_399() {
        println!(
            "{:?}",
            Solution::calc_equation(
                vec![vec_string!["a", "e"], vec_string!["b", "e"]],
                vec![4.0, 3.0],
                vec![
                    vec_string!["a", "b"],
                    vec_string!["e", "e"],
                    vec_string!["x", "x"],
                ],
            )
        );
    }
}
