use crate as soroban_sdk;
pub const WASM: &[u8] = soroban_sdk::contractfile!(
    file = "../target/wasm32-unknown-unknown/release/test_add_u64.wasm",
    sha256 = "3dcf26623184e64fd38dbfbb9715c3e1ed5aabf6f9910950a2890357119ca967",
);

#[test]
fn test_spec() {
    assert!(WASM.len() > 0);
}
