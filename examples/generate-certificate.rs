//! Example: Generate a self-signed certificate
//!
//! This example demonstrates how to generate a self-signed certificate
//! using the simplified API with the SigningAlgorithm enum.

use yubikey::{
    Certificate, PinPolicy, SigningAlgorithm, TouchPolicy, YubiKey,
    piv::{AlgorithmId, SlotId},
};
use x509_cert::{
    name::Name,
    serial_number::SerialNumber,
    spki::SubjectPublicKeyInfoOwned,
    time::Validity,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the YubiKey
    let mut yubikey = YubiKey::open()?;

    // Authenticate with the management key (default key)
    // In production, use a proper management key
    // yubikey.authenticate(...)?;

    // Choose the slot to use
    let slot = SlotId::Authentication;

    // Choose the signing algorithm
    let algorithm = SigningAlgorithm::EccP256; // or EccP384, Rsa2048, etc.

    // Generate a key pair in the slot
    // let subject_pki = yubikey.generate(
    //     slot,
    //     AlgorithmId::EccP256,
    //     PinPolicy::Default,
    //     TouchPolicy::Default,
    // )?;

    // For this example, we'll use a placeholder
    // In real usage, you'd generate or import a key first
    println!("To use this example:");
    println!("1. Connect your YubiKey");
    println!("2. Authenticate with the management key");
    println!("3. Generate a key pair in the desired slot");
    println!("4. Create a certificate with the chosen algorithm:");
    println!();
    println!("Available algorithms:");
    println!("  - SigningAlgorithm::EccP256  (NIST P-256)");
    println!("  - SigningAlgorithm::EccP384  (NIST P-384)");
    println!("  - SigningAlgorithm::Rsa1024  (RSA 1024-bit)");
    println!("  - SigningAlgorithm::Rsa2048  (RSA 2048-bit)");
    println!("  - SigningAlgorithm::Rsa3072  (RSA 3072-bit)");
    println!("  - SigningAlgorithm::Rsa4096  (RSA 4096-bit)");
    println!();
    println!("Example usage:");
    println!(r#"
let cert = Certificate::generate_self_signed(
    &mut yubikey,
    slot,
    serial_number,
    validity,
    subject,
    subject_pki,
    SigningAlgorithm::EccP256,  // <-- Choose your algorithm here
    |builder| Ok(()),  // No extensions
)?;
"#);

    Ok(())
}
