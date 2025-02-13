# Winternitz Signature Implementation

An implementation of the Winternitz One-Time Signature (WOTS) scheme, designed for use with Solana. This implementation supports both SHA-256 and Keccak hash functions.

## Features

- Support for multiple hash functions (SHA-256 and Keccak)
- Optimized Merkle tree computation
- Memory-safe implementation using `MaybeUninit` and `transmute`
- Fixed-size signatures and public keys

## Usage

### Key Generation

```rust
use winternitz::{WinternitzPrivkey, Hash, Sha256};

// Generate a new private key
let privkey = WinternitzPrivkey::generate();

// Derive the corresponding public key
let pubkey = privkey.pubkey::<Sha256>();
```

### Signing Messages

```rust
// Sign a message
let message = b"Hello, World!";
let signature = privkey.sign::<Sha256>(message);
```

### Signature Verification

```rust
// Verify the signature
let is_valid = signature.verify::<Sha256>(message, &pubkey);
assert!(is_valid);
```

### Public Key Recovery

```rust
// Recover public key from signature and message
let recovered_pubkey = signature.recover_pubkey::<Sha256>(message);
assert_eq!(recovered_pubkey, pubkey);
```

### Merkle Tree Computation

```rust
// Compute the Merkle root of a public key
let merkle_root = pubkey.merklize();
```

## Technical Details

### Structure

- `WinternitzPrivkey`: Private key structure containing 32 hash-length seeds
- `WinternitzPubkey`: Public key structure containing 32 hash-length values
- `WinternitzSignature`: Signature structure containing 32 hash-length values
- `Hash`: Trait for hash function implementations (SHA-256 and Keccak)

### Constants

- `HASH_LENGTH`: Length of hash output (32 bytes)
- `KEY_LENGTH`: Total length of key material (1024 bytes)

### Memory Safety

The implementation uses `MaybeUninit` for safe handling of uninitialized memory in performance-critical operations. All unsafe code blocks are carefully documented and contained within safe abstractions.

### Hash Function Abstraction

The `Hash` trait allows for flexible hash function selection:

```rust
pub trait Hash {
    fn hash(data: &[u8]) -> [u8; HASH_LENGTH];
    fn hashv(data: &[&[u8]]) -> [u8; HASH_LENGTH];
}
```

Implemented for both SHA-256 and Keccak.

## Security Considerations

1. **One-Time Usage**: Winternitz signatures are one-time signatures. Each private key should only be used once.
2. **Message Length**: The implementation is optimized for 32-byte messages (typical hash outputs).
3. **Random Number Generation**: Key generation relies on the system's random number generator.

## Dependencies

- `solana_nostd_sha256`: SHA-256 implementation
- `solana_nostd_keccak`: Keccak implementation
- `rand`: Random number generation
- `core`: No-std compatible core library

## Performance Optimizations

1. Efficient Merkle tree computation using quarter and eighth tree optimizations
2. Minimal memory allocations using fixed-size arrays
3. Optimized hash chaining operations

## Memory Layout

All key structures use a fixed-size layout of 32 arrays, each containing `HASH_LENGTH` bytes:

```rust
#[repr(C)]
pub struct WinternitzSignature {
    data: [[u8; HASH_LENGTH]; 32]
}
```

This ensures consistent memory representation and optimal performance.

## Contributing

When contributing, please ensure:
1. All code is no-std compatible
2. Tests cover new functionality
3. Memory safety is maintained
4. Documentation is updated appropriately

## License

MIT