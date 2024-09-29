fn plus_one(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();

    for i in (0..len).rev() {
        if nums[i] < 9 {
            nums[i] += 1;
            return num;
        }

        nums[i] = 0;
    }

    let mut result = vec![1];
    result.extend(nums)

    result
}
