struct Solution;
impl Solution {
    // 0 ms, faster than 100.00% 
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        if (bits[bits.len() - 1] == 1) {
            return false;
        } else {
            let mut i = 0;
            while (i < (bits.len() + 1)) {
                if (i == bits.len()) {
                    return false;
                }
                if (bits[i] == 0) {
                    if (i == bits.len() - 1) {
                        break;
                    }
                    i += 1;
                    continue;
                } else if (bits[i] == 1) {
                    if (bits[i + 1] == 1) {
                        i += 2;
                        continue;
                    } else if (bits[i + 1] == 0) {
                        i += 2;
                        continue;
                    }
                }
                i += 1;
            }
            return true;
        }
    }
}
