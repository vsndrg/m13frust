mod tasks;

fn main() {
    // tasks::t01sum::sum();
    // tasks::t02maxn::maxn();

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let rev_numbers = tasks::t03reverse::reverse(numbers);

    for &num in &rev_numbers {
        print!("{} ", num);
    }
}
