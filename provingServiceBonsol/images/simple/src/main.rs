

#![no_main]
use risc0_zkvm::{guest::{env, sha::Impl},sha::{Digest, Sha256}};

risc0_zkvm::guest::entry!(main);

fn FHEAdd(user_previous_balance: u128, user_new_balance: u128) -> u128 {

    let constant = u128::from_dec_str("73786976294838206464").unwrap(); // 2^66

    let mut normalized_prev_balance: u128;

    if constant > user_previous_balance {
        normalized_prev_balance = u128::from(1);
    } else {
        normalized_prev_balance = user_previous_balance - constant;
    }

    let normalized_new_balance = user_new_balance - constant;

    // Multiply the two normalized values
    let product = normalized_prev_balance.overflowing_mul(normalized_new_balance).0;

    // Add 2 times the constant because of the property of the encryption function
    let encrypted_sum = product + constant;
    encrypted_sum
}

fn main() {
    // The Bonsol Host will send your inputs packed in a Vec<u8> 
    let mut public1 = Vec::new();
    env::read_slice(&mut public1);

    // Ensure that `public1` has exactly 16 bytes (128 bits) since u128::from_be_bytes expects an array of 16 bytes.
    assert_eq!(public1.len(), 16, "Expected 16 bytes for a u128 integer.");

    // Convert the Vec<u8> into an array of 16 bytes. This step is necessary because from_be_bytes expects a fixed-size array.
    let public1_array: [u8; 16] = public1.try_into().expect("Slice with incorrect length");

    // Convert the byte array into a u128 integer assuming big-endian format.
    let previous_vote_state = u128::from_be_bytes(public1_array);


    let mut public2 = Vec::new();
    env::read_slice(&mut public2);

    // Ensure that `public1` has exactly 16 bytes (128 bits) since u128::from_be_bytes expects an array of 16 bytes.
    assert_eq!(public2.len(), 16, "Expected 16 bytes for a u128 integer.");

    // Convert the Vec<u8> into an array of 16 bytes. This step is necessary because from_be_bytes expects a fixed-size array.
    let public2_array: [u8; 16] = public2.try_into().expect("Slice with incorrect length");

    // Convert the byte array into a u128 integer assuming big-endian format.
    let new_vote_value = u128::from_be_bytes(public2_array);
    
    
    // Run the computation.
    let result = FHEAdd(previous_vote_state, new_vote_value);
    
    //Serialise the output into a byte array
    let byte_array: [u8; 16] = result.to_le_bytes();

    //commit the serialized result onto the seal
    env::commit_slice(byte_array);
   
}
