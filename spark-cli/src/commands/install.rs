use std::path::Path;
use crate::cli;
use crate::errors::cmd;
use crate::SparkError;
/// This function should install the sparkx64.efi binary in the current ESP partition.
pub fn install_spark(args: &[String], force_install: bool) -> Result<(), SparkError>{
    let _efi_bin_path: &Path = cli::get_efi_bin_path(args)?;
    
    // If the user did not used the flags '-y' or '--yes' in the execution, ask confirmation prompt
    let continue_program = if force_install{
        true
    } else{
        cli::ask_user_confirmation("install")
    };

    // Aborts the program immediately 
    if !continue_program{
        return Err(cmd::Error::UserAborted)?;
    }

    Ok(())
}
