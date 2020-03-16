#![no_std]
#![no_main]

extern crate panic_semihosting;

use rtfm::app;
use ssd1306::{prelude::*, Builder};
use stm32l0xx_hal as hal;

use hal::{
    delay::Delay,
    exti::TriggerEdge,
    gpio::*,
    pac,
    prelude::*,
    rcc::Config,
    spi::{self, Mode, Phase, Polarity, NoMiso},
    syscfg,
    timer::{Timer},
};

use embedded_graphics::prelude::*;
use embedded_graphics::Drawing;
use embedded_graphics::{text_6x8, egcircle, icoord};

#[app(device = stm32l0xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
    }

    #[init]
    fn init(cx: init::Context) {
        // Configure the clock.
        let mut rcc = cx.device.RCC.freeze(Config::hsi16());
        let mut syscfg = syscfg::SYSCFG::new(cx.device.SYSCFG, &mut rcc);

        // Acquire the GPIOB peripheral. This also enables the clock for GPIOB in
        // the RCC register.
        let gpioa = cx.device.GPIOA.split(&mut rcc);
        let gpiob = cx.device.GPIOB.split(&mut rcc);
        let gpioc = cx.device.GPIOC.split(&mut rcc);

        let mut cs = gpiob.pb12.into_push_pull_output();
        cs.set_low(); // not sure if needed, did not try without it

        let sck = gpiob.pb13;
        let mosi = gpiob.pb15;

        // Initialise the SPI peripheral.
        let mut spi = cx.device
           .SPI2
           .spi(
                (sck, NoMiso, mosi), 
                spi::MODE_0, 
                1_000_000.hz(), 
                &mut rcc
            );

        let dc = gpiob.pb8.into_push_pull_output();
        let mut res = gpiob.pb9.into_push_pull_output();

        let mut delay = Delay::new(cx.core.SYST, rcc.clocks);

        let mut disp: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();

        disp.reset(&mut res, &mut delay).unwrap();
        disp.init().unwrap();
    
        // Top side
        disp.set_pixel(0, 0, 1);
        disp.set_pixel(1, 0, 1);
        disp.set_pixel(2, 0, 1);
        disp.set_pixel(3, 0, 1);
    
        // Right side
        disp.set_pixel(3, 0, 1);
        disp.set_pixel(3, 1, 1);
        disp.set_pixel(3, 2, 1);
        disp.set_pixel(3, 3, 1);
    
        // Bottom side
        disp.set_pixel(0, 3, 1);
        disp.set_pixel(1, 3, 1);
        disp.set_pixel(2, 3, 1);
        disp.set_pixel(3, 3, 1);
    
        // Left side
        disp.set_pixel(0, 0, 1);
        disp.set_pixel(0, 1, 1);
        disp.set_pixel(0, 2, 1);
        disp.set_pixel(0, 3, 1);



        let c = egcircle!((20, 20), 8, fill = Some(1u8));
        let t = text_6x8!("Hello Rust!", fill = Some(20u8)).translate(icoord!(20, 16));

        disp.draw(&mut c);
        disp.draw(&mut t);
    
        disp.flush().unwrap();

        // Return the initialised resources.
    }

    extern "C" {
        fn USART4_USART5();    
    }
};
