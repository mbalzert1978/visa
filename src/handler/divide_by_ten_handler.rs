use super::{into_next, CreditCard, Handler};

#[derive(Default)]
pub struct DivideByTen {
    next: Option<Box<dyn Handler>>,
}

impl DivideByTen {
    pub fn new(next: impl Handler + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}
impl Handler for DivideByTen {
    fn handle(&mut self, card: &mut CreditCard) {
        card.set_divide_ten(
            card.number
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_string())
                .flat_map(|c| c.parse::<u8>())
                .sum::<u8>()
                % 10
                == 0,
        );
    }
    fn next(&mut self) -> &mut Option<Box<dyn Handler>> {
        &mut self.next
    }
}
