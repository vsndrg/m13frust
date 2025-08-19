mod tasks;

// fn read_num<T: std::str::FromStr>() -> T
// where
//     T::Err: std::fmt::Debug,
// {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     input.trim().parse::<T>().unwrap()
// }
//
// fn read_vec<T: std::str::FromStr>() -> Vec<T>
// where
//     T::Err: std::fmt::Debug,
// {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     let arr: Vec<T> = input
//         .split_whitespace()
//         .map(|s| s.parse::<T>().unwrap())
//         .collect();
//
//     arr
// }

fn main() {
    // 01
    // tasks::t01sum::sum();

    // 02
    // tasks::t02maxn::maxn();

    // 03
    // let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let rev_numbers = tasks::t03reverse::reverse(numbers);
    //
    // for &num in &rev_numbers {
    //     print!("{} ", num);
    // }
    // println!();

    // 04
    // let s: &str = "AEIOU aeiou";
    //
    // println!("{}", tasks::t04cntvow::count_vowels(s));

    // 05
    // let nums = vec![2, 7, 11, 15];
    // let trg = 9;
    //
    // println!("{:?}", tasks::t05twosum::two_sum(nums, trg));

    // 06
    // let nums = vec![-1, 0, 1, 2, -1, -4];
    // println!("{:?}", tasks::t06threesum::three_sum(nums));

    // 07
    // let nums = vec![-1, 2, 1, -4];
    // let target = 1;
    // println!("{}", tasks::t07threesumc::three_sum_closest(nums, target));

    // 08
    // let n = read_num();
    // let arr = read_vec();
    //
    // tasks::t08gohome::going_home(n, arr);

    // 09
    // for _ in 0..read_num() {
    //     tasks::t09twotms::two_teams_composing(read_num(), read_vec());
    // }

    // 10
    // let k = read_num();
    // let mut cases: Vec<(i32, Vec<i32>)> = Vec::new();
    // for _ in 0..k {
    //     cases.push((read_num(), read_vec()));
    // }
    // tasks::t10eqsums::equal_sums(k, cases);

    // 11
    // let t = read_num();
    // for _ in 0..t {
    //     honest_coach(read_num(), &mut read_vec());
    // }

    // 12
    // let n = read_num();
    // tasks::t12compress::compress_words(n, read_vec());

    // 13
    // tasks::t13booleval::bool_eval();

    // 14
    tasks::t14truthtab::run();
}

