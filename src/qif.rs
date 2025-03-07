use std::{fmt, io::{self, Read, Write }, fs::File};

use crate::{ Section, Type, Transaction };

/// A structure that represents a QIF document.
#[derive(Debug, PartialEq)]
pub struct QIF {
    pub cash: Option<Section>,
    pub bank: Option<Section>,
    pub credit_card: Option<Section>,
    pub liability: Option<Section>,
    pub asset: Option<Section>
}

impl QIF {
    pub fn builder() -> QIFBuilder {
        QIFBuilder::new()
    }

    pub fn to_string(&self) -> String {
        let mut content = String::default();

        if self.cash.is_some() {
            content.push_str(&self.field_to_string(Type::Cash));
        }
        
        if self.bank.is_some() {
            content.push_str(&self.field_to_string(Type::Bank));
        }

        if self.credit_card.is_some() {
            content.push_str(&self.field_to_string(Type::CreditCard));
        }
        
        if self.liability.is_some() {
            content.push_str(&self.field_to_string(Type::Liability));
        }
        
        if self.asset.is_some() {
            content.push_str(&self.field_to_string(Type::Asset));
        }

        content
    }

    pub fn from_str(s: &str) -> QIF {
        let mut builder = QIF::builder();

        let blocks: Vec<&str> = s.split("^").collect();

        let blocks_without_whitespace: Vec<String> = blocks.iter()
        .map(|&s| remove_whitespace(s)).collect();

        let mut current_section: Option<Section> = None;

        for block in blocks_without_whitespace {
            if let Some(section) = Section::from_str(&block) {
                if !builder.update_field(section.clone()) {
                    builder.set_field(section.clone());
                }

                current_section = match section.qif_type {
                    Type::Cash => builder.cash.clone(),
                    Type::Bank => builder.bank.clone(),
                    Type::CreditCard => builder.credit_card.clone(),
                    Type::Liability => builder.liability.clone(),
                    Type::Asset => builder.asset.clone(),
                };
            } else if let Ok(transaction) = Transaction::from_str(&block) {
                if let Some(mut current) = current_section.clone() {
                    current.add_transaction_if_not_exists(&transaction);

                    builder.update_field(current);
                }
            }
        }

        builder.build()
    }

    fn field_to_string(&self, field: Type) -> String {
        match  field {
            Type::Cash => if let Some(cash) = self.cash.clone() {
                cash.to_string()
            } else {
                String::default()
            },
            Type::Bank => if let Some(bank) = self.bank.clone() {
                bank.to_string()
            } else {
                String::default()
            },
            Type::CreditCard => if let Some(credit_card) = self.credit_card.clone() {
                credit_card.to_string()
            } else {
                String::default()
            },
            Type::Liability => if let Some(liability) = self.liability.clone() {
                liability.to_string()
            } else {
                String::default()
            },
            Type::Asset => if let Some(asset) = self.asset.clone() {
                asset.to_string()
            } else {
                String::default()
            },
        }
    }

    pub fn load_from_file(p: &str) -> Result<Self, String> {
        match file_contents_from(p) {
            Ok(content) => Ok(Self::from_str(&content)),
            Err(error) => Err(format!("{}", error))
        }
    }

    pub fn save(&self, p: &str) -> Result<(), io::Error> {
        let mut output = File::create(p)?;

        match write!(output, "{}", format!("{}", self.to_string())) {
            Ok(()) => Ok(()),
            Err(error) => Err(error)
        }
    }
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn file_contents_from(f: &str) -> Result<String, io::Error> {
    let mut file_contents = String::default();
    File::open(f)?.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}

impl fmt::Display for QIF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct QIFBuilder {
    pub cash: Option<Section>,
    pub bank: Option<Section>,
    pub credit_card: Option<Section>,
    pub liability: Option<Section>,
    pub asset: Option<Section>
}

impl QIFBuilder {
    pub fn new() -> Self {
        QIFBuilder { 
            cash: None, 
            bank: None, 
            credit_card: None, 
            liability: None, 
            asset: None 
        }
    }

    pub fn set_field(&mut self, section: Section) -> &mut Self {
        match section.qif_type {
            Type::Cash => self.cash = Some(section.clone()),
            Type::Bank => self.bank = Some(section.clone()),
            Type::CreditCard => self.credit_card = Some(section.clone()),
            Type::Liability => self.liability = Some(section.clone()),
            Type::Asset => self.asset = Some(section.clone())
        }
        self
    }

    pub fn build(&self) -> QIF {
        QIF { 
            cash: self.cash.clone(), 
            bank: self.bank.clone(), 
            credit_card: self.credit_card.clone(), 
            liability: self.liability.clone(), 
            asset: self.asset.clone() 
        }
    }

    pub fn update_field(&mut self, section: Section) -> bool {
        let mut was_updated = false;
        match section.qif_type {
            Type::Cash => {
                if let Some(mut cash) = self.cash.clone() {
                    for transaction in &section.transactions {
                       cash.add_transaction_if_not_exists(transaction) 
                    }

                    self.set_field(cash);
                    was_updated = true; 
                }
            },
            Type::Bank => {
                if let Some(mut bank) = self.bank.clone() {
                    for transaction in &section.transactions {
                       bank.add_transaction_if_not_exists(transaction) 
                    }

                    self.set_field(bank);
                    was_updated = true
                }
            },
            Type::CreditCard => {
                if let Some(mut credit_card) = self.credit_card.clone() {
                    for transaction in &section.transactions {
                       credit_card.add_transaction_if_not_exists(transaction) 
                    }

                    self.set_field(credit_card);
                    was_updated = true
                }
            },
            Type::Liability => {
                if let Some(mut liability) = self.liability.clone() {
                    for transaction in &section.transactions {
                        liability.add_transaction_if_not_exists(transaction) 
                    }

                    self.set_field(liability);
                    was_updated = true
                }
            },
            Type::Asset => {
                if let Some(mut asset) = self.asset.clone() {
                    for transaction in &section.transactions {
                       asset.add_transaction_if_not_exists(transaction) 
                    }

                    self.set_field(asset);
                    was_updated = true
                }
            },
        }
        return was_updated;
    }
}