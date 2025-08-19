// pub fn bool_eval() {
//     let mut input: String = String::new();
//
//     std::io::stdin().read_line(&mut input).unwrap();
//     let [a, b]: [bool; 2] = input
//         .split_whitespace()
//         .map(|s| match s {"true" => true, "false" => false, _ => panic!("Invalid input")})
//         .collect::<Vec<_>>()
//         .try_into()
//         .unwrap();
//
//     println!("A && !B = {}", a && !b);
//     println!("A || B = {}", a || b);
//     println!("A -> B = {}", !a || b);
// }

