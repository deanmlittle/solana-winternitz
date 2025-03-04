# Winternitz Signature Implementation

An implementation of the Winternitz One-Time Signature (WOTS) scheme, designed for use with Solana. This implementation supports both SHA-256 and Keccak hash functions and is compatible with no_std environments.

## Features

- Support for multiple hash functions via the `WinternitzHash` trait
- Optimized address generation via Merkle tree computation
- Split signature support for more efficient verification
- Address encoding/decoding with base58
- BIP32 derivation path support for key generation from seed
- no_std compatible for Solana programs

## Usage

### Key Generation

```rust
use winternitz::{hash::WinternitzKeccak, privkey::WinternitzPrivkey};

// Generate a new random private key
let privkey = WinternitzPrivkey::generate();

// Or derive from a seed using BIP32
let seed = [0u8; 64]; // Use a proper seed in practice
let path = "m/30583'/0'/0'";
let privkey = WinternitzPrivkey::from_seed(seed, path).unwrap();

// Derive the corresponding public key
let pubkey = privkey.pubkey::<WinternitzKeccak>();
```

### Signing Messages

```rust
// Sign a message
let message = b"Hello, World!";
let signature = privkey.sign::<WinternitzKeccak>(message);
```

### Signature Verification

```rust
// Recover public key from signature and message
let recovered_pubkey = signature.recover_pubkey::<WinternitzKeccak>(message);

// Verify by comparing public keys
assert_eq!(recovered_pubkey, pubkey);

// Or verify using address
let address = pubkey.address::<WinternitzKeccak>();
```

### Split Signature

```rust
// Split the signature for more efficient verification
let (pairing_hash, prime, execute) = signature.split::<WinternitzKeccak>(message);

// Recover the address using only the prime part
let recovered_address = prime.recover_address::<WinternitzKeccak>(message, &pairing_hash);

// Recover the pairing hash using only the execute part
let recovered_pairing_hash = execute.recover_pairing_hash::<WinternitzKeccak>(message);
```

### Address Handling

```rust
// Get address from public key
let address = pubkey.address::<WinternitzKeccak>();

// Convert address to string
let address_str = address.to_array_string();

// Parse address from string
let parsed_address = WinternitzAddress::try_from(address_str.as_str()).unwrap();
```

## Technical Details

### Structure

- `WinternitzPrivkey`: Private key structure containing 32 32-byte seeds
- `WinternitzPubkey`: Public key structure containing 32 32-byte values
- `WinternitzSignature`: Full signature structure containing 32 32-byte values
- `WinternitzPrimeSignature`: Partial signature containing 28 32-byte values
- `WinternitzExecuteSignature`: Partial signature containing 4 32-byte values
- `WinternitzAddress`: 32-byte address value with Base58 encoding/decoding

### Hash Function Abstraction

The `WinternitzHash` trait allows for flexible hash function selection:

```rust
pub trait WinternitzHash {
    fn hash(msg: &[u8]) -> [u8; 32];
    fn hashd(msg: &[u8]) -> [u8; 32];
    fn hashv(msg: &[&[u8]]) -> [u8; 32];
    fn hash_pair(a: &[u8], b: &[u8]) -> [u8; 32];
}
```

Implemented for both `WinternitzSha256` and `WinternitzKeccak`.

## Security Considerations

1. **One-Time Usage**: Winternitz signatures are one-time signatures. Each private key should only be used once.
2. **Deterministic Signatures**: The implementation produces deterministic signatures based on the message content.
3. **Key Secrecy**: Always keep private keys secure; exposure compromises security.

## Dependencies

- `solana_nostd_sha256`: SHA-256 implementation
- `solana_nostd_keccak`: Keccak implementation
- `arraystring`: Fixed-capacity string implementation for no_std
- `fd_bs58`: Base58 encoding/decoding for addresses
- `bip32`: For hierarchical deterministic key derivation
- `rand`: Random number generation for key creation

## Memory Layout

All key structures use a fixed-size layout with 32-byte arrays:

```rust
#[repr(C)]
pub struct WinternitzPubkey(pub [[u8; 32]; 32]);
```

This ensures consistent memory representation and optimal performance.

## Contributing

When contributing, please ensure:
1. All code is no_std compatible
2. Tests cover new functionality
3. Memory safety is maintained
4. Documentation is updated appropriately

## License

MIT