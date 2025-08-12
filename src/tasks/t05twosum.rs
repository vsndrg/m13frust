// Stupid approach O(n^2)
// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     for i in 0..nums.len() {
//         for j in 0..nums.len() {
//             if i != j && nums[i] + nums[j] == target {
//                 return vec![i as i32, j as i32];
//             }
//         }
//     }
//     vec![]
// }

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut inds: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let comp = target - nums[i];

        if let Some(ind) = inds.get(&comp) {
            return vec![*ind, i as i32];
        }
        inds.insert(num, i as i32);
    }
    vec![]
}

