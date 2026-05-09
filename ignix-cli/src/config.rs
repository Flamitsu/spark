use std::ops::Range;
/*
 * The structure of this archive is: Structs and later the constants with the associated value.
 * Its done this way because Rust does not implements in this date the capability to associate 
 * values directly into the struct so it's a "default."
 *
 * If you wonder why all the config is here, is because the GPT standard does not change but
 * diabolical disks can appear. So I left this here centralized so its easier to patch.
 * 
 * The structure of this archive is normally: comment -> data. This is because in some IDEs you 
 * can have the autocompletion and it will show up the comment that references to that value.
*/

pub struct BLSConfigurations{
    pub esp_directories: [&'static str;3]
}

pub const BLS_CONFIG: BLSConfigurations = BLSConfigurations {
    esp_directories: ["EFI/BOOT", "loader/entries", "EFI/ignix/"]
};

pub struct DevLinuxTags{
    pub devname: &'static str,
    pub devtype: &'static str,
    pub partuuid: &'static str
}
pub const DEV_LINUX_TAGS: DevLinuxTags = DevLinuxTags{
    devname: "DEVNAME=",
    devtype: "DEVTYPE=",
    partuuid: "PARTUUID="
};

pub struct GptSpecification {
    pub efi_sig: [u8;8],
    pub esp_guid_sig: [u8;16]
}
pub const GPT_SPEC: GptSpecification = GptSpecification{
    efi_sig: *b"EFI PART",
    esp_guid_sig: [0x28, 0x73, 0x2A, 0xC1,
    0x1F, 0xF8,
    0xD2, 0x11,
    0xBA, 0x4B,
    0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B]
};
// Those values follow the instructions of the GPT std at 18/04/2026.
pub struct GptLimits{
    pub gpt_partitions: usize,
    pub partition_entry_size: usize,
    pub lba_sector_size: usize,
    pub header_size: usize,
    pub buffer_size: usize,
    pub header_part_lba: usize,
    pub header_part_size: usize
}
/*
 * Why does this have a constructor and the other ones do not? 
 * It's because parameters like the buffer size should be
 * calculated with the current config and not be hardcoded. You give the gpt_partitions (normally
 * 128), the entry_size (normally 128) and the lba_sector_size (normally 4096 or 512) */
impl GptLimits {
    const fn new(gpt_partitions: usize, partition_entry_size: usize, lba_sector_size: usize) -> Self{
        let buffer_size = (gpt_partitions * partition_entry_size) + lba_sector_size;
        Self {
            gpt_partitions,
            partition_entry_size,
            lba_sector_size,
            header_size: 92,
            buffer_size,
            header_part_lba: 2,
            header_part_size: 128
        }
    }
}

const LIMITS: GptLimits = GptLimits::new(128, 128, 4096);

// GPT HEADER OFFSETS (LBA 1)
pub struct GptHeaderOffsets{
    // GPT header signature start (Normally "EFI PART" in Little Endian)
    pub sig: Range<usize>,
    // GPT header size (normally 92)
    pub size: Range<usize>,
    // CRC32 bytes location. 
    pub crc: Range<usize>,
    // Where does the partition array starts
    pub part_lba: Range<usize>,
    // How many partitions can the disk have.
    pub part_count: Range<usize>,
    // Each GPT entry size (normally is 128 bytes).
    pub part_size: Range<usize>,
    // CRC32 of the partition array.
    pub part_crc: Range<usize>,
}

pub const HEADER: GptHeaderOffsets = GptHeaderOffsets{
    sig: 0..8,
    size: 12..16,
    crc: 16..20,
    part_lba: 72..80,
    part_count: 80..84,
    part_size: 84..88,
    part_crc: 88..92
};

pub struct GptEntryOffsets{
    // Partition type GUID (Linux Root or Linux x86 for example).
    pub type_guid: Range<usize>,
    // Partition unique GUID (UUID v4 unique per partition).
    pub unique_guid: Range<usize>
}
pub const PARTITION: GptEntryOffsets = GptEntryOffsets {
    type_guid: 0..16,
    unique_guid: 16..32
};

pub struct Routes;
impl Routes{
    // route: /usr/lib/ignix/ignix{arch}.efi
    pub const DEFAULT_EFI_BIN_PATH: &'static str = "/usr/lib/ignix/ignixx64.efi";
    // route: /sys/class/block/
    pub const BLOCK_DEV_ROUTE: &'static str = "/sys/class/block/";
    // route: queue/logical_block_size
    pub const LOGICAL_BLOCK_SIZE: &'static str = "queue/logical_block_size";
    // route: /proc/mounts
    pub const MOUNTPOINTS: &'static str = "/proc/mounts";
}

pub struct Flag;

impl Flag {
    pub const FORCE_FLAG: &'static str = "--force";
    pub const ALLOW_VIRTUAL_FLAG: &'static str = "--allow-virtual";
    pub const NO_NVRAM: &'static str = "--no-nvram";
    pub const INSTALL_ROUTE: &'static str = "--install-route=";
    pub const EFI_BIN_PATH: &'static str = "--efi-bin=";
    pub const REMOVABLE_FLAG: &'static str = "--removable";
}
