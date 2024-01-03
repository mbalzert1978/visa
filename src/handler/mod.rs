use crate::credit_card::CreditCard;

mod divide_by_ten_handler;
mod divide_by_two_handler;
mod format_handler;

pub use divide_by_ten_handler::DivideByTen;
pub use divide_by_two_handler::DivideByTwo;
pub use format_handler::FormatHandler;

pub trait Handler {
    fn execute(&mut self, card: &mut CreditCard) {
        self.handle(card);

        if let Some(next) = self.next() {
            next.execute(card);
        }
    }
    fn handle(&mut self, card: &mut CreditCard);
    fn next(&mut self) -> &mut Option<Box<dyn Handler>>;
}
fn into_next(handler: impl Handler + Sized + 'static) -> Option<Box<dyn Handler>> {
    Some(Box::new(handler))
}
