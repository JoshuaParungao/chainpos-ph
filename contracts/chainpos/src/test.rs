#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_record_sale() {
    let env = Env::default();

    let receipt_id = String::from_str(&env, "RCPT001");
    let customer = String::from_str(&env, "Joshua");

    ChainPOSContract::record_sale(
        env.clone(),
        receipt_id.clone(),
        customer,
        500,
    );

    let receipt = ChainPOSContract::verify_receipt(
        env.clone(),
        receipt_id,
    );

    assert_eq!(receipt.amount, 500);
}

#[test]
fn test_verify_receipt() {
    let env = Env::default();

    let receipt_id = String::from_str(&env, "RCPT002");
    let customer = String::from_str(&env, "Maria");

    ChainPOSContract::record_sale(
        env.clone(),
        receipt_id.clone(),
        customer,
        1000,
    );

    let receipt = ChainPOSContract::verify_receipt(
        env.clone(),
        receipt_id,
    );

    assert_eq!(receipt.status, String::from_str(&env, "VALID"));
}

#[test]
fn test_refund_sale() {
    let env = Env::default();

    let receipt_id = String::from_str(&env, "RCPT003");
    let customer = String::from_str(&env, "John");

    ChainPOSContract::record_sale(
        env.clone(),
        receipt_id.clone(),
        customer,
        700,
    );

    ChainPOSContract::refund_sale(
        env.clone(),
        receipt_id.clone(),
    );

    let receipt = ChainPOSContract::verify_receipt(
        env.clone(),
        receipt_id,
    );

    assert_eq!(receipt.status, String::from_str(&env, "REFUNDED"));
}

#[test]
fn test_multiple_sales() {
    let env = Env::default();

    let receipt_id = String::from_str(&env, "RCPT004");
    let customer = String::from_str(&env, "Ana");

    ChainPOSContract::record_sale(
        env.clone(),
        receipt_id.clone(),
        customer,
        1200,
    );

    let receipt = ChainPOSContract::verify_receipt(
        env.clone(),
        receipt_id,
    );

    assert_eq!(receipt.amount, 1200);
}

#[test]
fn test_receipt_exists() {
    let env = Env::default();

    let receipt_id = String::from_str(&env, "RCPT005");
    let customer = String::from_str(&env, "Leo");

    ChainPOSContract::record_sale(
        env.clone(),
        receipt_id.clone(),
        customer,
        300,
    );

    let receipt = ChainPOSContract::verify_receipt(
        env.clone(),
        receipt_id,
    );

    assert_eq!(receipt.customer, String::from_str(&env, "Leo"));
}