use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod detect;
mod patch;
mod pe_inject;

use detect::detect_version;
use patch::{apply_v10_to_v11_patch, apply_securom_bypass_patch};
use pe_inject::inject_pmc_bb_dll;

#[derive(Parser, Debug)]
#[command(name = "apply_crack")]
#[command(about = "Apply SecuROM crack to Mercenaries 2 retail EXE", long_about = None)]
struct Args {
    /// Path to retail Mercenaries2.exe (v1.0 or v1.1)
    #[arg(value_name = "FILE")]
    exe_path: PathBuf,

    /// Output path for cracked EXE (default: same dir as input, named Mercenaries2-cracked.exe)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Skip the v1.0 → v1.1 update (assume EXE is already v1.1)
    #[arg(long)]
    skip_update: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Verify input exists
    if !args.exe_path.exists() {
        eprintln!("error: EXE not found: {}", args.exe_path.display());
        std::process::exit(1);
    }

    // Detect version
    let exe_data = fs::read(&args.exe_path)?;
    let version = detect_version(&exe_data)?;

    eprintln!("[*] Detected version: {}", match version {
        detect::ExeVersion::V10 => "v1.0",
        detect::ExeVersion::V11 => "v1.1",
    });

    let mut patched = exe_data.clone();

    // Step 1: Update v1.0 → v1.1 if needed
    if version == detect::ExeVersion::V10 && !args.skip_update {
        eprintln!("[*] Applying v1.0 → v1.1 update patch...");
        patched = apply_v10_to_v11_patch(&patched)?;
        eprintln!("[+] v1.0 → v1.1 update applied");
    }

    // Step 2: Apply SecuROM bypass
    eprintln!("[*] Applying SecuROM bypass patch...");
    patched = apply_securom_bypass_patch(&patched)?;
    eprintln!("[+] SecuROM bypass applied");

    // Step 3: Inject pmc_bb.dll into import table
    eprintln!("[*] Injecting pmc_bb.dll into import table...");
    patched = inject_pmc_bb_dll(&patched)?;
    eprintln!("[+] pmc_bb.dll injected");

    // Write output
    let output_path = args.output.unwrap_or_else(|| {
        let mut path = args.exe_path.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();
        path.push("Mercenaries2-cracked.exe");
        path
    });

    fs::write(&output_path, &patched)?;
    eprintln!("[+] Cracked EXE written to: {}", output_path.display());
    eprintln!("\nSuccess! Next steps:");
    eprintln!("  1. Copy pmc_bb.dll next to the cracked EXE");
    eprintln!("  2. Copy the cracked EXE over your game's Mercenaries2.exe");
    eprintln!("  3. Run the game!");

    Ok(())
}
