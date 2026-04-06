use crate::config::{ALLOW_VIRTUAL_FLAG, DEFAULT_EFI_BIN_PATH, EFI_BIN_PATH, INSTALL_ROUTE, FORCE_FLAG, NO_NVRAM, REMOVABLE_FLAG};
use crate::errors::{IgnixError, io, cmd};
use std::io::{Write, stdin,stdout};
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

pub fn parse_install_args(args: &[String]) -> Result<InstallOptions, IgnixError>{
    
    let mut force = false;
    let mut allow_virtual = false;
    let mut no_nvram = false;
    let mut removable_device = false;
    let mut install_route = None;
    let mut efi_bin_provided = None;

    for arg in args.iter().skip(2){
        
        match arg.as_str(){
            FORCE_FLAG => force = true,
            ALLOW_VIRTUAL_FLAG => allow_virtual = true,
            NO_NVRAM => no_nvram = true,
            REMOVABLE_FLAG => removable_device = true,
            _ => parse_prefixed_arg(arg, &mut install_route, &mut efi_bin_provided)?
        }
    } 

    let efi_bin = match efi_bin_provided {
        Some(path) => path,
        None => is_valid_efi_bin_path(DEFAULT_EFI_BIN_PATH)?,
    };

    Ok(InstallOptions {
        force,
        allow_virtual,
        no_nvram,
        removable_device,
        efi_bin,
        install_route
    })
}

/// Asigns the values to the RemoveOptions struct
pub fn parse_remove_args(args: &[String]) -> Result<RemoveOptions, IgnixError>{
    Ok(RemoveOptions {
        force: args.iter()
            .skip(2).any(|a| a == FORCE_FLAG)
    })
}

pub fn ask_user_confirmation(context: &str) -> Result<bool, IgnixError>{
    
    println!("Remember to use capital letters as shown:");
    println!("Type 'YES' to {} or 'NO' to cancel.",context);
    
    stdout().flush().ok();

    let mut lector = String::new();
    stdin().read_line(&mut lector).ok();
    
    match lector.trim(){
        "YES" => Ok(true),
        "NO" => Err(cmd::Error::UserAborted)?,
        _ => {
            eprintln!("The program did not understoot the input '{}', assuming 'NO'.",lector);
            Err(cmd::Error::UserAborted)?
        }
    }
}

fn parse_prefixed_arg( arg: &str, route: &mut Option<PathBuf>, efi: &mut Option<PathBuf>) 
    -> Result<(), IgnixError> {
    if let Some(path) = arg.strip_prefix(INSTALL_ROUTE) {
        *route = Some(is_valid_install_path(path)?);
    } else if let Some(path) = arg.strip_prefix(EFI_BIN_PATH) {
        *efi = Some(is_valid_efi_bin_path(path)?);
    } else {
        Err(cmd::Error::InvalidArgument(arg.to_string()))?
    }
    Ok(())
}

fn is_valid_efi_bin_path(route: &str) -> Result<PathBuf, IgnixError>{
    let path = PathBuf::from(route);
    if !path.exists() || path.extension().is_none_or(|ext| ext != "efi") {
        Err(io::Error::NotFound(path.display().to_string()))?;
    }
    Ok(path)
}

fn is_valid_install_path(route: &str) -> Result<PathBuf, IgnixError>{
    let path = PathBuf::from(route);
    if path.exists(){
        return Ok(path);
    }
    Err(io::Error::NotFound(path.display().to_string()))?
}
