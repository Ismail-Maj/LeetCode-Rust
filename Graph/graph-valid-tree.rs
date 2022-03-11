// https://leetcode.com/problems/graph-valid-tree/

struct Solution;

impl Solution {
    pub fn find(parent: &mut Vec<usize>, element: usize) -> usize {
        if parent[element] == element {
            return element;
        } else {
            let root = Solution::find(parent, parent[element]);
            parent[element] = root;
            return root;
        }
    }

    pub fn union_by_depth(parent: &mut Vec<usize>, rank: &mut Vec<usize>, a: usize, b: usize) {
        if rank[a] > rank[b] {
            parent[b] = a;
        } else {
            parent[a] = b;
            if rank[a] == rank[b] {
                rank[b] += 1;
            }
        }
    }

    // O(ack(m)) time - O(n (+ log m?)) space - union find
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut parent : Vec<usize> = (0..n as usize).collect();
        let mut rank : Vec<usize> = vec![0; n as usize];
        let m = edges.len();
        for edge in edges {
            let p_src = Solution::find(&mut parent, edge[0] as usize);
            let p_dst = Solution::find(&mut parent, edge[1] as usize);
            if p_src == p_dst {
                return false;
            }
            Solution::union_by_depth(&mut parent, &mut rank, p_src, p_dst);
        }
        (m + 1) == n as usize
    }

    //optional
    pub fn union_by_size(parent: &mut Vec<usize>, rank: &mut Vec<usize>, a: usize, b: usize) {
        if rank[a] > rank[b] {
            parent[b] = a;
            rank[a] += rank[b];
        } else {
            parent[a] = b;
            rank[b] += rank[a];
        }
    }
}
