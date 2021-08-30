// https://leetcode.com/problems/course-schedule/
// this version is faster for dense graphs
// check this solution for sparse graphs https://leetcode.com/problems/course-schedule/discuss/658436/Rust-Easy-to-understand-recursive-DFS.-(0-ms-faster-than-100.)

struct Solution;

const PREREQUISITE: usize = 0;
const COURSE: usize = 1;

use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Status {
    Todo,
    InProgress,
    Done,
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;

        let mut graph = vec![vec![false; num_courses]; num_courses];
        for edge in prerequisites.iter() {
            graph[edge[0] as usize][edge[1] as usize] = true;
        }

        let mut status = vec![Status::Todo; num_courses];
        (0..num_courses).all(|course| !has_cycle(course, &mut status, &graph))
    }
}

fn has_cycle(course: usize, status: &mut Vec<Status>, graph: &Vec<Vec<bool>>) -> bool {
    match status[course] {
        Status::Done => false,
        Status::InProgress => true,
        _ => {
            status[course] = Status::InProgress;
            if graph[course].iter().enumerate().any(|(i, &b)| b && has_cycle(i, status, graph)) {
                return true;
            }
            status[course] = Status::Done;
            false
        }
    }
}