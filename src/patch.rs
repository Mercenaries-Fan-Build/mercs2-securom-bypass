use std::io::Cursor;

// Embedded binary patch files (all generated with qbsdiff, so the embedded
// qbsdiff patcher applies them byte-for-byte — no external bspatch needed).
//
// Both v1.0 variants converge on the identical canonical v1.1 (5b9976f1...),
// after which a single v1.1 -> cracked patch carries the SecuROM bypass AND the
// pmc_bb.dll import rename baked in. Each patch is verified to reproduce an
// exact known-good SHA-256 (see detect.rs).
const PATCH_V10_UNSIGNED_TO_V11: &[u8] = include_bytes!("../patches/v10_unsigned_to_v11.bspatch");
const PATCH_V10_SIGNED_TO_V11: &[u8] = include_bytes!("../patches/v10_signed_to_v11.bspatch");
const PATCH_V11_TO_CRACKED: &[u8] = include_bytes!("../patches/v11_to_cracked.bspatch");

pub fn apply_v10_unsigned_to_v11_patch(exe_data: &[u8]) -> Result<Vec<u8>, String> {
    apply_patch(exe_data, PATCH_V10_UNSIGNED_TO_V11, "v1.0 (unsigned) → v1.1 update")
}

pub fn apply_v10_signed_to_v11_patch(exe_data: &[u8]) -> Result<Vec<u8>, String> {
    apply_patch(exe_data, PATCH_V10_SIGNED_TO_V11, "v1.0 (signed) → v1.1 update")
}

pub fn apply_securom_bypass_patch(exe_data: &[u8]) -> Result<Vec<u8>, String> {
    apply_patch(exe_data, PATCH_V11_TO_CRACKED, "SecuROM bypass + pmc_bb.dll")
}

fn apply_patch(old: &[u8], patch: &[u8], description: &str) -> Result<Vec<u8>, String> {
    let mut new = Vec::new();
    qbsdiff::Bspatch::new(patch)
        .map_err(|e| format!("Failed to parse {} patch: {}", description, e))?
        .apply(old, Cursor::new(&mut new))
        .map_err(|e| format!("Failed to apply {}: {}", description, e))?;
    Ok(new)
}
