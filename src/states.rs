use pinocchio::program_error::ProgramError;

#[repr(C)]
pub struct Counter {
    pub count: u64,
}

impl Counter {
    pub const LEN: usize = size_of::<u64>();

    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if bytes.len() != Counter::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { &mut *core::mem::transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) })
    }
}
