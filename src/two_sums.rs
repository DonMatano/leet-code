#[cfg(test)]
mod tests {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (curr_index, num) in nums.iter().enumerate() {
            let val = target - num;
            match map.get(&val) {
                Some(stored_index) => {
                    return vec![*stored_index, curr_index.try_into().unwrap()]
                },
                None => map.insert(*num, curr_index.try_into().unwrap()),
            };
        }
        return vec![];
    }
    #[test]
    fn test1() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(vec![0, 1], two_sum(nums, 9));
    }
    #[test]
    fn test2() {
        let nums = vec![3,2,4];
        assert_eq!(vec![1,2], two_sum(nums, 6));
    }
    #[test]
    fn test3() {
        let nums = vec![3,3];
        assert_eq!(vec![0,1], two_sum(nums, 6));
    }
}
