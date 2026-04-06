// CONSTANTS & SYSTEM CONFIG
pub const ESP_DIR: [&str;3] = ["EFI/BOOT/","loader/entries/","EFI/ignix/"];
pub const MOUNTPOINTS: &str = "/proc/mounts";
pub const DEVNAME: &str = "DEVNAME=";
pub const DEVTYPE: &str = "DEVTYPE=";
pub const PARTUUID: &str = "PARTUUID=";
pub const EFI_PART_SIGN: [u8;8] = *b"EFI PART";
pub const ESP_GUID_BYTES: [u8;16]= [
    0x28, 0x73, 0x2A, 0xC1,
    0x1F, 0xF8,
    0xD2, 0x11,
    0xBA, 0x4B,
    0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B 
];
// Those values follow the instructions of the GPT standard at 06/04/2026
pub const MAX_GPT_PARTITIONS: usize = 128;
pub const MAX_GPT_PARTITION_ENTRY_SIZE: usize = 128;
pub const MAX_LBA_SECTOR_SIZE: usize = 4096;
pub const MAX_GPT_HEADER_SIZE: usize = 92;
pub const MAX_BUFFER_SIZE: usize = MAX_GPT_PARTITIONS * MAX_GPT_PARTITION_ENTRY_SIZE + MAX_LBA_SECTOR_SIZE;

// GPT HEADER OFFSETS (LBA 1)
// GPT header signature start (Normally "EFI PART")
pub const GPT_HDR_SIG_START: usize = 0;
pub const GPT_HDR_SIG_END: usize = 8;
// GPT header size (normally 92)
pub const GPT_HDR_SIZE_START: usize = 12;
pub const GPT_HDR_SIZE_END: usize = 16;
// CRC32 header location.
pub const GPT_HDR_CRC_START: usize = 16;
pub const GPT_HDR_CRC_END: usize = 20;
// Where does the partition array starts
pub const GPT_HDR_PART_LBA_START: usize = 72;
pub const GPT_HDR_PART_LBA_END: usize = 80;
// How many partitions can the disk have.
pub const GPT_HDR_PART_COUNT_START: usize = 80;
pub const GPT_HDR_PART_COUNT_END: usize = 84;
// Each GPT entry size (normally 128)
pub const GPT_HDR_PART_SIZE_START: usize = 84;
pub const GPT_HDR_PART_SIZE_END: usize = 88;
// CRC32 of the partition array
pub const GPT_HDR_PART_CRC_START: usize = 88;
pub const GPT_HDR_PART_CRC_END: usize = 92;


// PARTITION ENTRY OFFSETS
pub const PART_TYPE_GUID_START: usize = 0;
pub const PART_TYPE_GUID_END: usize = 16;

pub const PART_UNIQUE_GUID_START: usize = 16;
pub const PART_UNIQUE_GUID_END: usize = 32;

// ROUTES
pub const DEFAULT_EFI_BIN_PATH: &str = "/usr/lib/ignix/ignixx64.efi";
pub const BLOCK_DEV_ROUTE: &str = "/sys/class/block/";
pub const LOGICAL_BLOCK_SIZE: &str = "queue/logical_block_size";

// ARGUMENTS  
pub const FORCE_FLAG: &str = "--force";
pub const ALLOW_VIRTUAL_FLAG: &str = "--allow-virtual";
pub const NO_NVRAM: &str = "--no-nvram";
pub const INSTALL_ROUTE: &str = "--install-route=";
pub const EFI_BIN_PATH: &str = "--efi-bin=";
pub const REMOVABLE_FLAG: &str = "--removable";
