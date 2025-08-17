// fn honest_coach(n: usize, skills: &mut [i32]) {
//     skills.sort_unstable();
//
//     let mut min_diff = skills[1] - skills[0];
//     for i in 0..n - 1 {
//         min_diff = std::cmp::min(min_diff, skills[i + 1] - skills[i]);
//     }
//
//     println!("{}", min_diff);
// }
//
