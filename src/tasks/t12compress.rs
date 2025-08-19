// pub fn compress_words(n: usize, words: Vec<String>) {
//     let mut ans: String = words[0].clone();
//
//     for word in words.iter().take(n).skip(1) {
//         let prev_word = &ans;
//         let cur_word = word;
//
//         let l_prev = prev_word.len();
//         let l_cur = cur_word.len();
//         let max_overlap = std::cmp::min(l_prev, l_cur);
//
//         let mut part = &cur_word[..];
//         for pos in (1..=max_overlap).rev() {
//             if cur_word[..pos] == prev_word[l_prev - pos..] {
//                 part = &cur_word[pos..];
//                 break;
//             }
//         }
//         ans += part;
//     }
//
//     println!("{}", ans);
// }
