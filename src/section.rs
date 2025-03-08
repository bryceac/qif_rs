use std::fmt;
use regex::Regex;

use crate::{Type, Transaction, DateFormat};

/** 
 * structure that houses the type and transactions in a QIF file 
*/
#[derive(Clone, Debug)]
pub struct Section {
    pub qif_type: Type,
    pub transactions: Vec<Transaction>
}

impl Section {
    /**
     * builder method use to create a QIF section.
     * 
     * The creation is done like this:
     * 
     * let expected_section = Section::builder()
        .set_type("Bank")
        .add_transaction(sam_hill)
        .add_transaction(fake_street)
        .add_transaction(velociraptor_entertainment)
        .build().unwrap();
     */
    pub fn builder() -> SectionBuilder {
        SectionBuilder::new()
    }

    pub fn to_string(&self, df: &DateFormat) -> String {
        let mut content = format!("!Type:{}\r\n", self.qif_type.to_str());

        for transaction in self.transactions.clone() {
            let transaction_string = format!("{}\r\n\r\n", transaction.to_string(df));
            content.push_str(&transaction_string);
        }

        content
    }

    pub fn from_str(s: &str, df: &DateFormat) -> Option<Self> {
        let mut builder = Section::builder();
        
        builder.set_type(&extract_type(s));
        
        if let Ok(transaction) = Transaction::from_str(s, df) {
            builder.add_transaction(transaction);
        }

        builder.build()
    }

    pub fn add_transaction_if_not_exists(&mut self, transaction: &Transaction) {
        if let None = self.transactions.iter().find(|&e| e == transaction) {
            self.transactions.push(transaction.to_owned())
        }
    }
}

fn extract_type(s: &str) -> String {
    if let Ok(regex) = Regex::new("!Type:([A-Z|a-z]{4,9})") {
        if let Some(captures) = regex.captures(s) {
            let (_, [account_type]) = captures.extract();

            account_type.to_string()
        } else {
            String::default()
        }
    } else {
        String::default()
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string(&DateFormat::MonthDayFullYear))
    }
}

impl PartialEq for Section {
    fn eq(&self, other: &Self) -> bool {
        self.qif_type == other.qif_type &&
        self.transactions == other.transactions
    }
}

pub struct SectionBuilder {
    qif_type: Option<Type>,
    transactions: Vec<Transaction>
}

impl SectionBuilder {
    pub fn new() -> Self {
        SectionBuilder {
            qif_type: None,
            transactions: vec![]
        }
    }

    pub fn set_type(&mut self, t: &str) -> &mut Self {
        self.qif_type = Type::from(t);
        self
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> &mut Self {
        self.transactions.push(transaction);
        self
    }

    pub fn build(&self) -> Option<Section> {
        if let Some(qif_type) = self.qif_type.clone() {
            Some(Section { 
                qif_type, 
                transactions: self.transactions.clone() 
            })
        } else {
            None
        }
    }
}