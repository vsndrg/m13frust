// pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
//     let mut ns = nums;
//     let length = ns.len();
//     let mut closest_sum = ns[0] + ns[1] + ns[2];
//
//     ns.sort_unstable();
//
//     for i in 0..length - 1 {
//         let mut b = i + 1;
//         let mut e = length - 1;
//
//         while b < e {
//             let sum: i32 = ns[i] + ns[b] + ns[e];
//
//             if (target - sum).abs() < (target - closest_sum).abs() {
//                 closest_sum = sum;
//             }
//
//             if sum < target {
//                 b += 1;
//             }
//             else if sum > target {
//                 e -= 1;
//             }
//             else {
//                 return target;
//             }
//
//         }
//     }
//     closest_sum
// }
