impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ret = Vec::new();

        for (pos_a, a) in nums.iter().enumerate() {
            // println!("loop a: {:?} {:?}", pos_a, a);
            for (pos_b, b) in nums.iter().skip(pos_a + 1).enumerate() {
                // println!("loop b: {:?} {:?}", pos_b + pos_a + 1, b);
                if a + b == target {
                    ret.push(pos_a as i32);
                    ret.push(pos_b as i32 + pos_a as i32 + 1);
                    return ret;
                }
            }
        }

        ret
    }
}
