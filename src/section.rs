use std::fmt;

use crate::{Type, Transaction};

/** 
 * structure that houses the type and transactions in a QIF file 
*/
#[derive(Clone, Debug)]
pub struct Section {
    pub qif_type: Type,
    pub transactions: Vec<Transaction>
}

impl Section {
    pub fn builder() -> SectionBuilder {
        SectionBuilder::new()
    }

    pub fn to_string(&self) -> String {
        let mut content = format!("!Type:{}\r\n", self.qif_type.to_str());

        for transaction in self.transactions.clone() {
            let transaction_string = format!("{}\r\n\r\n", transaction.to_string());
            content.push_str(&transaction_string);
        }

        content
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let mut builder = Section::builder();
        
        builder.set_type(&extract_type_from(s));
        

        if let Ok(transaction) = Transaction::from_str(s) {
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

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Section {
    fn eq(&self, other: &Self) -> bool {
        self.qif_type == other.qif_type &&
        self.transactions == other.transactions
    }
}

fn retrieve_first_line(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    
    if !lines.is_empty() {
        lines[0].to_owned()
    } else {
        String::default()
    }
}

fn extract_type_from(s: &str) -> String {
    let first_line = retrieve_first_line(s);

    if first_line.starts_with("!Type:") {
        let pieces: Vec<String> = first_line.split(":").map(|s| s.to_owned()).collect();
    
        pieces[1].clone()
    } else {
        String::default()
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