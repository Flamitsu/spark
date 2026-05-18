#[repr(transparent)]
pub struct Status(pub usize);
impl Status {
    pub const SUCCESS: Self = Status(0);
}
