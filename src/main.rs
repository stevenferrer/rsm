use rsm::{balances, system};

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type Nonce = types::Nonce;
	type BlockNumber = types::BlockNumber;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

fn main() {
	let mut rt = Runtime::new();

	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	rt.balances.set_balance(&alice, 100);

	rt.system.inc_block_number();
	assert_eq!(rt.system.block_number(), 1);

	rt.system.inc_nonce(&alice);
	let _res = rt
		.balances
		.transfer(alice.clone(), bob.clone(), 30)
		.map_err(|e| eprintln!("error: {}", e));

	rt.system.inc_nonce(&alice);
	let _res = rt
		.balances
		.transfer(alice.clone(), charlie.clone(), 20)
		.map_err(|e| println!("error: {}", e));

	println!("{:#?}", rt);
}
