#[derive(PartialEq, Eq)]
// This parameter makes the code 'secure' but in the end it will treat it as plain usize. Just because if not it can miss interpret what the UEFI protocol or status is saying.
#[repr(transparent)]
pub struct Status(pub usize);
impl Status {
    pub const SUCCESS: Self = Status(0);
}
