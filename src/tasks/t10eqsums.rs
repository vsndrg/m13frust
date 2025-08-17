// pub fn equal_sums(_n: i32, cases: Vec<(i32, Vec<i32>)>) {
//     let mut sums: std::collections::HashMap<i32, (i32, i32)> = std::collections::HashMap::new();
//
//     for (i, (_, arr)) in cases.iter().enumerate() {
//         let sum: i32 = arr.iter().sum();
//
//         for (y, a) in arr.iter().enumerate() {
//             let s = sum - a;
//
//             if let Some(&(k, x)) = sums.get(&s) {
//                 if i as i32 != k {
//                     println!("YES");
//                     println!("{} {}", i + 1, y + 1);
//                     println!("{} {}", k + 1, x + 1);
//                     return;
//                 }
//                 continue;
//             }
//
//             sums.insert(s, (i as i32, y as i32));
//         }
//     }
//
//     println!("NO");
// }
//
