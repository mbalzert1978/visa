pub struct CreditCard {
    number: String,
}

impl CreditCard {
    pub fn new(number: &str) -> CreditCard {
        CreditCard {
            number: number.to_string(),
        }
    }
    pub fn is_valid(&self) -> bool {
        self.is_valid_format() && self.is_dividable_by_ten() && self.is_dividable_by_two()
    }
    fn is_dividable_by_ten(&self) -> bool {
        let total_sum: u32 = self
            .number
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .sum();

        total_sum % 10 == 0
    }
    fn is_dividable_by_two(&self) -> bool {
        let total_sum: u32 = self
            .number
            .split_whitespace()
            .flat_map(|block| block.chars().filter(|c| c.is_ascii_digit()))
            .map(|c| c.to_digit(10).unwrap())
            .sum();

        total_sum % 2 == 0
    }
    fn is_valid_format(&self) -> bool {
        let blocks = self
            .number
            .chars()
            .filter(|i| !i.is_whitespace())
            .collect::<Vec<char>>();
        if !is_valid_lenght(&blocks) {
            return false;
        }
        if !is_digits(&blocks) {
            return false;
        }
        if is_all_zero(&blocks) {
            return false;
        }
        true
    }
}

fn is_all_zero(value: &[char]) -> bool {
    if let Some(numbers) = parse_to_digit(value) {
        numbers.into_iter().all(|i| i == 0)
    } else {
        false
    }
}

fn is_digits(value: &[char]) -> bool {
    parse_to_digit(value).is_some()
}

fn parse_to_digit(value: &[char]) -> Option<Vec<u32>> {
    value
        .iter()
        .map(|i| i.to_digit(10))
        .collect::<Option<Vec<u32>>>()
}

fn is_valid_lenght(value: &Vec<char>) -> bool {
    value.len() == 16
}
