// https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/

struct Solution;

impl Solution {

    pub fn find(parent: &mut Vec<usize>, element: usize) -> usize{
        if parent[element] == element {
            element
        } else {
            let root = Solution::find(parent, parent[element]);
            parent[element] = root;
            root
        }
    }

    pub fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, a: usize, b: usize){
        if rank[a] > rank[b] {
            parent[b] = a;
            rank[a] += rank[b];
        } else {
            parent[a] = b;
            rank[b] += rank[a];
        }

    }

    // O(ack(m)) time - O(n (+ log m?)) space - union find
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = edges.len();
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank: Vec<usize> = vec![1; n];
        let mut components = n as i32; 
        for edge in edges {
            let p_src = Solution::find(&mut parent, edge[0] as usize);
            let p_dst = Solution::find(&mut parent, edge[1] as usize);
            if p_src != p_dst {
                components -= 1;
                Solution::union(&mut parent, &mut rank, p_src, p_dst);
            }
        }

        components
    }
}