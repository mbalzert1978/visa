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
        self.check_format() && self.check_sum10() && self.check_sum2()
    }
    pub fn check_sum10(&self) -> bool {
        let sum: u32 = self.number.chars().filter_map(|c| c.to_digit(10)).sum();
        sum % 10 == 0
    }
    fn check_sum2(&self) -> bool {
        let sum: u32 = self
            .number
            .split_whitespace()
            .flat_map(|s| s.chars().filter_map(|c| c.to_digit(10)))
            .sum();
        sum % 2 == 0
    }
    fn check_format(&self) -> bool {
        let blocks: Vec<&str> = self.number.split(' ').collect();
        let numbers = self.number.replace(' ', "");
        if !is_valid_length(&blocks, 4) {
            return false;
        }
        if !is_valid_length(&numbers, 16) {
            return false;
        }
        if !is_numeric(&numbers) {
            return false;
        }
        if !is_valid_numbers(&numbers) {
            return false;
        }
        true
    }
}

trait HasLength {
    fn len(&self) -> usize;
}

impl HasLength for String {
    fn len(&self) -> usize {
        self.len()
    }
}
impl HasLength for Vec<&str> {
    fn len(&self) -> usize {
        self.len()
    }
}
fn is_valid_length<T: HasLength>(value: &T, count: usize) -> bool {
    value.len() == count
}

fn is_numeric(value: &str) -> bool {
    value.chars().all(|c| c.is_ascii_digit())
}
fn is_valid_numbers(value: &str) -> bool {
    value.chars().filter_map(|c| c.to_digit(10)).any(|d| d > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_card_should_be_true() {
        let card = CreditCard::new("1234 5678 9012 3450");
        assert!(card.is_valid());
    }

    #[test]
    fn test_all_zero_should_be_false() {
        assert!(!is_valid_numbers("0000"));
    }
    #[test]
    fn test_check_sum10_should_be_true() {
        let card = CreditCard::new("37");
        assert!(card.check_sum10());
    }
    #[test]
    fn test_check_sum2_should_be_true() {
        let card = CreditCard::new("4 8");
        assert!(card.check_sum2());
    }
}
