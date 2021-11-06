// https://leetcode.com/problems/alien-dictionary/

struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    // O(V + E) time - O(V) space - topological sort
    pub fn alien_order(words: Vec<String>) -> String {
        let mut graph: HashMap<char, Vec<char>> = HashMap::new();
        let mut in_degree: HashMap<char, i32> = HashMap::new();
        let mut result: Vec<char> = vec![];

        for i in 0..words.len() - 1 {
            let word1 = words[i].chars().collect::<Vec<char>>();
            let word2 = words[i + 1].chars().collect::<Vec<char>>();
            let mut j = 0;
            while j < word1.len() && j < word2.len() {
                if word1[j] != word2[j] {
                    graph.entry(word1[j]).or_insert(vec![]).push(word2[j]);
                    in_degree.entry(word2[j]).and_modify(|x| *x += 1).or_insert(1);
                    break;
                }
                j += 1;
            }
            if j == usize::min(word1.len(), word2.len()) {
                if word1.len() > word2.len() {
                    return "".to_string();
                }
            }
        }

        let m: HashSet<char> = words.iter().flat_map(|w| w.chars()).collect();

        let mut stack: Vec<char> = m.difference(&in_degree.keys().cloned().collect::<HashSet<char>>()).cloned().collect();

        while !stack.is_empty() {
            let c = stack.pop().unwrap();
            result.push(c);
            if let Some(neighbors) = graph.get(&c) {
                for neighbor in neighbors {
                    in_degree.entry(*neighbor).and_modify(|x| *x -= 1);
                    if in_degree[&neighbor] == 0 {
                        stack.push(*neighbor);
                    }
                }
            }
        }
        
        if result.len() == m.len() as usize {
            result.into_iter().collect()
        } else {
            String::new()
        }
    }
}