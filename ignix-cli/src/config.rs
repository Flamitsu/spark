// This is the archive where all the config of the program resides. 

// This constants are being used in gpt.rs at this moment
/// EFI part signature in array of integer bytes.
pub const EFI_PART_SIGN: [u8;8] = *b"EFI PART";
/// GPT max header size. Is duplicated from the std one. (92*2)
pub const MAX_HEADER_SIZE: usize = 184;
/// GPT max partition array size. Is 128 bytes per entry and 128 bytes the std of GPT.
const MAX_PARTITION_ARRAY_SIZE: usize = 16384;

pub const MAX_BUFFER_SIZE: usize = MAX_HEADER_SIZE + MAX_PARTITION_ARRAY_SIZE;

/// Default efi bin path that defines where the EFI binary should be located (default and fallback)
pub const DEFAULT_EFI_BIN_PATH: &str = "/usr/lib/ignix/ignixx64.efi";

/// This flag is what the user should put as an argument to give a distinct EFI bin path.
pub const EFI_BIN_PATH: &str = "--efi-bin=";

/// Defines the route that looks up the program to search for real block devices.
pub const BLOCK_DEV_ROUTE: &str = "/sys/class/block/";

/// Defines the route '/sys/class/block/{device}/queue/logical_block_size' with format to get the sector block size
pub const LOGICAL_BLOCK: &str = "/queue/logical_block_size";

/// Neccesary for the confirmation flags in the CLI usage of the tool while the install process. 
pub const SHORT_CONFIRMATION_FLAG: &str = "-y";

/// Neccesary for the confirmation flags in the CLI usage of the tool while the install process.
pub const LONG_CONFIRMATION_FLAG: &str = "--yes";

/// Flag to allow the program to install in a virtual device.
pub const ALLOW_VIRTUAL_FLAG: &str = "--allow-virtual";

/// Flag argument to remove the neccesary NVRAM write variable.
pub const NO_NVRAM: &str = "--no-nvram";

/// Flag argument to say which route should the program be installed in.
pub const INSTALL_ROUTE: &str = "--install-route=";

/// Flag argument to install the bootloader in a removable device.
pub const REMOVABLE_FLAG: &str = "--removable";
