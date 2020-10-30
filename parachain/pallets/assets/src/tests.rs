

use crate::mock::{new_tester, AccountId, Assets, Origin, System, MockRuntime, TestEvent};

use frame_support::{assert_ok, assert_noop};
use sp_keyring::AccountKeyring as Keyring;
use sp_core::H160;
use frame_support::storage::StorageDoubleMap;

use crate::{Balances, TotalIssuance};

use super::*;

fn last_event() -> TestEvent {
	System::events().pop().expect("Event expected").event
}

fn set_balance<T>(asset_id: &H160, account_id: &AccountId, amount: T)
	where T : Into<U256> + Copy
{
	let free = amount.into();
	Balances::<MockRuntime>::insert(&asset_id, &account_id, &free);
	TotalIssuance::<MockRuntime>::insert(&asset_id, free);
}

#[test]
fn deposit_should_increase_balance_and_total_issuance() {
	new_tester().execute_with(|| {
		let asset_id = H160::zero();
		let alice: AccountId = Keyring::Alice.into();
		assert_ok!(<Assets as MultiAsset<_,_>>::deposit(asset_id, &alice, 500.into()));
		assert_eq!(Balances::<MockRuntime>::get(&asset_id, &alice), 500.into());
		assert_eq!(TotalIssuance::<MockRuntime>::get(&asset_id), 500.into());

		assert_ok!(<Assets as MultiAsset<_,_>>::deposit(asset_id, &alice, 20.into()));
		assert_eq!(Balances::<MockRuntime>::get(&asset_id, &alice), 520.into());
		assert_eq!(TotalIssuance::<MockRuntime>::get(&asset_id), 520.into());
	});
}

#[test]
fn deposit_should_raise_total_issuance_overflow_error() {
	new_tester().execute_with(|| {
		let asset_id = H160::zero();
		let alice: AccountId = Keyring::Alice.into();
		TotalIssuance::<MockRuntime>::insert(&asset_id, U256::MAX);

		assert_noop!(
			<Assets as MultiAsset<_,_>>::deposit(asset_id, &alice, U256::one()),
			Error::<MockRuntime>::TotalIssuanceOverflow
		);
	});
}

#[test]
fn deposit_should_raise_balance_overflow_error() {
	new_tester().execute_with(|| {
		let asset_id = H160::zero();
		let alice: AccountId = Keyring::Alice.into();
		Balances::<MockRuntime>::insert(&asset_id, &alice, U256::MAX);

		assert_noop!(
			<Assets as MultiAsset<_,_>>::deposit(asset_id, &alice, U256::one()),
			Error::<MockRuntime>::BalanceOverflow
		);
	});
}

#[test]
fn withdrawal_should_decrease_balance_and_total_issuance() {
	new_tester().execute_with(|| {
		let asset_id = H160::zero();
		let alice: AccountId = Keyring::Alice.into();
		set_balance(&asset_id, &alice, 500);

		assert_ok!(<Assets as MultiAsset<_,_>>::withdraw(asset_id, &alice, 20.into()));
		assert_eq!(Balances::<MockRuntime>::get(&asset_id, &alice), 480.into());
		assert_eq!(TotalIssuance::<MockRuntime>::get(&asset_id), 480.into());
	});
}

#[test]
fn withdrawal_should_raise_total_issuance_underflow_error() {
	new_tester().execute_with(|| {
		let asset_id = H160::zero();
		let alice: AccountId = Keyring::Alice.into();
		TotalIssuance::<MockRuntime>::insert(&asset_id, U256::one());

		assert_noop!(
			<Assets as MultiAsset<_,_>>::withdraw(asset_id, &alice, 10.into()),
			Error::<MockRuntime>::TotalIssuanceUnderflow
		);

	});
}

#[test]
fn withdrawal_should_raise_balance_underflow_error() {
	new_tester().execute_with(|| {
		let asset_id = H160::zero();
		let alice: AccountId = Keyring::Alice.into();
		TotalIssuance::<MockRuntime>::insert(&asset_id, U256::from(500));

		assert_noop!(
			<Assets as MultiAsset<_,_>>::withdraw(asset_id, &alice, 10.into()),
			Error::<MockRuntime>::InsufficientBalance
		);

	});
}

#[test]
fn transfer_free_balance() {
	new_tester().execute_with(|| {

		let asset_id = H160::zero();
		let alice: AccountId = Keyring::Alice.into();
		let bob: AccountId = Keyring::Bob.into();

		assert_ok!(<Assets as MultiAsset<_,_>>::deposit(asset_id, &alice, 500.into()));
		assert_ok!(<Assets as MultiAsset<_,_>>::deposit(asset_id, &bob, 500.into()));
		assert_ok!(<Assets as MultiAsset<_,_>>::transfer(asset_id, &alice, &bob, 250.into()));

		assert_eq!(Balances::<MockRuntime>::get(&asset_id, &alice), 250.into());
		assert_eq!(Balances::<MockRuntime>::get(&asset_id, &bob), 750.into());
		assert_eq!(TotalIssuance::<MockRuntime>::get(&asset_id), 1000.into());
	});
}

#[test]
fn transfer_should_raise_insufficient_balance() {
	new_tester().execute_with(|| {

		let asset_id = H160::zero();
		let alice: AccountId = Keyring::Alice.into();
		let bob: AccountId = Keyring::Bob.into();

		assert_ok!(<Assets as MultiAsset<_,_>>::deposit(asset_id, &alice, 500.into()));
		assert_ok!(<Assets as MultiAsset<_,_>>::deposit(asset_id, &bob, 500.into()));

		assert_noop!(
			<Assets as MultiAsset<_,_>>::transfer(asset_id, &alice, &bob, 1000.into()),
			Error::<MockRuntime>::InsufficientBalance,
		);
	});
}
