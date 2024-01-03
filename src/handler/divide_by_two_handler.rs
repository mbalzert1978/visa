use super::{into_next, CreditCard, Handler};

#[derive(Default)]
pub struct DivideByTwo {
    next: Option<Box<dyn Handler>>,
}

impl DivideByTwo {
    pub fn new(next: impl Handler + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}
impl Handler for DivideByTwo {
    fn handle(&mut self, card: &mut CreditCard) {
        card.set_divide_two(
            card.number
                .split_whitespace()
                .all(|block| block.parse::<u32>().map_or(false, |digits| digits % 2 == 0)),
        );
    }
    fn next(&mut self) -> &mut Option<Box<dyn Handler>> {
        &mut self.next
    }
}
