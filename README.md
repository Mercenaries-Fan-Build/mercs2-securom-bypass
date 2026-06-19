# mercs2-crack-game

**apply_crack** — a standalone tool to patch retail Mercenaries 2 EXEs for offline play.

## What it does

- **Detects EXE version:** Identifies your EXE by SHA-256 — v1.0 unsigned (disc), v1.0 EA-signed, v1.1 update, or an already-cracked binary.
- **Applies v1.0 → v1.1 update** (if needed): Brings either v1.0 variant up to the canonical v1.1 retail build.
- **Strips SecuROM + swaps the DLL import:** A single bypass patch removes DRM validation and rewrites the import table to load [pmc_bb.dll](https://github.com/Mercenaries-Fan-Build/pmc-blackbox/releases) (replacing SecuROM's cruise.dll) in one step.
- **Outputs cracked EXE:** Writes the result as `Mercenaries2-cracked.exe` (or your chosen output path).

All patches are embedded in the binary and applied with a pure-Rust bsdiff
patcher ([qbsdiff](https://crates.io/crates/qbsdiff)) — **nothing to pre-install.**
Every patch is verified to reproduce an exact known-good SHA-256, so the output
is byte-for-byte identical to a hand-verified working build.

## Installation

Download the latest binary from [GitHub Releases](https://github.com/austinkregel/mercs2-securom-bypass/releases) or build from source.

## Usage

```bash
apply_crack /path/to/Mercenaries2.exe --output /path/to/Mercenaries2-cracked.exe
```

Or use the default output path (same directory, named `Mercenaries2-cracked.exe`):

```bash
apply_crack /path/to/Mercenaries2.exe
```

The correct patch chain is selected automatically from the detected version —
a v1.1 input skips the update step, and an already-cracked input is a no-op.

## Building from source

### Prerequisites

- Rust 1.70+ ([rustup](https://rustup.rs/))

### Build

```bash
cargo build --release
```

Output: `target/release/apply_crack` (or `apply_crack.exe` on Windows).

## Installation guide

1. Obtain a copy of the retail game (v1.0 or v1.1).
2. Run `apply_crack` on your `Mercenaries2.exe` to produce `Mercenaries2-cracked.exe`.
3. Download [pmc_bb.dll](https://github.com/Mercenaries-Fan-Build/pmc-blackbox/releases) and place it next to the cracked EXE.
4. Replace your game's `Mercenaries2.exe` with the cracked version.
5. Run the game!

The game should now start without SecuROM.

## Troubleshooting

**"Unable to detect EXE version"**
- Your EXE doesn't match any known v1.0/v1.1/cracked build. If the size matches but the SHA-256 doesn't, it's an unknown variant and the tool will warn that patching may corrupt it — use an unmodified retail EXE.

**"Failed to apply v1.0 → v1.1 update" / "Failed to apply SecuROM bypass"**
- The patch was incompatible with your EXE. Ensure you're using an unmodified retail EXE that matches a known SHA-256.

**Game crashes on startup**
- Ensure `pmc_bb.dll` is in the same directory as the EXE and is not corrupted. Check `pmc_blackbox.log` for error details.

## See also

- [mercs2-pmc-blackbox](https://github.com/Mercenaries-Fan-Build/pmc-blackbox/releases) — The DLL that handles SecuROM spoofing and game startup.
- [mercs2-wad-simulator](https://github.com/Mercenaries-Fan-Build/mercs2-wad-simulator) — A Rust workspace for Mercenaries 2 WAD analysis, asset extraction, and Xbox-to-PC conversion.
