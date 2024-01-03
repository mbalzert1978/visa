// Schreibe eine Klasse, welche überprüft ob eine bestimmte Kreditkartennummer zulässig ist.

// Zulässig ist eine Nummer, wenn sie folgende Bedingungen erfüllt:
// 1. Die Kreditkartennummer muss in dem Format "#### #### #### ####" (# = Zahl 0-9)
// 2. Die Summe aller Zahlen (#) muss teilbar durch 10 sein
// Bonus. Die Summe der einzelnen Zahlengruppen (####) muss teilbar durch 2 sein

// Beispiele:
// "1234 5678 9012 3450" -> Regel 1: check, Regel 2: check
// "1234 5678 9012 3455" -> Regel 1: check, Regel 2: nope
// "024 5678 92 3455"    -> Regel 1: nope, Regel 2: check
// "124 5678 92 3455"    -> Regel 1: nope, Regel 2: nope
// "0000 0000 0000 0000" -> Regel 1: check, Regel 2: nope
mod credit_card;
mod handler;
use credit_card::CreditCard;
use handler::{DivideByTen, DivideByTwo, FormatHandler, Handler};

fn main() {
    let divide_by_ten = DivideByTen::default();
    let divide_by_two = DivideByTwo::new(divide_by_ten);
    let mut format = FormatHandler::new(divide_by_two);

    let mut card = CreditCard::new("1234 5678 9012 3450");

    format.execute(&mut card);
    println!("the card is valid: {}", card.is_valid());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> FormatHandler {
        let divide_by_ten = DivideByTen::default();
        let divide_by_two = DivideByTwo::new(divide_by_ten);
        FormatHandler::new(divide_by_two)
    }

    #[test]
    fn valid_card_rule_1_and_2() {
        let mut chain = setup();
        let mut card = CreditCard::new("1234 5678 9012 3450");

        chain.execute(&mut card);
        assert!(card.is_valid());
    }

    #[test]
    fn valid_card_rule_1_but_not_2() {
        let mut chain = setup();
        let mut card = CreditCard::new("1234 5678 9012 3455");

        chain.execute(&mut card);
        assert!(card.format);
        assert!(!card.by_ten);
    }

    #[test]
    fn invalid_card_rule_1_but_valid_2() {
        let mut chain = setup();
        let mut card = CreditCard::new("024 5678 92 3455");

        chain.execute(&mut card);
        assert!(!card.format);
        assert!(card.by_ten);
    }

    #[test]
    fn invalid_card_neither_rule_1_nor_2() {
        let mut chain = setup();
        let mut card = CreditCard::new("124 5678 92 3455");

        chain.execute(&mut card);
        assert!(!card.format);
        assert!(!card.by_ten);
    }

    #[test]
    fn valid_card_rule_1_but_not_2_zeroes() {
        let mut chain = setup();
        let mut card = CreditCard::new("0000 0000 0000 0000");

        chain.execute(&mut card);
        assert!(card.format);
        assert!(!card.by_ten);
    }
}
