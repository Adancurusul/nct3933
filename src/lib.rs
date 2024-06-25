//! # Rust driver for 3-Channel Sink/Source Current DAC NCT3933U
//!
//! > This is a platform-independent Rust driver for NCT3933, provided in both synchronous and asynchronous versions, based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) and [`embedded-hal-async`](https://github.com/rust-embedded/embedded-hal/tree/master/embedded-hal-async) features, respectively.
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
//! The following are ways to use the library
//! ### Synchronous version
//! using STM32G031G8Ux

//! ```rust
//! #![no_std]
//! #![no_main]
//!
//! #![allow(dead_code)]
//! #![allow(unused_imports)]
//!
//! use nct3933::NCT3933Sync as NCT3933;
//! use nct3933::errors::NCT3933Error;
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
//!     let mut nct3933 = NCT3933::new(i2c, 0x2A).unwrap();
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
//! ### Asynchronous version
//! using STM32G031G8Ux
//!
//! ```rust
//! #![no_std]
//! #![no_main]
//!
//! #![allow(dead_code)]
//! #![allow(unused_imports)]
//!
//! use nct3933::NCT3933Async as NCT3933;
//! use nct3933::errors::NCT3933Error;
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
//!     let mut nct3933 = NCT3933::new(i2c, 0x2A).unwrap();
//!
//!    
//!     match nct3933.check_id().await {
//!         Ok(()) => { info!("NCT3933 found"); },
//!         Err(e) => { info!("NCT3933 not found :{:?}",e); },
//!     }
//!     match nct3933.set_wdt_state(1,0).await {
//!         Ok(()) => { info!("WDT state set");},
//!         Err(e) => { panic!("WDT state not set :{:?}",e);},
//!     }
//!
//!     match nct3933.set_current(1,10).await {
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

pub mod errors;
pub mod nct3933;

#[cfg(feature = "sync")]
pub use nct3933::nct3933_sync::NCT3933 as NCT3933Sync;

#[cfg(feature = "async")]
pub use nct3933::nct3933_async::NCT3933 as NCT3933Async;
