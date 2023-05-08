#[cfg(test)]
mod tests {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        for char in s.chars() {
            let total_number_of_char = m.entry(char).or_insert(0);
            *total_number_of_char += 1;
        }

        for char in t.chars() {
            match m.get_mut(&char) {
                None => return false,
                Some(total_number_of_char) => {
                    *total_number_of_char -= 1;
                    if *total_number_of_char <= 0 {
                        m.remove(&char);
                    }
                }
            }
        }
        if !m.is_empty() {
            return false
        }
        return true;
    }
    pub struct TestTable {
        test_value: (String, String),
        expect: bool,
    }
    #[test]
    fn is_working_anagram() {
        let tests = [TestTable {
            test_value: ("anagram".to_string(), "nagaram".to_string()),
            expect: true,
        }, TestTable {
            test_value: ("cat".to_string(), "rat".to_string()),
            expect: false,
        },  TestTable {
            test_value: ("ab".to_string(), "a".to_string()),
            expect: false,
        }];
        
        for test in tests {
            assert_eq!(test.expect, is_anagram(test.test_value.0, test.test_value.1))
        }
    }
}
