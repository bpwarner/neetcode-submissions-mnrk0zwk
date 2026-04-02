impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let unique_nums: HashSet::<_> = nums.iter().collect();
        unique_nums.len() != nums.len()
    }
}
