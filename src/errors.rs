use defmt;


#[derive(Debug, PartialEq, Eq, Copy, Clone,defmt::Format)]
pub enum NCT3933Error<E> {
    I2C(E),
    InvalidID,
    InvalidChannel,
    InvalidMode,
    InvalidCurrent,
}
