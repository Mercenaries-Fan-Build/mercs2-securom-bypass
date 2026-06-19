use std::io::Cursor;

// Embedded binary patch files
const PATCH_V10_TO_V11: &[u8] = include_bytes!("../patches/mercs2_v1.0_to_v1.1_update.bspatch");
const PATCH_SECUROM_BYPASS: &[u8] = include_bytes!("../patches/mercs2_v1.1_securom_bypass.bspatch");

pub fn apply_v10_to_v11_patch(exe_data: &[u8]) -> Result<Vec<u8>, String> {
    apply_patch(exe_data, PATCH_V10_TO_V11, "v1.0 → v1.1 update")
}

pub fn apply_securom_bypass_patch(exe_data: &[u8]) -> Result<Vec<u8>, String> {
    apply_patch(exe_data, PATCH_SECUROM_BYPASS, "SecuROM bypass")
}

fn apply_patch(old: &[u8], patch: &[u8], description: &str) -> Result<Vec<u8>, String> {
    let mut new = Vec::new();
    qbsdiff::Bspatch::new(patch)
        .map_err(|e| format!("Failed to parse {} patch: {}", description, e))?
        .apply(old, Cursor::new(&mut new))
        .map_err(|e| format!("Failed to apply {}: {}", description, e))?;
    Ok(new)
}
