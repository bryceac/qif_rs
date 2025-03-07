mod qif_type;
mod transaction;
mod transaction_status;
mod split;
mod errors;
mod date_format;
mod section;
mod qif;

// expose structures and enums for simple usage
pub use qif_type::QIFType as Type;
pub use transaction::Transaction as Transaction;
pub use transaction_status::TransactionStatus as TransactionStatus;
pub use split::Split as Split;
pub use section::Section as Section;
pub use date_format::DateFormat as DateFormat;
pub use qif::QIF as QIF;

#[cfg(test)]
mod tests {
    use crate::errors::TransactionBuildingError;

    use super::*;
    use chrono::prelude::*;

    #[test]
    fn qif_type_outputs_to_correct_string() {
        let expected = "Bank";

        assert_eq!(Type::Bank.to_str(), expected);
    }

    #[test]
    fn qif_type_parses_correctly() {
        let expected = Type::Bank;

        assert_eq!(Type::from("Bank").unwrap(), expected);
    }

    #[test]
    fn invalid_type_results_in_none() {
        assert!(Type::from("Car").is_none());
    }

    #[test]
    fn transaction_status_outputs_correctly() {
        let expected = "*";

        assert_eq!(TransactionStatus::Reconciled.to_str(), expected);
    }

    #[test]
    fn transaction_status_parses_correctly() {
        let expected = TransactionStatus::Cleared;

        assert_eq!(TransactionStatus::from("X").unwrap(), expected);
    }

    #[test]
    fn invalid_transaction_status_results_in_none() {
        assert!(TransactionStatus::from("a").is_none());
    }

    #[test]
    fn create_transaction() {
        let today = Local::now();

        let format = DateFormat::MonthDayFullYear;

        let transaction = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .build();

        assert!(transaction.is_ok());
    }

    #[test]
    fn transaction_creation_fails_if_date_format_is_wrong() {
        let today = Local::now();

        let format = DateFormat::MonthDayFullYear;

        let transaction = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .build();

        if let Err(error) = transaction {
            assert_eq!(error, TransactionBuildingError::NoDate)
        }
    }

    #[test]
    fn transaction_creation_fails_if_amount_is_empty() {
        let today = Local::now();

        let format = DateFormat::MonthDayFullYear;

        let transaction = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_memo("Open Account")
        .set_status("*")
        .build();

        if let Err(error) = transaction {
            assert_eq!(error, TransactionBuildingError::NoAmount)
        }
    }

    #[test]
    fn transaction_creation_fails_if_vendor_is_empty() {
        let today = Local::now();

        let format = DateFormat::MonthDayFullYear;

        let transaction = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .build();

        if let Err(error) = transaction {
            assert_eq!(error, TransactionBuildingError::NoVendor)
        }
    }

    #[test]
    fn transaction_string_outputs_in_qif_format() {
        let today = Local::now();
        let format = DateFormat::MonthDayFullYear;

        let expected = format!("D{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\n^", 
        today.format("%m/%d/%Y").to_string(),
        500.0,
        "*",
        1260,
        "Sam Hill Credit Union",
        "Open Account",
        "Sam Hill Credit Union",
        "Opening Balance");

        let transaction = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .build();

        if let Ok(transaction) = transaction {
            assert_eq!(transaction.to_string(), expected);
        }
    }

    #[test]
    fn parse_transaction_from_string() {
        let today = Local::now();
        let format = DateFormat::MonthDayFullYear;

        if let Ok(expected_transaction) = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .build() {
            let text = format!("D{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\n^", 
                today.format(format.chrono_str()).to_string(),
                500.0,
                "*",
                1260,
                "Sam Hill Credit Union",
                "Open Account",
                "Sam Hill Credit Union",
                "Opening Balance");

                if let Ok(transaction) = Transaction::from_str(&text) {
                    assert_eq!(expected_transaction, transaction)
                } 
        }
    }

    #[test]
    fn create_split() {
        let split = Split::builder()
        .set_category("Opening Balance")
        .set_memo("Bonus for new Account")
        .set_amount(50.0)
        .build();

        assert!(split.is_some())
    }

    #[test]
    fn split_creation_fails_without_amount() {
        let split = Split::builder()
        .set_category("Opening Balance")
        .set_memo("Bonus for new Account")
        .build();

        assert!(split.is_none())
    }

    #[test]
    fn parse_transaction_with_explicit_split() {
        let today = Local::now();
        let format = DateFormat::MonthDayFullYear;

        let expected_split = Split::builder()
        .set_category("Opening Balance")
        .set_memo("Bonus for new Account")
        .set_amount(50.0)
        .build().unwrap();

        let expected_transaction = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .add_split(expected_split)
        .build().unwrap();

        let text = format!("D{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\nS{}\r\nE{}\r\n${}\r\n^",
        today.format(format.chrono_str()).to_string(),
        500.0,
        "*",
        1260,
        "Sam Hill Credit Union",
        "Open Account",
        "Sam Hill Credit Union",
        "Opening Balance",
        "Opening Balance",
        "Bonus for new Account",
        50.0);

        if let Ok(transaction) = Transaction::from_str(&text) {
            assert_eq!(transaction, expected_transaction)
        }

    }

    #[test]
    fn parse_transaction_with_percentage_split() {
        let today = Local::now();
        let format = DateFormat::MonthDayFullYear;

        let expected_split = Split::builder()
        .set_category("Opening Balance")
        .set_memo("Bonus for new Account")
        .set_amount(50.0)
        .build().unwrap();

        let expected_transaction = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .add_split(expected_split)
        .build().unwrap();

        let text = format!("D{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\nS{}\r\nE{}\r\n%{}\r\n^",
        today.format(format.chrono_str()).to_string(),
        500.0,
        "*",
        1260,
        "Sam Hill Credit Union",
        "Open Account",
        "Sam Hill Credit Union",
        "Opening Balance",
        "Opening Balance",
        "Bonus for new Account",
        10);

        if let Ok(transaction) = Transaction::from_str(&text) {
            assert_eq!(transaction, expected_transaction)
        }

    }

    #[test]
    fn transaction_with_split_string_outputs_correctly() {
        let today = Local::now();
        let format = DateFormat::MonthDayFullYear;

        let expected = format!("D{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\nS{}\r\nE{}\r\n${:.2}\r\n^",
        today.format(format.chrono_str()).to_string(),
        500.0,
        "*",
        1260,
        "Sam Hill Credit Union",
        "Open Account",
        "Sam Hill Credit Union",
        "Opening Balance",
        "Opening Balance",
        "Bonus for new Account",
        50.0);

        let split = Split::builder()
        .set_category("Opening Balance")
        .set_memo("Bonus for new Account")
        .set_amount(50.0)
        .build().unwrap();

        let transaction = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .add_split(split)
        .build().unwrap();

        assert_eq!(expected, transaction.to_string())
    }

    #[test]
    fn parse_transaction_with_multiple_splits() {
        let today = Local::now();
        let format = DateFormat::MonthDayFullYear;

        let initial_expected_split = Split::builder()
        .set_category("Opening Balance")
        .set_memo("Initial Deposit")
        .set_amount(450.0)
        .build().unwrap();

        let bonus_expected_split = Split::builder()
        .set_category("Opening Balance")
        .set_memo("Bonus for new Account")
        .set_amount(50.0)
        .build().unwrap();

        let expected_transaction = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .add_split(initial_expected_split)
        .add_split(bonus_expected_split)
        .build().unwrap();

        let text = format!("D{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\nS{}\r\nE{}\r\n${}\r\nS{}\r\nE{}\r\n${}\r\n^",
        today.format(format.chrono_str()).to_string(),
        500.0,
        "*",
        1260,
        "Sam Hill Credit Union",
        "Open Account",
        "Sam Hill Credit Union",
        "Opening Balance",
        "Opening Balance",
        "Initial Deposit",
        450.0,
        "Opening Balance",
        "Bonus for new Account",
        50.0);

        if let Ok(transaction) = Transaction::from_str(&text) {
            assert_eq!(transaction, expected_transaction)
        }

    }

    #[test]
    fn transaction_with_multiple_splits_writes_proper_string() {
        let today = Local::now();
        let format = DateFormat::MonthDayFullYear;

        let expected = format!("D{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\nS{}\r\nE{}\r\n${:.2}\r\nS{}\r\nE{}\r\n${:.2}\r\n^",
        today.format(format.chrono_str()).to_string(),
        500.0,
        "*",
        1260,
        "Sam Hill Credit Union",
        "Open Account",
        "Sam Hill Credit Union",
        "Opening Balance",
        "Opening Balance",
        "Initial Deposit",
        450.0,
        "Opening Balance",
        "Bonus for new Account",
        50.0);

        let initial_split = Split::builder()
        .set_category("Opening Balance")
        .set_memo("Initial Deposit")
        .set_amount(450.0)
        .build().unwrap();

        let bonus_split = Split::builder()
        .set_category("Opening Balance")
        .set_memo("Bonus for new Account")
        .set_amount(50.0)
        .build().unwrap();

        let transaction = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .add_split(initial_split)
        .add_split(bonus_split)
        .build().unwrap();

        assert_eq!(expected, transaction.to_string())
    }

    #[test]
    fn parse_section() {
        let today = Local::now();
        let format = DateFormat::MonthDayFullYear;

        let text = format!("!Type:{}\r\nD{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\n^",
        "Bank",
        today.format(format.chrono_str()).to_string(),
        500.0,
        "*",
        1260,
        "Sam Hill Credit Union",
        "Open Account",
        "Sam Hill Credit Union",
        "Opening Balance");

        if let Ok(expected_transaction) = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .build() {
            if let Some(expected_section) = Section::builder()
            .set_type("Bank")
            .add_transaction(expected_transaction)
            .build() {
                if let Some(section) = Section::from_str(&text) {
                    assert_eq!(expected_section, section)
                }
            }
        }
    }

    #[test]
    fn qif_section_output_strng_correctly() {
        let today = Local::now();
        let format = DateFormat::MonthDayFullYear;

        let expected_text = format!("!Type:{}\r\nD{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\n^\r\n\r\n",
        "Bank",
        today.format(format.chrono_str()).to_string(),
        500.0,
        "*",
        1260,
        "Sam Hill Credit Union",
        "Open Account",
        "Sam Hill Credit Union",
        "Opening Balance");

        if let Ok(transaction) = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .build() {
            if let Some(section) = Section::builder()
            .set_type("Bank")
            .add_transaction(transaction)
            .build() {
                assert_eq!(expected_text, section.to_string())
            }
        }
    }

    #[test]
    fn parse_qif() {
        let today = Local::now();
        let format = DateFormat::MonthDayFullYear;

        let sam_hill = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .build().unwrap();

        let fake_street = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_vendor("Fake Street Electronics")
        .set_category("Gifts")
        .set_amount(-200.0)
        .set_memo("Headset")
        .build().unwrap();

        let velociraptor_entertainment = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_vendor("Velociraptor Entertainent")
        .set_amount(50000.0)
        .set_memo("Pay Day")
        .build().unwrap();

        let expected_section = Section::builder()
        .set_type("Bank")
        .add_transaction(sam_hill)
        .add_transaction(fake_street)
        .add_transaction(velociraptor_entertainment)
        .build().unwrap();

        let text = format!("!Type:{}\r\nD{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\n^\r\n\r\nD{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\n^\r\n\r\nD{}\r\nT{:.2}\r\nC{}\r\nN{}\r\nP{}\r\nM{}\r\nA{}\r\nL{}\r\n^\r\n\r\n",
        "Bank",
        today.format(format.chrono_str()).to_string(),
        500.0,
        "*",
        1260,
        "Sam Hill Credit Union",
        "Open Account",
        "Sam Hill Credit Union",
        "Opening Balance",
        today.format(format.chrono_str()).to_string(),
        -200.0,
        "*",
        0,
        "Fake Street Electronics",
        "Headset",
        "Fake Street Electronics",
        "Gifts",
        today.format(format.chrono_str()).to_string(),
        50000.0,
        "*",
        0,
        "Velociraptor Entertainment",
        "Pay Day",
        "Velociraptor Entertainment",
        "");

        let qif = QIF::from_str(&text);

        if let Some(bank) = qif.bank {
            assert_eq!(bank, expected_section)
        }
    }

    #[test]
    fn write_file() {
        let today = Local::now();
        let format = DateFormat::MonthDayFullYear;

        let sam_hill = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_check_number(1260)
        .set_vendor("Sam Hill Credit Union")
        .set_address("Sam Hill Credit Union")
        .set_category("Opening Balance")
        .set_amount(500.0)
        .set_memo("Open Account")
        .set_status("*")
        .build().unwrap();

        let fake_street = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_vendor("Fake Street Electronics")
        .set_category("Gifts")
        .set_amount(-200.0)
        .set_memo("Headset")
        .build().unwrap();

        let velociraptor_entertainment = Transaction::builder()
        .set_date(&today.format(format.chrono_str()).to_string(), &format)
        .set_vendor("Velociraptor Entertainent")
        .set_amount(50000.0)
        .set_memo("Pay Day")
        .build().unwrap();

        let bank_section = Section::builder()
        .set_type("Bank")
        .add_transaction(sam_hill)
        .add_transaction(fake_street)
        .add_transaction(velociraptor_entertainment)
        .build().unwrap();

        let qif = QIF::builder()
        .set_field(bank_section)
        .build();

        assert!(qif.save("test.qif").is_ok())
    }
}
