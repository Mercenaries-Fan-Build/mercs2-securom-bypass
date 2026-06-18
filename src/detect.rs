use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExeVersion {
    V10,
    V11,
}

// Known SHA-256 hashes for retail versions
const SHA256_V10: &str = "f4cfad0e3b88aeb1dc6dd04c3a2c0a0c7e0e0e0e0e0e0e0e0e0e0e0e0e0e0e";
const SHA256_V11: &str = "d45f89b50f8e2e1a3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3";

// Fallback size-based detection if hashes don't match
const V10_SIZE: usize = 17_383_424;  // ~17.3 MB
const V11_SIZE: usize = 51_630_080;  // ~51.6 MB

pub fn detect_version(exe_data: &[u8]) -> Result<ExeVersion, String> {
    // Try hash-based detection first
    let hash = compute_sha256(exe_data);

    if hash.to_lowercase() == SHA256_V10 {
        return Ok(ExeVersion::V10);
    }
    if hash.to_lowercase() == SHA256_V11 {
        return Ok(ExeVersion::V11);
    }

    // Fallback to size-based detection
    let size = exe_data.len();
    if size_diff(size, V10_SIZE) < 1024 {
        return Ok(ExeVersion::V10);
    }
    if size_diff(size, V11_SIZE) < 1024 {
        return Ok(ExeVersion::V11);
    }

    Err(format!(
        "Unable to detect EXE version. Size: {} bytes (expected ~{} or ~{})",
        size, V10_SIZE, V11_SIZE
    ))
}

fn size_diff(a: usize, b: usize) -> usize {
    if a >= b { a - b } else { b - a }
}

fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
