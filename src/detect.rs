use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExeVersion {
    /// v1.0 retail, unsigned (16,846,848 bytes — original disc)
    V10Unsigned,
    /// v1.0 retail, EA-signed (17,122,568 bytes — Authenticode overlay)
    V10Signed,
    /// v1.1 retail full update (53,944,080 bytes)
    V11,
    /// Already cracked (53,482,288 bytes — imports pmc_bb.dll)
    Cracked,
}

// Known SHA-256 hashes (empirically verified against storage/ corpus).
const SHA256_V10_UNSIGNED: &str =
    "ada5545526c4d09d8000f0e7600c0cbc24b7b360638fa2dde407cd7f976534d7";
const SHA256_V10_SIGNED: &str =
    "a1532b4c7652fe9feee1191f5bd04aa073cd0f036e49831c754e7d895241dfa8";
const SHA256_V11: &str =
    "7a348847e103d71e8c17e7a51a0f3b4d4422e0c9cb46ec6acc9fe5e4e6be36b5";
const SHA256_CRACKED: &str =
    "958eb22776067c2dbb7d684e472c5045d419ec0ecfb49bfea7d23fcf4a83f115";

// Fallback size-based detection (used only when the hash is unrecognized).
const SIZE_V10_UNSIGNED: usize = 16_846_848;
const SIZE_V10_SIGNED: usize = 17_122_568;
const SIZE_V11: usize = 53_944_080;
const SIZE_CRACKED: usize = 53_482_288;

pub fn detect_version(exe_data: &[u8]) -> Result<ExeVersion, String> {
    let hash = compute_sha256(exe_data);

    // Hash-based detection (exact, preferred).
    match hash.as_str() {
        SHA256_V10_UNSIGNED => return Ok(ExeVersion::V10Unsigned),
        SHA256_V10_SIGNED => return Ok(ExeVersion::V10Signed),
        SHA256_V11 => return Ok(ExeVersion::V11),
        SHA256_CRACKED => return Ok(ExeVersion::Cracked),
        _ => {}
    }

    // Size-based fallback for unrecognized builds (regional variants, etc.).
    // Patches are byte-specific, so this is best-effort and will warn.
    let size = exe_data.len();
    let guess = match size {
        SIZE_V10_UNSIGNED => Some(ExeVersion::V10Unsigned),
        SIZE_V10_SIGNED => Some(ExeVersion::V10Signed),
        SIZE_V11 => Some(ExeVersion::V11),
        SIZE_CRACKED => Some(ExeVersion::Cracked),
        _ => None,
    };

    if let Some(v) = guess {
        eprintln!(
            "[!] Unrecognized SHA-256 ({}) but size matches {:?}.\n\
             [!] This is an unknown build — patching may produce a corrupt EXE.",
            hash, v
        );
        return Ok(v);
    }

    Err(format!(
        "Unable to detect EXE version.\n  size: {} bytes\n  sha256: {}\n\
         Known inputs:\n\
         \x20 v1.0 unsigned: {} bytes\n\
         \x20 v1.0 signed:   {} bytes\n\
         \x20 v1.1 retail:   {} bytes\n\
         \x20 cracked:       {} bytes",
        size, hash, SIZE_V10_UNSIGNED, SIZE_V10_SIGNED, SIZE_V11, SIZE_CRACKED
    ))
}

fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
