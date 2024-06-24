// src/lib.rs

//! # Rust driver for 3-Channel Sink/Source Current DAC NCT3933U
//! 
//! > This is a platform agnostic Rust driver for the NCT3933, based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//! ## The Device 
//! The NCT3933U includes three adjustable current DACs that are each capable of sinking 
//! and sourcing current through SMBusTM interface. Each output has 128 sinking and
//! sourcing settings that are programmed by the SMBusTM interface. The output current 
//! also can be programmable for twofold sinking/sourcing increase respectively. The 
//! NCT3933U features step speed controlled function which can easily interfacing with 
//! general DC/DC converter for voltage adjustment. The NCT3933U also provides power 
//! saving function to reduce 60% power consumption when system enters standby mode. 
//! 
//! [NCT3933U](https://!item.szlcsc.com/246282.html)
//! 
//! ## Usage
//! using STM32G031G8Ux
//! 
//! ```rust
//! #![no_std]
//! #![no_main]
//! 
//! #![allow(dead_code)]
//! #![allow(unused_imports)]
//! 
//! use nct3933::NCT3933 ;
//! use nct3933::NCT3933Error;
//! use defmt:: info;
//! use embassy_stm32::i2c::Config;
//! use embassy_time::Timer;
//! use {defmt_rtt as _, panic_probe as _};
//! use embassy_stm32::i2c::{self, I2c};
//! use embassy_stm32::{bind_interrupts, peripherals};
//! use embassy_stm32::time::Hertz;
//! 
//! bind_interrupts!(struct Irqs {
//!     I2C1 => i2c::EventInterruptHandler<peripherals::I2C1>, i2c::ErrorInterruptHandler<peripherals::I2C1>;
//! });
//! 
//! #[embassy_executor::main]
//! async fn main(_spawner: embassy_executor::Spawner) {
//!     let p = embassy_stm32::init(Default::default());
//! 
//!     let i2c = I2c::new(
//!         p.I2C1,
//!         p.PB6,
//!         p.PB7,
//!         Irqs,
//!         p.DMA1_CH1,
//!         p.DMA1_CH2,
//!         Hertz(100_000),
//!         Default::default(),
//!     );
//!     
//!     let mut nct3933 = NCT3933::new(i2c, 0x2A >> 1).unwrap();
//! 
//!    
//!     match nct3933.check_id() {
//!         Ok(()) => { info!("NCT3933 found"); },
//!         Err(e) => { info!("NCT3933 not found :{:?}",e); },
//!     }
//!     match nct3933.set_wdt_state(1,0) {
//!         Ok(()) => { info!("WDT state set");},
//!         Err(e) => { panic!("WDT state not set :{:?}",e);},
//!     }
//! 
//!     match nct3933.set_current(1,10) {
//!         Ok(()) => { info!("Current set");},
//!         Err(e) => { panic!("Current not set :{:?}",e);},
//!     }
//! 
//!     loop { 
//!         Timer::after_millis(2000).await;
//!     }
//! }
//! 
//! ```
//! 

#![no_std]
#![no_main]


//use embassy_stm32::i2c::{self, I2c};
extern crate embedded_hal as hal;
use defmt;

//use hal::i2c::{Read, Write, WriteRead};
use hal::i2c;
// const CHANNEL1_ADDR: u8 = 0x01;
// const CHANNEL2_ADDR: u8 = 0x02;
// const CHANNEL3_ADDR: u8 = 0x03;
const SETTING1_ADDR: u8 = 0x04;
const SETTING2_ADDR: u8 = 0x05;
const ID1_ADDR: u8 = 0x5D;
const ID2_ADDR: u8 = 0x5E;

#[derive(Debug, PartialEq, Eq, Copy, Clone,defmt::Format)]
pub enum NCT3933Error<E> {
    I2C(E),
    InvalidID,
    InvalidChannel,
    InvalidMode,
    InvalidCurrent,
}



//pub struct NCT3933 <const ADDR :u8,T,E>
#[derive(Debug, Default)]
pub struct NCT3933<I2C> {
    /// The concrete I2C device implementation.
    i2c: I2C,
    /// The I2C device address.
    address: u8,

}

impl<I2C,  E> NCT3933<I2C>
where
    I2C: i2c::I2c<Error = E>,
    //D: DelayMs<u8>,
{
    ///Initialize the NCT3933
    pub fn new(i2c: I2C, address: u8) -> Result<Self, E> {
        let nct3933 = NCT3933 {
            i2c,
            address,
        };
        Ok(nct3933)
    }

    ///read register
    pub fn read_register(&mut self, reg_addr: u8) -> Result<u8, NCT3933Error<E>> {
        let mut data = [0];
        self.i2c.write_read(self.address, &[reg_addr], &mut data).map_err(NCT3933Error::I2C)?;
        Ok(data[0])
    }
    ///write register
    pub fn write_register(&mut self, reg: u8, value: u8) -> Result<(), NCT3933Error<E>> {
        self.i2c.write(self.address, &[reg, value]).map_err(NCT3933Error::I2C)?;
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
        if  mode > 1 {
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
        let reg_data: u8 ;
        if current >= -1270 && current <= 1270 {
            self.set_gain(channel, 0)?;
            if current <= 0 {
                reg_data = (-(current / 10) as u8) & (0x7F);
            } else {
                reg_data = ((current / 10) as u8 + 0x80) & (0xFF);
            }
        } else if current >= -2540 && current <= 2540 {
            self.set_gain(channel, 1)?;
            if current<=0 {
                reg_data = (-(current / 20) as u8) & (0x7F);
            }
            else {
                reg_data = ((current / 20) as u8 + 0x80) & (0xFF);
            }
        } else {
            return Err(NCT3933Error::InvalidCurrent);
        }
        self.write_register(channel, reg_data)?;

        Ok(())
    }
}
