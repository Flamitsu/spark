use crate::utils::confirmation;
use crate::auto_detect::detect_kernels;
use crate::auto_detect::detect_new_kernel;
pub fn install(){
    let confirm = confirmation("install");
    if confirm == true{
        println!("hell yeah");
        detect_new_kernel();
        detect_kernels();
    }
    else{
        println!("The installation process has been aborted.")
    }
}
