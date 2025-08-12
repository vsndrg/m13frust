mod tasks;

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

    let nums = vec![2, 7, 11, 15];
    let trg = 9;

    println!("{:?}", tasks::t05twosum::two_sum(nums, trg));
}
