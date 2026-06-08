// src/db.rs
use crate::models::{Account, Category, Counterparty, Transaction}; // Import all models
use rusqlite::{Connection, Result};

pub struct UmbraDB {
    conn: Connection,
}

impl UmbraDB {
    // 1. Constructor: Opens the connection
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(UmbraDB { conn })
    }

    // 2. Get Accounts
    pub fn get_accounts(&self) -> Result<Vec<Account>> {
        let mut stmt = self
            .conn
            .prepare("SELECT account_id, account_name FROM Accounts")?;
        let accounts = stmt
            .query_map([], |row| {
                Ok(Account {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })?
            .collect(); // Rust infers the type automatically now
        accounts
    }

    // 3. Get Categories (Moved inside!)
    pub fn get_categories(&self) -> Result<Vec<Category>> {
        let mut stmt = self
            .conn
            .prepare("SELECT category_id, category_name FROM Categories")?;
        let categories = stmt
            .query_map([], |row| {
                Ok(Category {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })?
            .collect();
        categories
    }

    // 4. Get Counterparties (Moved inside!)
    pub fn get_counterparties(&self) -> Result<Vec<Counterparty>> {
        let mut stmt = self
            .conn
            .prepare("SELECT counterparty_id, counterparty_name FROM Counterparties")?;
        let counterparties = stmt
            .query_map([], |row| {
                Ok(Counterparty {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })?
            .collect();
        counterparties
    }

    // 5. Save Transaction
    // Notice we use &txn.direction (borrowing string) vs txn.category_id (copying int)
    pub fn save_transaction(&self, txn: &Transaction) -> Result<()> {
        self.conn.execute(
            "INSERT INTO Transactions (date, amount, category_id, account_id, counterparty_id, direction, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                txn.date.to_string(),
                txn.amount,
                txn.category_id,
                txn.account_id,
                txn.counterparty_id,
                &txn.direction, // Need & for String in rusqlite
                &txn.note,      // Need & for String in rusqlite
            ),
        )?;
        Ok(())
    }

    // NEW: Calculate the live balance
    // Returns tuple: (Opening, Credits, Debits, Total)
    pub fn get_balance_breakdown(&self, account_id: i32) -> Result<(f64, f64, f64, f64)> {
        // 1. Get Opening
        let opening: f64 = self
            .conn
            .query_row(
                "SELECT opening_amount FROM OpeningBalances WHERE account_id = ?1",
                [account_id],
                |row| row.get(0),
            )
            .unwrap_or(0.0);

        // 2. Get Credits
        let credits: f64 = self.conn.query_row(
                "SELECT IFNULL(SUM(amount), 0.0) FROM Transactions WHERE account_id = ?1 AND direction = 'credit'",
                [account_id],
                |row| row.get(0),
            )?;

        // 3. Get Debits
        let debits: f64 = self.conn.query_row(
                "SELECT IFNULL(SUM(amount), 0.0) FROM Transactions WHERE account_id = ?1 AND direction = 'debit'",
                [account_id],
                |row| row.get(0),
            )?;

        let total = opening + credits - debits;
        Ok((opening, credits, debits, total))
    }
}
