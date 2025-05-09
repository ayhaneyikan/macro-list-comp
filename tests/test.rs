#[cfg(test)]
pub mod compile_pass {
    use comp_macro::comp;

    #[test]
    fn simple_map() {
        let result: Vec<_> = comp![x for x in [1, 2, 3]].collect();
        assert_eq!(result, [1, 2, 3]);
    }

    #[test]
    fn map_arithmetic() {
        // addition
        let result: Vec<_> = comp![x + 2 for x in [-1, 2, 10]].collect();
        assert_eq!(result, [1, 4, 12]);
        // subtraction
        let result: Vec<_> = comp![x - (5 * 2) for x in [10, 2, 20]].collect();
        assert_eq!(result, [0, -8, 10]);
        // multiplication
        let result: Vec<_> = comp![6 * n for n in vec![2, -5, 0]].collect();
        assert_eq!(result, [12, -30, 0]);
        // division  // TODO frustrating to mix floats and integer types
        let result: Vec<_> = comp![100. / n for n in vec![0.5, 10., 50.]].collect();
        assert_eq!(result, [200., 10., 2.]);
    }

    #[test]
    fn complex_pattern_matching() {
        // tuple
        let result: Vec<_> = comp![x * y for (x, y) in vec![(1, 0), (2, 4), (10, -3)]].collect();
        assert_eq!(result, vec![0, 8, -30]);
        // struct
        struct TestVal {
            x: u32,
            y: String,
            z: Option<u32>,
        }
        impl TestVal {
            fn new(x: u32, y: String, z: Option<u32>) -> Self {
                Self { x, y, z }
            }
        }
        let vals = vec![
            TestVal::new(1, "40".to_string(), None),
            TestVal::new(3, "13".to_string(), Some(17)),
        ];
        let result: Vec<_> = comp![x + y.parse::<u32>().unwrap() + z.unwrap_or(0) for TestVal { x, y, z } in vals].collect();
        assert_eq!(result, vec![41, 33]);
    }

    #[test]
    fn comprehend_strings() {
        let strings = vec!["file1".to_string(), "my_name".to_string()];
        // concat
        let result: Vec<_> = comp![String::new() + s + "_suffix" for s in &strings].collect();
        assert_eq!(result, vec!["file1_suffix".to_string(), "my_name_suffix".to_string()]);
        // len
        let result: Vec<_> = comp![s.len() for s in &strings].collect();
        assert_eq!(result, vec![strings[0].len(), strings[1].len()]);
        // both
        let result: Vec<_> = comp![(String::new() + s + "_suffix").len() for s in &strings].collect();
        let suff_len = "_suffix".len();
        assert_eq!(result, vec![strings[0].len() + suff_len, strings[1].len() + suff_len]);
    }

    #[test]
    fn simple_filter() {
        let result: Vec<_> = comp![x for x in [0, 1, 2, 3, 4] if x % 2 == 0].collect();
        assert_eq!(result, [0, 2, 4]);
    }

    #[test]
    fn map_and_filter() {
        // square even
        let result: Vec<_> = comp![x * x for x in [0, 1, 2, 3, 4] if x % 2 == 0].collect();
        assert_eq!(result, [0, 4, 16]);

        // complex match and filter
        struct TestVal {
            x: u32,
            y: String,
            z: Option<i16>,
        }
        impl TestVal {
            fn new(x: u32, y: String, z: Option<i16>) -> Self {
                Self { x, y, z }
            }
        }
        let vals = vec![
            TestVal::new(1, "40".to_string(), None),
            TestVal::new(3, "13".to_string(), Some(-2)),
            TestVal::new(10, "2".to_string(), Some(5)),
        ];
        let result: Vec<_> = comp![x + y.parse::<u32>().unwrap() + z.unwrap() as u32 for TestVal { x, y, z } in vals if z.unwrap_or(-1) >= 0].collect();
        assert_eq!(result, vec![17]);

        // counting vals using comprehension
        assert_eq!(comp![x for x in vec![true, false, true, true, true, false] if x].count(), 4);
    }
}