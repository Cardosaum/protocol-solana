use super::*;
use ark_crypto_primitives::{Error, CRH as CRHTrait};
use ark_ff::{BigInteger, PrimeField};
use ark_serialize::Read;
use ark_std::{marker::PhantomData, vec::Vec};
use arkworks_gadgets::poseidon::CRH;
use arkworks_utils::poseidon::sbox::PoseidonSbox;
use arkworks_utils::poseidon::PoseidonParameters;
use solana_program::log::sol_log_compute_units;
// use solana_sdk::compute_budget::ComputeBudget
pub struct CircomPoseidonHasher<F: PrimeField>(PhantomData<F>);
pub struct DebugPoseidon;
use helpers::*;
use poseidon::*;
use sbox::*;

fn to_field_elements<F: PrimeField>(bytes: &[u8]) -> Result<Vec<F>, Error> {
    let max_size_bytes = F::BigInt::NUM_LIMBS * 8;

    // Pad the input with zeros
    let padding_len = (max_size_bytes - (bytes.len() % max_size_bytes)) % max_size_bytes;
    let padded_input: Vec<u8> = bytes
        .iter()
        .cloned()
        .chain(core::iter::repeat(0u8).take(padding_len))
        .collect();

    let res = padded_input
        .chunks(max_size_bytes)
        .map(F::read)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(res)
}

impl<F: PrimeField> CircomPoseidonHasher<F> {
    pub fn hash(input: &[u8], mut bytes: &[u8]) -> Result<Vec<u8>, Error> {
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

        // let params = PoseidonParameters::<F>::from_bytes(bytes)?;

        //
        // Compute `from_bytes` manually
        // loggin compute units as we progress
        //

        msg!("> width_u8...");
        let mut width_u8 = [0u8; 1];
        bytes.read_exact(&mut width_u8)?;
        let width = u8::from_be_bytes(width_u8);
        sol_log_compute_units();

        msg!("> full_rounds_len...");
        let mut full_rounds_len = [0u8; 1];
        bytes.read_exact(&mut full_rounds_len)?;
        let full_rounds = u8::from_be_bytes(full_rounds_len);
        sol_log_compute_units();

        msg!("> partial_rounds_u8...");
        let mut partial_rounds_u8 = [0u8; 1];
        bytes.read_exact(&mut partial_rounds_u8)?;
        let partial_rounds = u8::from_be_bytes(partial_rounds_u8);
        sol_log_compute_units();

        msg!("> exponentiation_u8...");
        let mut exponentiation_u8 = [0u8; 1];
        bytes.read_exact(&mut exponentiation_u8)?;
        let exp = i8::from_be_bytes(exponentiation_u8);
        sol_log_compute_units();

        msg!("> round_key_len...");
        let mut round_key_len = [0u8; 4];
        bytes.read_exact(&mut round_key_len)?;
        sol_log_compute_units();

        msg!("> round_key_len_usize...");
        let round_key_len_usize: usize = u32::from_be_bytes(round_key_len) as usize;
        let mut round_keys_buf = vec![0u8; round_key_len_usize];
        bytes.read_exact(&mut round_keys_buf)?;
        sol_log_compute_units();

        msg!("> round_keys...");
        let round_keys = to_field_elements::<F>(&round_keys_buf)?;
        let mut mds_matrix_inner_vec_len = [0u8; 4];
        bytes.read_exact(&mut mds_matrix_inner_vec_len)?;
        sol_log_compute_units();

        msg!("> inner_vec_len_usize...");
        let inner_vec_len_usize = u32::from_be_bytes(mds_matrix_inner_vec_len) as usize;
        let mut mds_matrix: Vec<Vec<F>> = vec![];
        sol_log_compute_units();

        msg!("> while bytes not empty...");
        while !bytes.is_empty() {
            let mut inner_vec_buf = vec![0u8; inner_vec_len_usize];
            bytes.read_exact(&mut inner_vec_buf)?;

            let inner_vec = to_field_elements::<F>(&inner_vec_buf)?;
            mds_matrix.push(inner_vec);
        }
        sol_log_compute_units();

        let params = PoseidonParameters {
            round_keys,
            mds_matrix,
            width,
            full_rounds,
            partial_rounds,
            sbox: PoseidonSbox(exp),
        };

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
