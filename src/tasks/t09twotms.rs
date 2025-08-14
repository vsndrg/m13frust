pub fn two_teams_composing(n: i32, arr: Vec<i32>) {
    let _ = n;
    let mut skills: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let mut same = 0;

    use std::cmp::{max, min};

    for a in arr {
        let cnt = skills.entry(a).or_insert(0);
        *cnt += 1;
        same = max(same, *cnt);
    }
    let uniq = skills.len() as i32;


    println!("{}", max(min(uniq, same - 1), min(uniq - 1, same)));
}

