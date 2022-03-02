pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    'l1: for i1 in 0..(nums.len() - 1) {
        for i2 in (i1 + 1)..(nums.len()) {
            let v1: &i32 = &nums[i1];
            let v2: &i32 = &nums[i2];
            if v1 + v2 == target {
                result = vec![i1 as i32, i2 as i32];
                break 'l1;
            }
        }
    }
    result
}
