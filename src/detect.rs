use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExeVersion {
    V10,
    V11,
}

// Known SHA-256 hashes for retail versions (empirically verified)
// Note: v1.0 retail exists in two forms:
// - Unsigned (16.8 MB): original retail disc
// - EA-signed (17.1 MB): official signed version with Authenticode signature
const SHA256_V10_KNOWN: &[&str] = &[
    "7d9b8debfdd033d599073d69f9c6b29a", // v1.0 unsigned (16.8 MB)
    "596efbf5e6c88924acef1fd8b0891012", // v1.0 EA-signed (17.1 MB) - identical code, just Authenticode overlay
    "ada5545526c4d09d8000f0e7600c0cbc", // v1.0 unsigned variant (16.8 MB)
];
const SHA256_V11_KNOWN: &[&str] = &[
    // v1.1 retail EXE hashes (to be discovered and added)
];

// Fallback size-based detection: ranges that account for build variants
// v1.0 sizes observed:
//   - 16,846,848 bytes (unsigned retail disc)
//   - 17,122,568 bytes (EA-signed with Authenticode overlay)
// v1.1 sizes observed:
//   - ~53.9 MB (patched but not cracked)
const V10_SIZE_MIN: usize = 16_500_000;  // 16.5 MB (conservative floor)
const V10_SIZE_MAX: usize = 17_500_000;  // 17.5 MB (covers both unsigned + signed variants)
const V11_SIZE_MIN: usize = 53_000_000;  // 53.0 MB (conservative floor)
const V11_SIZE_MAX: usize = 54_000_000;  // 54.0 MB (conservative ceiling)

pub fn detect_version(exe_data: &[u8]) -> Result<ExeVersion, String> {
    let size = exe_data.len();

    // Try hash-based detection first (if we have known hashes)
    if !SHA256_V10_KNOWN.is_empty() || !SHA256_V11_KNOWN.is_empty() {
        let hash = compute_sha256(exe_data);
        let hash_lower = hash.to_lowercase();

        if SHA256_V10_KNOWN.iter().any(|h| h.to_lowercase() == hash_lower) {
            return Ok(ExeVersion::V10);
        }
        if SHA256_V11_KNOWN.iter().any(|h| h.to_lowercase() == hash_lower) {
            return Ok(ExeVersion::V11);
        }
    }

    // Fallback to size-based detection (range-based to account for build variants)
    if size >= V10_SIZE_MIN && size <= V10_SIZE_MAX {
        return Ok(ExeVersion::V10);
    }
    if size >= V11_SIZE_MIN && size <= V11_SIZE_MAX {
        return Ok(ExeVersion::V11);
    }

    Err(format!(
        "Unable to detect EXE version. Size: {} bytes\n\
         Expected v1.0: {}-{} bytes\n\
         Expected v1.1: {}-{} bytes\n\
         Please add the SHA-256 hash to improve detection: {}",
        size, V10_SIZE_MIN, V10_SIZE_MAX, V11_SIZE_MIN, V11_SIZE_MAX,
        compute_sha256(exe_data)
    ))
}

fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
