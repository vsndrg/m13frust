pub fn reverse(numbers: Vec<i32>) -> Vec<i32> {
    let mut rev_numbers: Vec<i32> = Vec::with_capacity(numbers.len());

    for i in (0..numbers.len()).rev() {
        rev_numbers.push(numbers[i]);
    }

    rev_numbers
}

