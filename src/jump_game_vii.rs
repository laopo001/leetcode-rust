struct Solution;

fn run(arr: &[u8], min_jump: usize, max_jump: usize, start: usize, map: &mut Vec<i32>) -> bool {
    if (map[start] != -1) {
        return if map[start] == 0 { false } else { true };
    }
    let mut end2 = start + max_jump;
    let start2 = start + min_jump;

    if (start2 >= arr.len()) {
        return false;
    }
    if (end2 >= arr.len()) {
        end2 = arr.len() - 1;
    }

    let mut x = false;
    for i in (start2..=end2).rev() {
        if (arr[i] == 0) {
            if (i == arr.len() - 1) {
                return true;
            }
            x = x || run(arr, min_jump, max_jump, i, map);
        } else {
            if (i == arr.len() - 1) {
                return false;
            }
        }
    }
    if (x) {
        map[start] = 1;
    } else {
        map[start] = 0;
    }
    return x;
}

// impl Solution {
//     // Time Limit Exceeded
//     pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
//         let arr: Vec<u8> = s.as_bytes().iter().map(|x| return *x - '0' as u8).collect();
//         if arr[arr.len() - 1] == 1 {
//             return false;
//         }
//         let mut c = 0;
//         let queue = vec![0];
//         for i in arr.clone() {
//             if i == 1 {
//                 c += 1;
//             } else {
//                 if c > 0 && c > max_jump {
//                     return false;
//                 }
//                 c = 0;
//             }
//         }
//         return run(
//             arr.as_slice(),
//             min_jump as usize,
//             max_jump as usize,
//             0,
//             &mut vec![-1; arr.len()],
//         );
//     }
// }

impl Solution {
    // 4 ms, faster than 100.00% 
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let arr: Vec<u8> = s.as_bytes().iter().map(|x| return *x - '0' as u8).collect();
        let mut f = vec![0; arr.len()];
        let mut s = vec![0; arr.len()];
        f[0] = 1;
        s[0] = 1;
        for i in 1..arr.len() {
            if (arr[i] == 0 && i as i32 - min_jump >= 0) {
                let l = std::cmp::max(0, i as i32 - max_jump) as usize;
                let r = i - min_jump as usize;
                if l == 0 || (s[r] > s[l - 1]) {
                    f[i] = 1;
                }
            }
            s[i] = s[i - 1] + f[i];
        }
        return f[arr.len() - 1] == 1;
    }
}

#[test]
fn test() {
    let res = Solution::can_reach("00".to_string(), 1, 1);
    dbg!(res);
}
/*
    var canReach = function(s, minJump, maxJump) {
        if (s[s.length - 1] === '1') return false;
        const q = [0];
        const visited = new Set();
        while(q.length > 0) {
            const p = q.shift();
            if (p === s.length - 1) return true;
            if (visited.has(p)) continue;
            while(q[q.length - 1] < p - 2 * minJump) {
                q.pop();
            }
            for (let i = p + minJump; i <= Math.min(p + maxJump, s.length - 1); i++) {
                if (s[i] !== '1') {
                    q.unshift(i);
                }
            }
        }
        return false;
    };
*/
