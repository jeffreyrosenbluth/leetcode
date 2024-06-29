use std::collections::HashMap;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let dict: HashMap<i32, u32> = HashMap::new();
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        remove_duplicates(&mut nums);
        dbg!(nums);
        // assert_eq!(nums, vec![1, 1, 2, 2]);
    }
}
