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
mod validator;
use validator::CreditCard;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <credit_card_number>", args[0]);
        std::process::exit(1);
    }
    let card = CreditCard::new(&args[1]);
    println!("{}", card);
    println!("Is valid card? {}", card.is_valid());

}
