use crate::IgnixError;
use crate::cli::ask_user_confirmation;
/// This function should remove the ignix installation in the current ESP partition.
pub fn remove_ignix(options: bool)->Result<(),IgnixError>{
    
    if !options{
        ask_user_confirmation("remove")?;
    }
    todo!("Remove the Ignix installation in the ESP and the NVRAM");
}
