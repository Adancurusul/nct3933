#[cfg(feature = "sync")]
pub mod nct3933_sync;

#[cfg(feature = "async")]
pub mod nct3933_async ; 

pub use crate::errors::NCT3933Error;