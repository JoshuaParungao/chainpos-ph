#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, String};

#[contracttype]
#[derive(Clone)]
pub struct Receipt {
    pub receipt_id: String,
    pub customer: String,
    pub amount: i128,
    pub status: String,
}

#[contract]
pub struct ChainPOSContract;

#[contractimpl]
impl ChainPOSContract {

    // Record a new sale
    pub fn record_sale(
        env: Env,
        receipt_id: String,
        customer: String,
        amount: i128,
    ) {

        let receipt = Receipt {
            receipt_id: receipt_id.clone(),
            customer,
            amount,
            status: String::from_str(&env, "VALID"),
        };

        env.storage().persistent().set(&receipt_id, &receipt);
    }

    // Verify receipt
    pub fn verify_receipt(
        env: Env,
        receipt_id: String,
    ) -> Receipt {

        env.storage()
            .persistent()
            .get(&receipt_id)
            .unwrap()
    }

    // Refund receipt
    pub fn refund_sale(
        env: Env,
        receipt_id: String,
    ) {

        let mut receipt: Receipt = env
            .storage()
            .persistent()
            .get(&receipt_id)
            .unwrap();

        receipt.status = String::from_str(&env, "REFUNDED");

        env.storage()
            .persistent()
            .set(&receipt_id, &receipt);
    }
}

mod test;