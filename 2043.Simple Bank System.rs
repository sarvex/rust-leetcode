struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    /// Simple bank system supporting transfers, deposits, and withdrawals.
    ///
    /// # Intuition
    /// Validate account indices before every operation. Transfers, deposits,
    /// and withdrawals are straightforward balance mutations guarded by
    /// bounds and sufficiency checks.
    ///
    /// # Approach
    /// Store balances in a Vec indexed by `account - 1`. Each operation first
    /// validates account numbers, then checks sufficient funds for transfers
    /// and withdrawals before mutating balances.
    ///
    /// # Complexity
    /// - Each operation: O(1)
    /// - Space: O(n) for n accounts
    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    fn is_valid(&self, account: i32) -> bool {
        (account as usize) <= self.balance.len()
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if !self.is_valid(account1) || !self.is_valid(account2) {
            return false;
        }
        let idx1 = (account1 - 1) as usize;
        if self.balance[idx1] < money {
            return false;
        }
        self.balance[idx1] -= money;
        self.balance[(account2 - 1) as usize] += money;
        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if !self.is_valid(account) {
            return false;
        }
        self.balance[(account - 1) as usize] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if !self.is_valid(account) {
            return false;
        }
        let idx = (account - 1) as usize;
        if self.balance[idx] < money {
            return false;
        }
        self.balance[idx] -= money;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank_operations() {
        let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
        assert!(bank.transfer(3, 5, 10));
        assert!(bank.deposit(5, 20));
        assert!(!bank.transfer(5, 1, 100));
        assert!(bank.withdraw(1, 10));
    }
}
