extern crate embedded_hal as hal;

use crate::errors::NCT3933Error;

//use hal::i2c::{Read, Write, WriteRead};
use hal::i2c;
// const CHANNEL1_ADDR: u8 = 0x01;
// const CHANNEL2_ADDR: u8 = 0x02;
// const CHANNEL3_ADDR: u8 = 0x03;
const SETTING1_ADDR: u8 = 0x04;
const SETTING2_ADDR: u8 = 0x05;
const ID1_ADDR: u8 = 0x5D;
const ID2_ADDR: u8 = 0x5E;

//pub struct NCT3933 <const ADDR :u8,T,E>
#[derive(Debug, Default)]
pub struct NCT3933<I2C> {
    /// The concrete I2C device implementation.
    i2c: I2C,
    /// The I2C device address.
    address: u8,
}

impl<I2C, E> NCT3933<I2C>
where
    I2C: i2c::I2c<Error = E>,
    //D: DelayMs<u8>,
{
    pub fn new(i2c: I2C, address: u8) -> Result<Self, E> {
        let nct3933 = NCT3933 {
            i2c,
            address: address >> 1, //feature of nct2933 i2c address  0x2A >> 1 = 0x15
        };
        Ok(nct3933)
    }

    ///read register
    pub fn read_register(&mut self, reg_addr: u8) -> Result<u8, NCT3933Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[reg_addr], &mut data)
            .map_err(NCT3933Error::I2C)?;
        Ok(data[0])
    }
    ///write register
    pub fn write_register(&mut self, reg: u8, value: u8) -> Result<(), NCT3933Error<E>> {
        self.i2c
            .write(self.address, &[reg, value])
            .map_err(NCT3933Error::I2C)?;
        Ok(())
    }
    ///check the ID of the NCT3933
    pub fn check_id(&mut self) -> Result<(), NCT3933Error<E>> {
        let id1 = self.read_register(ID1_ADDR)?;
        let id2 = self.read_register(ID2_ADDR)?;
        if id1 == 0x39 && id2 == 0x33 {
            Ok(())
        } else {
            Err(NCT3933Error::InvalidID)
        }
    }
    ///read the ID data
    pub fn read_id_data(&mut self) -> Result<u16, NCT3933Error<E>> {
        let id1 = self.read_register(ID1_ADDR)?;
        let id2 = self.read_register(ID2_ADDR)?;
        Ok((id1 as u16) << 8 | id2 as u16)
    }
    ///read the WDT state
    pub fn read_wdt_state(&mut self) -> Result<u8, NCT3933Error<E>> {
        let setting1 = self.read_register(SETTING1_ADDR)?;
        let state = (setting1 & 0x40) >> 6;
        Ok(state)
    }
    ///set the WDT state
    ///enable_state: 0:disable, 1:enable
    ///delay: 0:1400ms, 1:2800ms, 2:5500ms, 3: 11000ms
    pub fn set_wdt_state(&mut self, enable_state: u8, delay: u8) -> Result<(), NCT3933Error<E>> {
        if delay > 3 || enable_state > 1 {
            return Err(NCT3933Error::InvalidChannel);
        }
        let setting1 = self.read_register(SETTING1_ADDR)?;
        let new_setting1: u8 = (setting1 & 0x0F) | ((enable_state << 7) + (delay << 4));
        self.write_register(SETTING1_ADDR, new_setting1)?;
        Ok(())
    }

    ///set the PS mode
    pub fn set_ps_mode(&mut self, enable_state: u8) -> Result<(), NCT3933Error<E>> {
        if enable_state > 1 {
            return Err(NCT3933Error::InvalidMode);
        }

        let setting1 = self.read_register(SETTING2_ADDR)?;

        let new_setting1: u8 = (setting1 & 0xBF) | (enable_state << 6); //bit 6
        self.write_register(SETTING2_ADDR, new_setting1)?;
        Ok(())
    }
    ///set the gain
    ///channel : 1-3, mode 0/1
    pub fn set_gain(&mut self, channel: u8, mode: u8) -> Result<(), NCT3933Error<E>> {
        if mode > 1 {
            return Err(NCT3933Error::InvalidMode);
        }
        let setting1 = self.read_register(SETTING2_ADDR)?;
        let new_setting1: u8 = match channel {
            1 => (setting1 & 0xFE) + mode,
            2 => (setting1 & 0xFB) + (mode << 2),
            3 => (setting1 & 0xEF) + (mode << 4),
            _ => return Err(NCT3933Error::InvalidChannel), // invalid channel
        };
        //let mut new_setting1:u8 = (setting1 & 0xFC) | gain; //bit 0,1
        self.write_register(SETTING2_ADDR, new_setting1)?;
        Ok(())
    }
    ///set the current
    /// channel: 1~3
    /// current: -2540(uA)~2540(uA)
    pub fn set_current(&mut self, channel: u8, current: i16) -> Result<(), NCT3933Error<E>> {
        if channel > 3 || channel < 1 {
            return Err(NCT3933Error::InvalidChannel);
        }
        let reg_data: u8;
        if current >= -1270 && current <= 1270 {
            self.set_gain(channel, 0)?;
            if current <= 0 {
                reg_data = (-(current / 10) as u8) & (0x7F);
            } else {
                reg_data = ((current / 10) as u8 + 0x80) & (0xFF);
            }
        } else if current >= -2540 && current <= 2540 {
            self.set_gain(channel, 1)?;
            if current <= 0 {
                reg_data = (-(current / 20) as u8) & (0x7F);
            } else {
                reg_data = ((current / 20) as u8 + 0x80) & (0xFF);
            }
        } else {
            return Err(NCT3933Error::InvalidCurrent);
        }
        self.write_register(channel, reg_data)?;

        Ok(())
    }
}
