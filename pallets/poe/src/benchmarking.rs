//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::Pallet as Poe;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use core::convert::TryInto;
use sp_std::vec::Vec;

fn add_claim<T: Config>(s: u32) -> Result<Vec::<u8>, &'static str> {
	let caller: T::AccountId = whitelisted_caller();
	let proof = generate_proof(s);
	Poe::<T>::create_claim(RawOrigin::Signed(caller).into(), proof.clone())?;
	Ok(proof)
}

fn generate_proof(s: u32) -> Vec::<u8> {
	let mut proof = Vec::<u8>::new();
	for i in 1 .. s {
		proof.push(i.try_into().unwrap());
	}
	proof
}

benchmarks! {
	create_claim {
		let s in 1 .. 32;
		let caller: T::AccountId = whitelisted_caller();
		let proof = generate_proof(s);
	}: _(RawOrigin::Signed(caller), proof)

	revoke_claim {
		let s in 1 .. 32;
		let caller: T::AccountId = whitelisted_caller();
		let proof = add_claim::<T>(s)?;
	}: _(RawOrigin::Signed(caller), proof)

	transfer_claim {
		let s in 1 .. 32;
		let caller: T::AccountId = whitelisted_caller();
		let recipient: T::AccountId = account("recipient", 0, 0);
		let proof = add_claim::<T>(s)?;
	}: _(RawOrigin::Signed(caller), recipient, proof)
}

impl_benchmark_test_suite!(Poe, crate::mock::new_test_ext(), crate::mock::Test,);
