[![crates.io](https://img.shields.io/crates/v/nct3933.svg)](https://crates.io/crates/nct3933)
# Rust driver for 3-Channel Sink/Source Current DAC NCT3933U

> This is a platform agnostic Rust driver for the NCT3933, based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.

## The Device 
The NCT3933U includes three adjustable current DACs that are each capable of sinking 
and sourcing current through SMBusTM interface. Each output has 128 sinking and
sourcing settings that are programmed by the SMBusTM interface. The output current 
also can be programmable for twofold sinking/sourcing increase respectively. The 
NCT3933U features step speed controlled function which can easily interfacing with 
general DC/DC converter for voltage adjustment. The NCT3933U also provides power 
saving function to reduce 60% power consumption when system enters standby mode. 

[NCT3933U](https://item.szlcsc.com/246282.html)

## Usage
using STM32G031G8Ux

```rust
#![no_std]
#![no_main]

#![allow(dead_code)]
#![allow(unused_imports)]

use nct3933::NCT3933 ;
use nct3933::NCT3933Error;
use defmt:: info;
use embassy_stm32::i2c::Config;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};
use embassy_stm32::i2c::{self, I2c};
use embassy_stm32::{bind_interrupts, peripherals};
use embassy_stm32::time::Hertz;

bind_interrupts!(struct Irqs {
    I2C1 => i2c::EventInterruptHandler<peripherals::I2C1>, i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_stm32::init(Default::default());

    let i2c = I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        Irqs,
        p.DMA1_CH1,
        p.DMA1_CH2,
        Hertz(100_000),
        Default::default(),
    );

    let mut nct3933 = NCT3933::new(i2c, 0x2A >> 1).unwrap();// important to shift the address by 1 !! nct3933 feature

    
    match nct3933.check_id() {
        Ok(()) => { info!("NCT3933 found"); },
        Err(e) => { info!("NCT3933 not found :{:?}",e); },
    }
    match nct3933.set_wdt_state(1,0) {
        Ok(()) => { info!("WDT state set");},
        Err(e) => { panic!("WDT state not set :{:?}",e);},
    }

    match nct3933.set_current(1,10) {
        Ok(()) => { info!("Current set");},
        Err(e) => { panic!("Current not set :{:?}",e);},
    }

    loop { 
        Timer::after_millis(2000).await;
    }
}
 
```


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.