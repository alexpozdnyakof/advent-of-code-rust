pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

pub fn chars_slice_to_str(slice: &[char]) -> String {
    slice.into_iter().collect()
}

// Additional common functions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }

    #[test]
    fn chars_to_slice_test() {
        let chars = vec!['m', 'u', 'l'];
        let result = chars_slice_to_str(&chars[0..3]);
        assert_eq!(result, "mul");
    }
}
