use super::*;
use ark_crypto_primitives::{Error, CRH as CRHTrait};
use ark_ff::{BigInteger, PrimeField};
use ark_std::{marker::PhantomData, vec::Vec};
use arkworks_gadgets::poseidon::CRH;
use arkworks_utils::poseidon::PoseidonParameters;
use solana_program::log::sol_log_compute_units;
// use solana_sdk::compute_budget::ComputeBudget
pub struct CircomPoseidonHasher<F: PrimeField>(PhantomData<F>);
pub struct DebugPoseidon;
use helpers::*;

impl<F: PrimeField> CircomPoseidonHasher<F> {
    pub fn hash(input: &[u8], param_bytes: &[u8]) -> Result<Vec<u8>, Error> {
        msg!("> Starting hash...");
        sol_log_compute_units();

        // msg!("> Stack bomb...");
        // stack_bomb();
        // sol_log_compute_units();

        // msg!("> Ackermann...");
        // call_ackermann();
        // sol_log_compute_units();

        // msg!("> Consume units...");
        // consume_all_compute_units();
        // sol_log_compute_units();

        let params = PoseidonParameters::<F>::from_bytes(param_bytes)?;
        msg!("\n\nIHAAA EU APARECO\n\n");
        let output: F = <CRH<F> as CRHTrait>::evaluate(&params, input)?;
        let value = output.into_repr().to_bytes_le();
        Ok(value)
    }
}

use ark_bn254::Fr as Bn254;
pub type BN254CircomPoseidon3x5Hasher = CircomPoseidonHasher<Bn254>;
pub type DEBUGBN254Poseidon = DebugPoseidon;

impl DebugPoseidon {
    pub fn hash(input: &[u8], param_bytes: &[u8]) -> Vec<u8> {
        let res: Vec<u8> = input.to_vec();
        res
    }
}
