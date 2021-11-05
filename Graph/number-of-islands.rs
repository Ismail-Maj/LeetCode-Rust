// https://leetcode.com/problems/number-of-islands/

struct Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut res: i32 = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    res += 1;
                    dfs(i, j, &mut grid);
                }
            }
        }
        res
    }
}

pub fn dfs(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
    if i >= grid.len() || j >= grid[0].len() || grid[i][j] != '1' {
        return;
    }
    grid[i][j] = '0';
    dfs(i + 1, j, grid);
    dfs(i - 1, j, grid);
    dfs(i, j + 1, grid);
    dfs(i, j - 1, grid);
}
