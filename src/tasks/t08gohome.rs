// Works slow: TLE at test 74.
// pub fn going_home(n: usize, arr: Vec<i32>) {
//     let mut sums: std::collections::HashMap<i32, (usize, usize)> = std::collections::HashMap::new();
//
//     for i in 0..n {
//         for j in i + 1..n {
//             let sum = arr[i] + arr[j];
//
//             if let Some(&(k, l)) = sums.get(&sum) {
//                 if i != k && i != l && j != k && j != l {
//                     println!("YES");
//                     println!("{} {} {} {}", i + 1, j + 1, k + 1, l + 1);
//                     return;
//                 }
//             }
//             else {
//                 sums.insert(sum, (i, j));
//             }
//         }
//     }
//
//     println!("NO");
// }
//

// pub fn going_home(n: usize, arr: Vec<i32>) {
//     let max_sum = 5_000_001;
//     let mut sums: Vec<Option<(usize, usize)>> = vec![None; max_sum];
//
//     for i in 0..n {
//         for j in i + 1..n {
//             let sum = arr[i] + arr[j];
//             let idx = sum as usize;
//
//             if let Some((k, l)) = sums[idx] {
//                 if i != k && i != l && j != k && j != l {
//                     println!("YES");
//                     println!("{} {} {} {}", i + 1, j + 1, k + 1, l + 1);
//                     return;
//                 }
//             }
//             else {
//                 sums[idx] = Some((i, j));
//             }
//         }
//     }
//
//     println!("NO");
// }
//
