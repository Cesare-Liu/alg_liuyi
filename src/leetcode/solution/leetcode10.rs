/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 */

// @lc code=start
use crate::leetcode::solution::Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
		let s1: Vec<char> = s.chars().collect();
		let p1: Vec<char> = p.chars().collect();
		let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
		pub fn is_same(i: usize, j: usize) -> bool {
			if s1[i] == p1[j] {
				return true;
			}
			if p[j] = '*' {
				return true;
			}
			false	
		}
		dp[0][0] = true;
		for i in 1..s.len() + 1 {
			dp[i][0] = false;
		}
		for i in 1..s.len() + 1 {
			for j in 0..p.len() + 1 {

			}
		}
		if s1.len() >= 2 && s1[1] == '*' {

		}
    }
}
// @lc code=end

