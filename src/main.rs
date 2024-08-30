use rsm::{
	balances, poe,
	support::{self, Dispatch},
	system,
};

mod types {
	use rsm::support;

	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = support::Header<BlockNumber>;
	pub type Block = support::Block<Header, Extrinsic>;
	pub type Content = &'static str;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[macros::runtime]
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	poe: poe::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type Nonce = types::Nonce;
	type BlockNumber = types::BlockNumber;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl poe::Config for Runtime {
	type Content = types::Content;
}

fn main() {
	let mut rt = Runtime::new();

	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	rt.balances.set_balance(&alice, 100);

	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer {
					to: bob.clone(),
					amount: 69,
				}),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer { to: charlie, amount: 20 }),
			},
		],
	};

	let block_2 = types::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::poe(poe::Call::create_claim { claim: "Hello, world!" }),
			},
			support::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::poe(poe::Call::create_claim { claim: "Hello, world!" }),
			},
		],
	};

	let block_3 = types::Block {
		header: support::Header { block_number: 3 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice,
				call: RuntimeCall::poe(poe::Call::revoke_claim { claim: "Hello, world!" }),
			},
			support::Extrinsic {
				caller: bob,
				call: RuntimeCall::poe(poe::Call::revoke_claim { claim: "Hello, world!" }),
			},
		],
	};

	rt.execute_block(block_1).expect("invalid block");
	rt.execute_block(block_2).expect("invalid block");
	rt.execute_block(block_3).expect("invalid block");

	println!("{:#?}", rt);
}
