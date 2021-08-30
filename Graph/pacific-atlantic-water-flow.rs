// https://leetcode.com/problems/pacific-atlantic-water-flow/

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (i, j) = (heights.len(), heights[0].len());
        
        let mut pacific: Vec<(usize, usize)> = (0..j).map(|y| (0, y)).collect();
        pacific.append(&mut((1..i).map(|x| (x, 0)).collect()));
        
        let mut atlantic: Vec<(usize, usize)> = (0..j).map(|y| (i-1, y)).collect();
        atlantic.append(&mut((0..i-1).map(|x| (x, j-1)).collect()));
        
        dfs(pacific, &heights, &i, &j).intersection(&dfs(atlantic, &heights, &i, &j))
        .map(|&(x, y)| vec![x as i32, y as i32]).collect()
    }
}

pub fn dfs(mut stack: Vec<(usize, usize)>, constrains: &Vec<Vec<i32>>, &i: &usize, &j: &usize) -> HashSet<(usize, usize)> {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();
        seen.insert((x, y));
        for (dx, dy) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (next_x, next_y) = (x+(dx as usize), y+(dy as usize));
            if 0 <= next_x && next_x < i && 0 <= next_y && next_y < j {
                if !seen.contains(&(next_x, next_y)) {
                    if constrains[x][y] <= constrains[next_x][next_y] {
                        stack.push((next_x, next_y));
                    }
                }
            }   
        }
    }
    seen
}