use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

/// Develop a program that multiplies, divides, adds, subtracts
/// two numeric variables a, b, the value of which is > 2^20.
use num_bigint::ToBigUint;

fn operations<T>(a: T, b: T)
where
    T: Add + Sub + Mul + Div + Clone + Display,
    <T as Add>::Output: Display,
    <T as Sub>::Output: Display,
    <T as Mul>::Output: Display,
    <T as Div>::Output: Display,
{
    let sum = a.clone() + b.clone();
    println!("Addition: {a} + {b} = {sum}");

    let difference = a.clone() - b.clone();
    println!("Subtraction: {a} - {b} = {difference}");

    let product = a.clone() * b.clone();
    println!("Multiplication: {a} * {b} = {product}");

    let quotient = a.clone() / b.clone();
    println!("Division: {a} / {b} = {quotient}");
}

fn calculate_u64(a: u64, b: u64) {
    if a.min(b) <= 2_u64.pow(20) {
        eprintln!("Provide a numbers that is greater than 2^20 (1_048_576)");
        return;
    }

    println!("====== calculate ======");
    println!("a = {a}");
    println!("b = {b}");
    operations(a, b);
}

fn calculate_bignum(a: u64, b: u64) {
    if a.min(b) <= 2_u64.pow(20) {
        eprintln!("Provide a numbers that is greater than 2^20 (1_048_576)");
        return;
    }

    let a = a.to_biguint().unwrap();
    let b = b.to_biguint().unwrap();

    println!("====== calculate_bignum ======");
    println!("a = {a}");
    println!("b = {b}");
    operations(a, b);
    println!("==============================");
}

fn main() {
    let a = 2_u64.pow(32);
    let b = 2_u64.pow(26);
    calculate_u64(a, b);

    let a = 2_u64.pow(62);
    let b = 2_u64.pow(56);
    calculate_bignum(a, b);
}
