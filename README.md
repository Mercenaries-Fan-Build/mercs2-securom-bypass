# mercs2-crack-game

**apply_crack** — a standalone tool to patch retail Mercenaries 2 EXEs for offline play.

## What it does

- **Detects EXE version:** Identifies whether your EXE is v1.0 (disc) or v1.1 (update).
- **Applies v1.0 → v1.1 update** (if needed): Uses the bsdiff v1.0-to-v1.1 retail patch to bring disc EXEs up to date.
- **Strips SecuROM:** Applies the bsdiff SecuROM bypass patch, removing DRM validation.
- **Injects pmc_bb.dll:** Modifies the EXE's import table to load [pmc_bb.dll](https://github.com/Mercenaries-Fan-Build/pmc-blackbox/releases) on startup — replacing SecuROM's cruise.dll.
- **Outputs cracked EXE:** Writes the result as `Mercenaries2-cracked.exe` (or your chosen output path).

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

Skip the v1.0 update if your EXE is already v1.1:

```bash
apply_crack /path/to/Mercenaries2.exe --skip-update
```

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
- Your EXE is neither v1.0 nor v1.1, or is corrupted. Verify the file size and hash.

**"Failed to apply v1.0 → v1.1 update"**
- The patch was incompatible with your EXE. Ensure you have the original retail v1.0 disc EXE.

**"Failed to apply SecuROM bypass"**
- The patch was incompatible. Verify you're using an unmodified retail EXE.

**Game crashes on startup**
- Ensure `pmc_bb.dll` is in the same directory as the EXE and is not corrupted. Check `pmc_blackbox.log` for error details.

## See also

- [mercs2-pmc-blackbox](https://github.com/Mercenaries-Fan-Build/pmc-blackbox/releases) — The DLL that handles SecuROM spoofing and game startup.
- [mercs2-wad-simulator](https://github.com/Mercenaries-Fan-Build/mercs2-wad-simulator) — A Rust workspace for Mercenaries 2 WAD analysis, asset extraction, and Xbox-to-PC conversion.
