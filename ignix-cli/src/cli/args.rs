use std::path::PathBuf;
#[allow(unused)]
pub struct InstallOptions {
    pub force: bool,
    pub allow_virtual: bool,
    pub no_nvram: bool,
    pub removable_device: bool,
    pub efi_bin: PathBuf,
    pub install_route: Option<PathBuf>,
}

pub struct RemoveOptions {
    pub force: bool,
}
