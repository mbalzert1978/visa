#[derive(Debug, Default)]
pub struct CreditCard {
    pub number: String,
    pub format: bool,
    pub by_ten: bool,
    pub by_two: bool,
}

impl CreditCard {
    pub fn new(number: &str) -> CreditCard {
        CreditCard {
            number: number.to_string(),
            format: false,
            by_ten: false,
            by_two: false,
        }
    }
    pub fn get_number_blocks(&self) -> Vec<&str> {
        self.number.split(' ').collect()
    }
    pub fn is_valid(&self) -> bool {
        self.by_ten && self.by_two && self.format
    }
    pub fn set_format(&mut self, format: bool) {
        self.format = format;
    }
    pub fn set_divide_ten(&mut self, divide_ten: bool) {
        self.by_ten = divide_ten;
    }
    pub fn set_divide_two(&mut self, divide_two: bool) {
        self.by_two = divide_two;
    }
}
