pub struct SavingsBankAccount {
    balance: i32,
}

impl SavingsBankAccount {
    fn new() -> SavingsBankAccount {
        SavingsBankAccount {
            balance: 0
        }
    }

    fn get_balance(&self) -> i32 {
        self.balance
    }

    fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Deposit amount should not be a negative value")
        }

        self.balance += amount
    }

    fn transfer_funds(&self, recipient: i32, amount: i32) -> Result<String, String> {
        Ok(format!("{amount} was successfully transferred to {recipient}"))
    }
}

mod tests {
    use super::*;

    #[test]
    fn new_bank_account_balance_should_be_zero() {
        let account = SavingsBankAccount::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn deposit_value_should_be_added_to_balance() {
        let mut account = SavingsBankAccount::new();

        account.deposit(100);
        assert_eq!(account.get_balance(), 100);
    }

    #[test]
    fn should_transfer_funds() -> Result<(), String> {
        let mut account = SavingsBankAccount::new();
        account.deposit(1000);
        account.transfer_funds(12345, 500)?;
        Ok(())
    }

    #[test]
    #[should_panic]
    fn should_panic_if_negative_deposit_value() {
        let mut account = SavingsBankAccount::new();
        account.deposit(-123)
    }
}

