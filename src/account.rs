use crate::transaction::{self, TransactionType};
use anyhow::{anyhow, Result};
use rust_decimal::Decimal;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Account {
    id: u16,
    total: Decimal,
    held: Decimal,
    locked: bool,
    transactions: HashMap<u32, Vec<TransactionType>>,
}

impl Account {
    pub fn new(id: u16) -> Self {
        Account {
            id,
            total: Decimal::new(0, 4),
            held: Decimal::new(0, 4),
            locked: false,
            transactions: HashMap::default(),
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn total(&self) -> Decimal {
        self.total
    }

    pub fn held(&self) -> Decimal {
        self.held
    }

    pub fn locked(&self) -> bool {
        self.locked
    }

    fn transaction_is_valid(&self, tx: &Option<TransactionType>) -> bool {
        match tx {
            Some(TransactionType::Deposit(t)) => {
                if self.locked {
                    return false;
                }

                if let None = t.amount() {
                    return false;
                }

                true
            }
            Some(TransactionType::Withdrawal(t)) => {
                if self.locked {
                    return false;
                };

                if let None = t.amount() {
                    return false;
                }

                let withdrawal_amount = t.amount().unwrap();

                self.available() >= withdrawal_amount
            }
            // These three types will do nothing if
            // the transaction that they reference doesn't exist,
            // so we can just try to apply them.
            //
            // There is expensive lookup logic that shouldn't
            // be repeated.
            Some(TransactionType::Dispute(_)) => !self.locked,
            Some(TransactionType::Resolve(_)) => !self.locked,
            Some(TransactionType::Chargeback(_)) => !self.locked,
            None => false,
        }
    }

    pub fn available(&self) -> Decimal {
        self.total - self.held
    }

    pub fn apply(&mut self, tx: Option<TransactionType>) -> Result<()> {
        if !self.transaction_is_valid(&tx) {
            return Err(anyhow!("invalid transaction: {:?}", tx));
        }

        let tx = tx.unwrap();

        match tx {
            TransactionType::Deposit(t) => {
                if let Some(_) = self.transactions.get(&t.transaction_id()) {
                    return Err(anyhow!("can't deposit the same money twice."));
                }

                self.transactions.insert(t.transaction_id(), vec![tx]);
                self.total += t.amount().unwrap();

                Ok(())
            }
            TransactionType::Withdrawal(t) => {
                if let Some(_) = self.transactions.get(&t.transaction_id()) {
                    return Err(anyhow!("can't withdraw the same money twice."));
                }

                self.transactions.insert(t.transaction_id(), vec![tx]);
                self.total -= t.amount().unwrap();

                Ok(())
            }
            TransactionType::Dispute(t) => {
                let txs = self.transactions.get(&t.transaction_id());

                if let None = txs {
                    return Err(anyhow!("dispute for nonexistent transaction."));
                }

                let txs = txs.unwrap();

                if let Some(_) = transaction::find_dispute(txs) {
                    return Err(anyhow!("transaction already disputed."));
                }

                let debit = transaction::find_debit(txs).unwrap();
                let txs = self.transactions.get_mut(&t.transaction_id()).unwrap();
                txs.push(tx);

                // apply the hold.
                match debit {
                    TransactionType::Deposit(t) => {
                        self.held += t.amount().unwrap();
                    }
                    TransactionType::Withdrawal(t) => {
                        self.held += t.amount().unwrap();
                        self.total += t.amount().unwrap();
                    }
                    _ => return Err(anyhow!("impossible transaction type found: {:?}", debit)),
                }

                Ok(())
            }
            TransactionType::Resolve(t) => {
                let txs = self.transactions.get(&t.transaction_id());

                if let None = txs {
                    return Err(anyhow!("dispute for nonexistent transaction."));
                }

                let txs = txs.unwrap();

                if let None = transaction::find_dispute(txs) {
                    return Err(anyhow!("transaction isn't disputed."));
                }

                if let Some(_) = transaction::find_resolve(txs) {
                    return Err(anyhow!("transaction is already resolved."));
                }

                let debit = transaction::find_debit(txs).unwrap();

                let txs = self.transactions.get_mut(&t.transaction_id()).unwrap();
                txs.push(tx);

                // make it rain
                match debit {
                    TransactionType::Deposit(t) | TransactionType::Withdrawal(t) => {
                        self.held -= t.amount().unwrap();
                    }
                    _ => return Err(anyhow!("impossible transaction type found: {:?}", debit)),
                }

                Ok(())
            }
            TransactionType::Chargeback(t) => {
                let txs = self.transactions.get(&t.transaction_id());

                if let None = txs {
                    return Err(anyhow!("dispute for nonexistent transaction."));
                }

                let txs = txs.unwrap();

                if let None = transaction::find_dispute(txs) {
                    return Err(anyhow!("transaction isn't disputed."));
                }

                if let Some(_) = transaction::find_resolve(txs) {
                    return Err(anyhow!("transaction is already resolved."));
                }

                let debit = transaction::find_debit(txs).unwrap();

                let txs = self.transactions.get_mut(&t.transaction_id()).unwrap();
                txs.push(tx);

                // reverse the transaction.
                match debit {
                    TransactionType::Deposit(t) => {
                        self.held -= t.amount().unwrap();
                        self.total -= t.amount().unwrap();
                    }
                    TransactionType::Withdrawal(t) => {
                        self.held -= t.amount().unwrap();
                        self.total += t.amount().unwrap();
                    }
                    _ => return Err(anyhow!("impossible transaction type found: {:?}", debit)),
                }

                self.locked = true;

                Ok(())
            }
        }
    }
}
