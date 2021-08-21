impl Solution {
    // bitmask iterative dfs strategy
    fn nqueens(n: isize) -> isize {
        let mut res = 0;

        let ones = (1<<n)-1 as isize;

        let mut stack: Vec<(isize, isize, isize)> = vec![(0, 0 ,0)];

        loop {
            match stack.pop(){
                Some((col, downward_diag, upward_diag)) => {
                    let mut valid_spots = ones & !(downward_diag | upward_diag | col);
                    while valid_spots != 0 {
                        let spot = -valid_spots & valid_spots; // get rightmost 1 bit
                        valid_spots = valid_spots ^ spot;
                        let col = col | spot;
                        if col == ones {
                            res+=1;
                        } else {
                            stack.push( (col, (downward_diag | spot) >> 1, (upward_diag | spot) << 1) );
                        }
                    }
                }
                _ => break
            }
        }
        res
    }
}
