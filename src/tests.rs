use crate::privkey::WinternitzPrivkey;

pub const TEST_MESSAGE: [u8;4] = *b"test";

#[test]
fn keygen() {
    WinternitzPrivkey::default();
}

#[test]
fn keygen_sign_verify_compact() {
    let privkey = WinternitzPrivkey::generate();
    let hash = privkey.pubkey().hash();
    let signature = privkey.sign(&TEST_MESSAGE);
    let recovered_hash = signature.recover_pubkey(&TEST_MESSAGE).hash();
    assert_eq!(hash, recovered_hash)
}