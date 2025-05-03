use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, OpenOptions};
use std::io::{Read, Seek, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountConfig {
    accounts: Vec<Account>,
    path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Account {
    email: String,
}

impl AccountConfig {
    pub fn load(path: PathBuf) -> Result<Self> {
        // Ensure parent directories exist
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }

        // Open the file (create if it doesn't exist)
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;

        // Check if the file is empty
        if file.metadata()?.len() == 0 {
            // If empty, write an empty JSON array
            file.write_all(b"[]")?;
        }

        // Move the cursor to the beginning of the file
        file.seek(std::io::SeekFrom::Start(0))?;

        // Read the file
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)
            .map_err(|e| Error::from(format!("Failed to read config file: {}", e.to_string())))?;

        let accounts: Vec<Account> = serde_json::from_str(&file_contents)
            .map_err(|e| Error::from(format!("Failed to parse config file: {}", e.to_string())))?;

        let config = AccountConfig { accounts, path };
        Ok(config)
    }

    pub fn add_account(&mut self, email: String) -> Result<()> {
        let account = Account { email };

        // Check if the account already exists
        if self.accounts.contains(&account) {
            return Err(Error::from("Account already exists"));
        }

        // Add the new account
        self.accounts.push(account);
        self.save_config()?;
        Ok(())
    }

    pub fn remove_account(&mut self, email: &str) -> Result<()> {
        let account = Account {
            email: email.to_string(),
        };

        // Check if the account exists
        if let Some(pos) = self.accounts.iter().position(|x| *x == account) {
            // Remove the account
            self.accounts.remove(pos);
            self.save_config()?;
            Ok(())
        } else {
            Err(Error::from("Account not found"))
        }
    }

    fn save_config(&self) -> Result<()> {
        // Open the file (create if it doesn't exist)
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        // Serialize the config to JSON
        let config_json = serde_json::to_string(&self.accounts)
            .map_err(|e| Error::from(format!("Failed to serialize config: {}", e.to_string())))?;

        // Write the JSON to the file
        file.write_all(config_json.as_bytes())
            .map_err(|e| Error::from(format!("Failed to write config file: {}", e.to_string())))?;

        Ok(())
    }

    pub fn accounts(&self) -> &[Account] {
        &self.accounts
    }
}
