use super::board_error::BoardError

pub struct Led {
    port: OutputPort
}

impl Led {
    pub fn on(&self) -> Result<(), BoardError> {
        port.
        Ok(())
    }

    pub fn off(&self) -> Result<(), BoardError> {
        todo!();

        Ok(())
    }

    pub fn toggle(&self) -> Result<(), BoardError> {
        todo!();

        Ok(())
    }
}
