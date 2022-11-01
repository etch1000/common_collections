use std::collections::HashMap;

fn median(nums: &mut [i32]) -> i32 {
    nums.sort();

    let mid = nums.len() / 2;

    nums[mid]
}

fn mode(nums: &[i32]) -> i32 {
    let mut occ = HashMap::new();

    for &val in nums {
        *occ.entry(val).or_insert(0) += 1;
    }

    occ.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

fn main() {
    let mut nums = [654, 364, 974, 126, 742, 379, 537, 537, 126, 126];

    println!("{}", median(&mut nums));

    println!("{}", mode(&nums));
}
