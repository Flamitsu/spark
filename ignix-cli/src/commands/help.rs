pub fn show_help() {
    const HELP_TEXT: &str = r#"
USAGE:
    ignix [COMMAND] [OPTIONS]

COMMANDS:
    help        Prints this help information

OPTIONS:
    --force                 Skip all interactive confirmation prompts. Only use if you know what you're doing.
    --efi-bin=[PATH]        Manual path to the EFI binary (default: auto-detect)
    --no-nvram              Skips all the logic to write a NVRAM variable.
    --allow-virtual         Allows to install the .efi bin in a virtual device.
    --install-route=[PATH]  Skips the auto-detect of the ESP partition and installs the .efi there
    --removable             Allows to install the .efi bin in a removable device. 
"#;
    println!("{}", HELP_TEXT.trim());
}
