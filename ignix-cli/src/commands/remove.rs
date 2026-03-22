use crate::IgnixError;
use crate::cli::{RemoveOptions, ask_user_confirmation};
/// This function should remove the ignix installation in the current ESP partition.
pub fn remove_ignix_installation(options: RemoveOptions) -> Result<(), IgnixError>{
    if !options.force{
        ask_user_confirmation("remove");
    }
    todo!("Remove the Ignix installation in the ESP and the NVRAM");
}
