// This should have the code to read the LBA1 of the disks and gave the little endian format for
// comparation with the GPT GUID EFI SYSTEM specification.
pub fn _get_uefi_partition(){
    struct GptGUID{
        dataset1: u32,
        dataset2: u16,
        dataset3: u16,
        dataset4: [u8; 8],
    }
}
