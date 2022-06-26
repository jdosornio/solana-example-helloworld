use solana_program::program_error::ProgramError;

pub enum Instruction {
    Increment,
    Decrement,
    Set(u32),
}

impl Instruction {
    pub fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        let (&op, data) = data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        match op {
            0 => Ok(Instruction::Increment),
            1 => Ok(Instruction::Decrement),
            2 => Ok(Instruction::Set(u32::from_le_bytes(
                data.try_into()
                    .map_err(|_| ProgramError::InvalidInstructionData)?,
            ))),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
