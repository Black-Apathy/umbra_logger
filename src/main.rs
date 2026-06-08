use dotenvy::dotenv;
use std::env;
// use std::path::Path;
use umbra_logger::db::UmbraDB;
use umbra_logger::models::{Category, Counterparty, Transaction};

use chrono::NaiveDate;
use inquire::{DateSelect, Select, Text};

// 1. Helper Function (UI Logic)
fn create_transaction_prompt(
    account_id: i32,
    default_date: NaiveDate,
    categories: Vec<Category>,
    counterparties: Vec<Counterparty>,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    // Step A: Ask for Date (Defaults to the last one used)
    // This allows you to just hit ENTER if the date hasn't changed.
    let date = DateSelect::new("Date:")
        .with_default(default_date)
        .prompt()?;

    let amount_str = Text::new("Amount:").prompt()?;
    let amount: f64 = amount_str.trim().parse()?;

    let direction = Select::new("Direction:", vec!["credit", "debit"]).prompt()?;
    let note = Text::new("Note:").prompt()?;

    let category = Select::new("Category:", categories).prompt()?;
    let mut cp_options = counterparties.clone();

    // 2. Insert a "Fake" option at the top (Index 0)
    // We use ID -1 to signal that this is the "None" choice
    cp_options.insert(
        0,
        Counterparty {
            id: -1,
            name: "❌ No Counterparty".to_string(),
        },
    );

    // 3. Ask the user
    let selected_cp = Select::new("Counterparty:", cp_options).prompt()?;

    // 4. Convert the choice: If ID is -1, it becomes None. Otherwise, it's Some(id).
    let final_cp_id = if selected_cp.id == -1 {
        None
    } else {
        Some(selected_cp.id)
    };

    Ok(Transaction {
        account_id,
        date,
        amount,
        direction: direction.to_string(),
        note,
        category_id: category.id,
        counterparty_id: final_cp_id, // Pass the Option
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize
    dotenv().ok();

    let db_path_string = env::var("DATABASE_URL").unwrap_or_else(|_| "MyFinance.db".to_string());

    println!("📂 Using Database: {}", db_path_string);

    // 3. Connect (Using the dynamic path)
    let db = UmbraDB::new(&db_path_string)?;
    println!("Database connection: Secured.");

    // 2. Load Global Data
    let accounts = db.get_accounts()?;
    let categories = db.get_categories()?;
    let counterparties = db.get_counterparties()?;

    // 3. Select Account (The Context)
    let chosen_account = Select::new("Select an Account:", accounts).prompt()?;

    // ================= MAIN MENU LOOP =================
    loop {
        println!("\n=== 🏦 {} Menu ===", chosen_account.name);

        let main_choice = Select::new(
            "What would you like to do?",
            vec!["📝 Add Transactions", "💰 Check Balance", "🚪 Exit"],
        )
        .prompt()?;

        match main_choice {
            "🚪 Exit" => {
                println!("Goodbye!");
                break;
            }
            "💰 Check Balance" => {
                match db.get_balance_breakdown(chosen_account.id) {
                    Ok((opening, credits, debits, total)) => {
                        println!("\n=== 🧾 BALANCE BREAKDOWN ===");
                        println!(" 🏁 Opening Balance:   {:>10.2}", opening);
                        println!(" 📈 Total Credits:     {:>10.2}", credits);
                        println!(" 📉 Total Debits:      {:>10.2}", debits);
                        println!("---------------------------------");
                        println!(" 💰 CALCULATED:        {:>10.2}", total);
                        println!("=================================\n");
                    }
                    Err(e) => println!("❌ Error: {}", e),
                }
                // Loop restarts here, taking you back to menu
            }
            "📝 Add Transactions" => {
                // --- SUB-SYSTEM: TRANSACTION ENTRY ---
                let mut current_date = DateSelect::new("Starting Date?").prompt()?;
                let mut cart: Vec<Transaction> = Vec::new();

                // Loop A: Input
                loop {
                    println!("\n--- Entry #{} ---", cart.len() + 1);
                    let new_txn = create_transaction_prompt(
                        chosen_account.id,
                        current_date,
                        categories.clone(),
                        counterparties.clone(),
                    )?;
                    current_date = new_txn.date; // Sticky Date update
                    cart.push(new_txn);

                    if !inquire::Confirm::new("Add another?")
                        .with_default(true)
                        .prompt()?
                    {
                        break;
                    }
                }

                // Loop B: Review & Save
                loop {
                    println!("\n--- REVIEW BATCH ({}) ---", cart.len());
                    for (i, txn) in cart.iter().enumerate() {
                        println!(
                            "{}. {} | {} | {} | {}",
                            i + 1,
                            txn.date,
                            txn.amount,
                            txn.note,
                            txn.category_id
                        );
                    }

                    let action = Select::new(
                        "Batch Action:",
                        vec![
                            "✅ Save & Commit",
                            "✏️ Edit Entry",
                            "❌ Delete Entry",
                            "🗑️ Discard Batch",
                        ],
                    )
                    .prompt()?;

                    match action {
                        "✅ Save & Commit" => {
                            let mut count = 0;
                            for txn in cart {
                                if db.save_transaction(&txn).is_ok() {
                                    count += 1;
                                }
                            }
                            println!("\n🚀 Saved {} transactions.", count);
                            break;
                        }
                        "✏️ Edit Entry" => {
                            let idx_str = Text::new("Entry # to edit?").prompt()?;
                            if let Ok(idx) = idx_str.trim().parse::<usize>() {
                                if idx > 0 && idx <= cart.len() {
                                    let old_date = cart[idx - 1].date;
                                    println!("Editing Entry #{}...", idx);
                                    let new_txn = create_transaction_prompt(
                                        chosen_account.id,
                                        old_date,
                                        categories.clone(),
                                        counterparties.clone(),
                                    )?;
                                    cart[idx - 1] = new_txn;
                                }
                            }
                        }
                        "❌ Delete Entry" => {
                            let idx_str = Text::new("Entry # to delete?").prompt()?;
                            if let Ok(idx) = idx_str.trim().parse::<usize>() {
                                if idx > 0 && idx <= cart.len() {
                                    cart.remove(idx - 1);
                                }
                            }
                        }
                        "🗑️ Discard Batch" => {
                            println!("Batch discarded.");
                            break;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
