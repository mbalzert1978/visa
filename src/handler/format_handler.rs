use super::{into_next, CreditCard, Handler};

#[derive(Default)]
pub struct FormatHandler {
    next: Option<Box<dyn Handler>>,
}

impl FormatHandler {
    pub fn new(next: impl Handler + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}
impl Handler for FormatHandler {
    fn handle(&mut self, card: &mut CreditCard) {
        let card_number = card
            .number
            .chars()
            .filter(|&c| !c.is_whitespace())
            .collect::<Vec<_>>();

        let is_valid = is_valid_length(&card.get_number_blocks(), 4)
            && is_valid_length(&card_number, 16)
            && is_digits(card_number.iter().cloned());

        card.set_format(is_valid);
    }

    fn next(&mut self) -> &mut Option<Box<dyn Handler>> {
        &mut self.next
    }
}

fn is_valid_length<T>(value: &[T], count: usize) -> bool {
    value.len() == count
}

fn is_digits<I>(mut value: I) -> bool
where
    I: Iterator<Item = char>,
{
    value.all(|c| c.is_ascii_digit())
}
