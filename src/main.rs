use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod detect;
mod patch;

use detect::ExeVersion;
use patch::{
    apply_securom_bypass_patch, apply_v10_signed_to_v11_patch, apply_v10_unsigned_to_v11_patch,
};

#[derive(Parser, Debug)]
#[command(name = "apply_crack")]
#[command(about = "Apply SecuROM crack to Mercenaries 2 retail EXE", long_about = None)]
struct Args {
    /// Path to retail Mercenaries2.exe (v1.0 signed/unsigned, or v1.1)
    #[arg(value_name = "FILE")]
    exe_path: PathBuf,

    /// Output path for cracked EXE (default: <input dir>/Mercenaries2-cracked.exe)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !args.exe_path.exists() {
        eprintln!("error: EXE not found: {}", args.exe_path.display());
        std::process::exit(1);
    }

    let exe_data = fs::read(&args.exe_path)?;
    let version = detect::detect_version(&exe_data)?;

    eprintln!(
        "[*] Detected version: {}",
        match version {
            ExeVersion::V10Unsigned => "v1.0 (unsigned retail)",
            ExeVersion::V10Signed => "v1.0 (EA-signed retail)",
            ExeVersion::V11 => "v1.1 (retail update)",
            ExeVersion::Cracked => "already cracked",
        }
    );

    if version == ExeVersion::Cracked {
        eprintln!("[+] This EXE is already cracked — nothing to do.");
        return Ok(());
    }

    // Step 1: bring any v1.0 variant up to the canonical v1.1.
    let v11 = match version {
        ExeVersion::V10Unsigned => {
            eprintln!("[*] Applying v1.0 (unsigned) → v1.1 update patch...");
            let out = apply_v10_unsigned_to_v11_patch(&exe_data)?;
            eprintln!("[+] Updated to v1.1");
            out
        }
        ExeVersion::V10Signed => {
            eprintln!("[*] Applying v1.0 (signed) → v1.1 update patch...");
            let out = apply_v10_signed_to_v11_patch(&exe_data)?;
            eprintln!("[+] Updated to v1.1");
            out
        }
        ExeVersion::V11 => exe_data,
        ExeVersion::Cracked => unreachable!(),
    };

    // Step 2: apply the SecuROM bypass. The pmc_bb.dll import rename is baked
    // into this patch, so the output is the final, ready-to-run cracked EXE.
    eprintln!("[*] Applying SecuROM bypass patch...");
    let cracked = apply_securom_bypass_patch(&v11)?;
    eprintln!("[+] SecuROM bypass applied (pmc_bb.dll import baked in)");

    let output_path = args.output.unwrap_or_else(|| {
        let mut path = args
            .exe_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_path_buf();
        path.push("Mercenaries2-cracked.exe");
        path
    });

    fs::write(&output_path, &cracked)?;
    eprintln!("[+] Cracked EXE written to: {}", output_path.display());
    eprintln!("\nSuccess! Next steps:");
    eprintln!("  1. Copy pmc_bb.dll next to the cracked EXE");
    eprintln!("  2. Copy the cracked EXE over your game's Mercenaries2.exe");
    eprintln!("  3. Run the game!");

    Ok(())
}
