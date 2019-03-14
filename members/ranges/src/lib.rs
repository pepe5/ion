extern crate small;

mod index;
mod parse;
mod range;
mod select;

pub use self::{index::*, parse::*, range::*, select::*};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges() {
        let range1 = Range::exclusive(Index::new(1), Index::new(5));
        assert_eq!(Some((1, 4)), range1.bounds(42));
        assert_eq!(Some((1, 4)), range1.bounds(7));
        let range2 = Range::inclusive(Index::new(2), Index::new(-4));
        assert_eq!(Some((2, 5)), range2.bounds(10));
        assert_eq!(None, range2.bounds(3));
    }

    #[test]
    fn index_ranges() {
        let valid_cases = vec![
            (Range::exclusive(Index::Forward(0), Index::Forward(3)), "0..3"),
            (Range::inclusive(Index::Forward(0), Index::Forward(2)), "0...2"),
            (Range::inclusive(Index::Forward(0), Index::Forward(4)), "0..=4"),
            (Range::inclusive(Index::Forward(2), Index::Backward(1)), "2...-2"),
            (Range::inclusive(Index::Forward(0), Index::Backward(0)), "0...-1"),
            (Range::exclusive(Index::Backward(2), Index::Backward(0)), "-3..-1"),
            (Range::from(Index::Backward(2)), "-3.."),
            (Range::to(Index::Forward(5)), "..5"),
        ];

        for (range, string) in valid_cases {
            assert_eq!(Some(range), parse_index_range(string));
        }

        let invalid_cases = vec!["0..A", "3-3..42", "0.=3", "0=..3", "0.=.3"];

        for range in invalid_cases {
            assert_eq!(None, parse_index_range(range))
        }
    }

    fn test_range<T: Iterator<Item = i8>>(range: &str, expected: T) {
        let actual: Vec<small::String> = parse_range(range).unwrap().collect();
        let expected: Vec<_> =
            expected.map(|i| small::String::from_string(i.to_string())).collect();

        assert_eq!(actual, expected);
    }

    fn test_fixed_range<T: Iterator<Item = i8>>(range: &str, expected: T, digits: usize) {
        let actual: Vec<small::String> = parse_range(range).unwrap().collect();
        let expected: Vec<_> =
            expected.map(|i| small::String::from_string(format!("{:01$}", i, digits))).collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn range_expand() {
        if parse_range("abc").is_some() {
            panic!("parse_range() failed");
        }

        test_range("-3...3", -3..=3);
        test_fixed_range("07...12", 7..=12, 2);
        test_range("-3...10", -3..=10);
        test_range("3...-3", (-3..=3).rev());
        test_fixed_range("03...-3", (-3..=3).rev(), 2);
        test_fixed_range("3...-03", (-3..=3).rev(), 3);
        test_fixed_range("3..=-03", (-3..=3).rev(), 3);
        test_range("-3..4", -3..4);
        test_range("3..-4", (-3..4).rev());
        test_range("-3...0", -3..=0);
        test_range("-3..0", -3..0);

        let actual_inclusive: &[Vec<small::String>] = &[
            parse_range("a...c").unwrap().collect(),
            parse_range("c...a").unwrap().collect(),
            parse_range("A...C").unwrap().collect(),
            parse_range("C...A").unwrap().collect(),
            parse_range("C..=A").unwrap().collect(),
        ];
        let actual_exclusive: &[Vec<small::String>] = &[
            parse_range("C..A").unwrap().collect(),
            parse_range("c..a").unwrap().collect(),
        ];

        let expected_inclusive = &[
            ["a", "b", "c"],
            ["c", "b", "a"],
            ["A", "B", "C"],
            ["C", "B", "A"],
            ["C", "B", "A"], // This is a duplicate on purpose
        ];
        let expected_exclusive = &[
            ["C", "B"],
            ["c", "b"],
        ];

        for (actual, expected) in actual_inclusive.iter().zip(expected_inclusive.iter()) {
            assert_eq!(actual, expected);
        }
        for (actual, expected) in actual_exclusive.iter().zip(expected_exclusive.iter()) {
            assert_eq!(actual, expected);
        }
    }
}
