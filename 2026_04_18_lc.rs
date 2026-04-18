use std::collections::HashMap;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut seen: HashMap<i32, usize> = HashMap::new();
        let mut min_dist = usize::MAX;

        for (j, &num) in nums.iter().enumerate() {
            // Check if current number matches a previously reversed number
            if let Some(&i) = seen.get(&num) {
                min_dist = min_dist.min(j - i);
            }

            // Store reverse(nums[j]) for future matches
            let rev = Self::reverse(num);
            seen.insert(rev, j);
        }

        if min_dist == usize::MAX {
            -1
        } else {
            min_dist as i32
        }
    }

    fn reverse(mut num: i32) -> i32 {
        let mut rev = 0;
        while num > 0 {
            rev = rev * 10 + num % 10;
            num /= 10;
        }
        rev
    }
}
