// pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
//     let mut found: Vec<Vec<i32>> = Vec::new();
//     let mut ns = nums;
//     let l = ns.len();
//
//     ns.sort_unstable();
//
//     for (i, &x) in ns.iter().enumerate() {
//         if i > 0 && x == ns[i - 1] {
//             continue;
//         }
//
//         let mut b = i + 1;
//         let mut e = l - 1;
//
//         while b < e {
//             let y = ns[b];
//             let z = ns[e];
//             let sum = x + y + z;
//
//             if sum < 0 {
//                 b += 1;
//             }
//             else if sum > 0 {
//                 e -= 1
//             }
//             else {
//                 found.push(vec![x, y, z]);
//                 b += 1;
//                 e -= 1;
//
//                 while b < e && ns[b] == y {
//                     b += 1;
//                 }
//                 while b < e && ns[e] == z {
//                     e -= 1;
//                 }
//             }
//         }
//     }
//
//     found
// }
//
