use std::convert::From;

use rust_decimal::Decimal;

use crate::InputRecord;
#[derive(Debug)]
pub enum TransactionType {
    Deposit(self::Transaction),
    Withdrawal(self::Transaction),
    Dispute(self::Transaction),
    Resolve(self::Transaction),
    Chargeback(self::Transaction),
}

#[derive(Debug, Clone, Copy)]
pub struct Transaction {
    client_id: u16,
    transaction_id: u32,
    amount: Option<Decimal>,
}

impl Transaction {
    pub fn amount(&self) -> &Option<Decimal> {
        &self.amount
    }

    // pub fn client_id(&self) -> u16 {
    //     self.client_id
    // }

    pub fn transaction_id(&self) -> u32 {
        self.transaction_id
    }
}

impl From<InputRecord> for Transaction {
    fn from(rec: InputRecord) -> Self {
        Transaction {
            client_id: rec.client,
            transaction_id: rec.tx,
            amount: rec.amount.to_owned(),
        }
    }
}

pub fn find_debit(txs: &Vec<TransactionType>) -> Option<TransactionType> {
    txs.iter().find_map(|t| match t {
        TransactionType::Deposit(t) => Some(TransactionType::Deposit(*t)),
        TransactionType::Withdrawal(t) => Some(TransactionType::Withdrawal(*t)),
        _ => None,
    })
}

pub fn find_dispute(txs: &Vec<TransactionType>) -> Option<TransactionType> {
    txs.iter().find_map(|t| match t {
        TransactionType::Dispute(t) => Some(TransactionType::Dispute(*t)),
        _ => None,
    })
}

pub fn find_resolve(txs: &Vec<TransactionType>) -> Option<TransactionType> {
    txs.iter().find_map(|t| match t {
        TransactionType::Resolve(t) => Some(TransactionType::Resolve(*t)),
        _ => None,
    })
}
