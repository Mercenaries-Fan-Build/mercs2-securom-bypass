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
    eprintln!("[DEBUG] Patch starts with: {:?}", &patch[..std::cmp::min(16, patch.len())]);
    eprintln!("[DEBUG] Old size: {}, Patch size: {}", old.len(), patch.len());

    let mut new = Vec::new();
    match qbsdiff::Bspatch::new(patch) {
        Ok(patcher) => {
            eprintln!("[DEBUG] Patch parsed successfully");
            match patcher.apply(old, Cursor::new(&mut new)) {
                Ok(_) => {
                    eprintln!("[DEBUG] Patch applied, new size: {}", new.len());
                    Ok(new)
                }
                Err(e) => {
                    eprintln!("[DEBUG] Apply error: {} (type: {})", e, std::any::type_name_of_val(&e));
                    Err(format!("Failed to apply {}: {}", description, e))
                }
            }
        }
        Err(e) => {
            eprintln!("[DEBUG] Parse error: {}", e);
            Err(format!("Failed to parse {} patch: {}", description, e))
        }
    }
}
