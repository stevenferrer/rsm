use num::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	type Balance: CheckedAdd + CheckedSub + Zero + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	pub fn transfer(
		&mut self,
		source: T::AccountId,
		dest: T::AccountId,
		amount: T::Balance,
	) -> Result<(), &'static str> {
		let source_bal = self.balance(&source);
		let dest_bal = self.balance(&dest);

		let new_source_bal = source_bal.checked_sub(&amount).ok_or("Not enough funds")?;
		let new_dest_bal = dest_bal.checked_add(&amount).ok_or("Overflow")?;

		self.set_balance(&source, new_source_bal);
		self.set_balance(&dest, new_dest_bal);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	struct TestConfig;
	impl super::Config for TestConfig {
		type Balance = u128;
	}
	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_balances() {
		let mut bals = super::Pallet::<TestConfig>::new();

		assert_eq!(bals.balance(&"alice".to_string()), 0);
		bals.set_balance(&"alice".to_string(), 100);
		assert_eq!(bals.balance(&"alice".to_string()), 100);
		assert_eq!(bals.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut bals = super::Pallet::<TestConfig>::new();

		assert_eq!(
			bals.transfer("alice".to_string(), "bob".to_string(), 1),
			Err("Not enough funds")
		);

		bals.set_balance(&"alice".to_string(), 100);
		assert_eq!(bals.transfer("alice".to_string(), "bob".to_string(), 1), Ok(()));
		assert_eq!(bals.balance(&"alice".to_string()), 99);
		assert_eq!(bals.balance(&"bob".to_string()), 1);

		assert_eq!(
			bals.transfer("alice".to_string(), "bob".to_string(), 100),
			Err("Not enough funds")
		);
	}
}
