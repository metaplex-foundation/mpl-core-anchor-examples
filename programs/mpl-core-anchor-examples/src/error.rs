use anchor_lang::prelude::*;

#[error_code]
pub enum WrapperError {
    #[msg("Invalid plugin type")]
    InvalidPluginType,
}
