use gsm::crypto;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

#[test]
fn encrypt_decrypt_roundtrip() {
    let plaintext = b"hello world";
    let password = b"supersecret";
    // encrypt returns (salt, nonce, ciphertext)
    let (salt, nonce, ciphertext) = crypto::encrypt(plaintext, password).expect("encrypt");
    let decrypted = crypto::decrypt(&ciphertext, password, &salt, &nonce).expect("decrypt");
    assert_eq!(decrypted, plaintext);
}

#[test]
fn decrypt_fails_with_wrong_password() {
    let plaintext = b"secret";
    let password = b"correct";
    let (salt, nonce, ciphertext) = crypto::encrypt(plaintext, password).expect("encrypt");

    let wrong_password = b"incorrect";
    let result = crypto::decrypt(&ciphertext, wrong_password, &salt, &nonce);
    assert!(
        result.is_err(),
        "decryption should fail with wrong password"
    );
}

#[test]
fn derive_key_matches_pbkdf2() {
    let password = b"password";
    let salt = b"saltsalt";
    let derived = crypto::derive_key(password, salt);

    let mut expected = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password, salt, 100_000, &mut expected);
    assert_eq!(derived.to_vec(), expected.to_vec());
}
