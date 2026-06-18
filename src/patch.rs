use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

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
    // Create temporary files for bspatch
    let mut old_file = NamedTempFile::new()
        .map_err(|e| format!("Failed to create temp file for old EXE: {}", e))?;
    old_file.write_all(old)
        .map_err(|e| format!("Failed to write old EXE to temp file: {}", e))?;

    let mut patch_file = NamedTempFile::new()
        .map_err(|e| format!("Failed to create temp file for patch: {}", e))?;
    patch_file.write_all(patch)
        .map_err(|e| format!("Failed to write patch to temp file: {}", e))?;

    let new_file = NamedTempFile::new()
        .map_err(|e| format!("Failed to create temp file for new EXE: {}", e))?;

    // Try to find and run bspatch command
    let status = Command::new("bspatch")
        .arg(old_file.path())
        .arg(new_file.path())
        .arg(patch_file.path())
        .status()
        .map_err(|_| {
            "bspatch command not found. Install bsdiff (e.g., 'apt install bsdiff' on Ubuntu)".to_string()
        })?;

    if !status.success() {
        return Err(format!("Failed to apply {}: bspatch exited with status {}", description, status));
    }

    // Read the result
    std::fs::read(new_file.path())
        .map_err(|e| format!("Failed to read patched EXE: {}", e))
}
