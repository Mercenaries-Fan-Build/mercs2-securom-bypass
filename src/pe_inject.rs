// PE Header constants
const DOS_HEADER_SIZE: usize = 64;
const PE_SIGNATURE_OFFSET: usize = 0x3C;
const PE_SIGNATURE: &[u8] = b"PE\0\0";
const OPTIONAL_HEADER_OFFSET: usize = 24;

// IMAGE_IMPORT_DESCRIPTOR structure
#[repr(C)]
struct ImportDescriptor {
    name_rva: u32,
    time_date_stamp: u32,
    forwarder_chain: u32,
    name_rva_iat: u32,
    first_thunk: u32,
}

impl ImportDescriptor {
    fn new(name_rva: u32, first_thunk: u32) -> Self {
        ImportDescriptor {
            name_rva,
            time_date_stamp: 0,
            forwarder_chain: 0xFFFFFFFF,
            name_rva_iat: first_thunk,
            first_thunk,
        }
    }

    fn as_bytes(&self) -> [u8; 20] {
        let mut bytes = [0u8; 20];
        bytes[0..4].copy_from_slice(&self.name_rva.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.time_date_stamp.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.forwarder_chain.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.name_rva_iat.to_le_bytes());
        bytes[16..20].copy_from_slice(&self.first_thunk.to_le_bytes());
        bytes
    }
}

pub fn inject_pmc_bb_dll(exe_data: &[u8]) -> Result<Vec<u8>, String> {
    let mut data = exe_data.to_vec();

    // Parse PE header
    if data.len() < DOS_HEADER_SIZE {
        return Err("EXE too small for PE header".to_string());
    }

    // Read PE offset from DOS header
    let pe_offset = u32::from_le_bytes([
        data[PE_SIGNATURE_OFFSET],
        data[PE_SIGNATURE_OFFSET + 1],
        data[PE_SIGNATURE_OFFSET + 2],
        data[PE_SIGNATURE_OFFSET + 3],
    ]) as usize;

    if pe_offset + 4 + OPTIONAL_HEADER_OFFSET + 8 > data.len() {
        return Err("Invalid PE header offset".to_string());
    }

    // Verify PE signature
    if &data[pe_offset..pe_offset + 4] != PE_SIGNATURE {
        return Err("Invalid PE signature".to_string());
    }

    // Get Optional Header magic to determine 32-bit vs 64-bit
    let magic_offset = pe_offset + OPTIONAL_HEADER_OFFSET;
    if magic_offset + 2 > data.len() {
        return Err("Cannot read Optional Header magic".to_string());
    }
    let magic = u16::from_le_bytes([data[magic_offset], data[magic_offset + 1]]);

    // Data Directories offset varies by architecture
    // PE32 (0x10b): Data Directories start at offset 96
    // PE32+ (0x20b): Data Directories start at offset 112
    let data_dir_offset = match magic {
        0x10b => 96,  // PE32 (32-bit)
        0x20b => 112, // PE32+ (64-bit)
        _ => return Err(format!("Unknown PE magic: 0x{:x}", magic)),
    };

    // Get the import table RVA from Optional Header Data Directories
    let import_table_rva_offset = pe_offset + OPTIONAL_HEADER_OFFSET + data_dir_offset;
    if import_table_rva_offset + 4 > data.len() {
        return Err("Invalid import table offset in Optional Header".to_string());
    }

    let import_table_rva = u32::from_le_bytes([
        data[import_table_rva_offset],
        data[import_table_rva_offset + 1],
        data[import_table_rva_offset + 2],
        data[import_table_rva_offset + 3],
    ]) as usize;

    if import_table_rva == 0 {
        return Err("No import table found in executable (RVA is 0)".to_string());
    }

    // Find the section containing the import table
    let sections_offset = pe_offset + 24 + 20; // After PE signature + COFF header
    let num_sections = u16::from_le_bytes([
        data[pe_offset + 6],
        data[pe_offset + 7],
    ]) as usize;

    let mut import_section_offset = None;
    for i in 0..num_sections {
        let section_offset = sections_offset + i * 40;
        if section_offset + 40 > data.len() {
            break;
        }

        let virt_addr = u32::from_le_bytes([
            data[section_offset + 12],
            data[section_offset + 13],
            data[section_offset + 14],
            data[section_offset + 15],
        ]) as usize;

        let virt_size = u32::from_le_bytes([
            data[section_offset + 8],
            data[section_offset + 9],
            data[section_offset + 10],
            data[section_offset + 11],
        ]) as usize;

        let raw_size = u32::from_le_bytes([
            data[section_offset + 16],
            data[section_offset + 17],
            data[section_offset + 18],
            data[section_offset + 19],
        ]) as usize;

        let raw_offset = u32::from_le_bytes([
            data[section_offset + 20],
            data[section_offset + 21],
            data[section_offset + 22],
            data[section_offset + 23],
        ]) as usize;

        if virt_addr <= import_table_rva && import_table_rva < virt_addr + virt_size {
            import_section_offset = Some((raw_offset, virt_addr, raw_size));
            break;
        }
    }

    let (raw_offset, virt_addr, _raw_size) = import_section_offset
        .ok_or_else(|| "Could not find import table section".to_string())?;

    let import_offset = raw_offset + (import_table_rva - virt_addr);

    // Find the end of the import table (look for NULL descriptor)
    let mut descriptor_offset = import_offset;
    let mut descriptor_count = 0;

    loop {
        if descriptor_offset + 20 > data.len() {
            return Err("Import table extends beyond file".to_string());
        }

        // Check if this is a NULL descriptor
        if data[descriptor_offset..descriptor_offset + 20].iter().all(|&b| b == 0) {
            break;
        }

        descriptor_offset += 20;
        descriptor_count += 1;

        // Safety limit
        if descriptor_count > 1000 {
            return Err("Import table too large".to_string());
        }
    }

    // Append new import descriptor for pmc_bb.dll
    let dll_name = b"pmc_bb.dll\0";
    let new_descriptor = ImportDescriptor::new(
        (import_table_rva + (descriptor_offset - import_offset) + 20 + 20) as u32, // RVA to name
        0x400000 + (descriptor_offset - import_offset) as u32 + 20 + 20 + dll_name.len() as u32 + 4, // First thunk
    );

    data.extend_from_slice(&new_descriptor.as_bytes());
    // Add NULL descriptor
    data.extend_from_slice(&[0u8; 20]);
    // Add DLL name string
    data.extend_from_slice(dll_name);
    // Add import entry (ordinal #1)
    data.extend_from_slice(&[0x00, 0x80, 0x00, 0x00]); // Ordinal 1 with high bit set
    // Add NULL terminator for IAT
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

    Ok(data)
}
