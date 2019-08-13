use crate::base::Solution;

impl Solution {
    // 0ms 100%
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut res = std::i32::MAX;
        let mut index = 0;
        if triangle.len() == 0 {
            return 0;
        }
        let len = triangle.last().unwrap().len();
        let val =  triangle.first().unwrap()[0];
        let mut dp = vec![0;len];
        dp[0]= val;
         // println!("{:?}",dp);
        for i in 1..triangle.len() {
            let clone_dp = dp.clone();
            for j in 0..triangle[i].len(){
               
                if j > 0 && j < triangle[i].len() - 1 {
                    dp[j] = std::cmp::min( triangle[i][j] + clone_dp[j], triangle[i][j] + clone_dp[j - 1]);
                } else if j == 0 {
                    dp[j] =  triangle[i][j] + clone_dp[j];
                } else if j == triangle[i].len() - 1 {
                    dp[j] =  triangle[i][j] + clone_dp[j - 1];
                }     
            }
            // println!("{:?}",dp);
        }
        for i in dp {
            if res > i {
                res = i;
            }
        }
        res
    }
}