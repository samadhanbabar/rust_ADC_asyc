#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use embassy_stm32::adc::{Adc, SampleTime};
use embassy_stm32::{adc, bind_interrupts, peripherals};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::usart::{Config as UartConfig, UartTx};
use embassy_time::{Duration, Timer};
use fmt::info;

bind_interrupts!(struct Irqs {
    ADC1_2 => adc::InterruptHandler<peripherals::ADC1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    // Blue Pill onboard LED is on PC13 and is active-low.
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);
    let mut adc = Adc::new(p.ADC1);
    let mut a0 = p.PA0;

    let mut uart_cfg = UartConfig::default();
    uart_cfg.baudrate = 115_200;
    let mut tx = UartTx::new_blocking(p.USART1, p.PA9, uart_cfg).unwrap();

    for _ in 0..3 {
        led.set_low();
        Timer::after(Duration::from_millis(150)).await;
        led.set_high();
        Timer::after(Duration::from_millis(150)).await;
    }

    let _ = tx.blocking_write(b"UART OK\r\n");
    let mut heartbeat_ticks: u8 = 0;

    loop {
        match select(
            adc.read(&mut a0, SampleTime::CYCLES239_5),
            Timer::after(Duration::from_millis(50)),
        )
        .await
        {
            Either::First(adc_value) => {
                info!("ADC A0 = {}", adc_value);

                let mut line = [0u8; 16];
                let len = format_adc_line(adc_value, &mut line);
                let _ = tx.blocking_write(&line[..len]);
            }
            Either::Second(_) => {
                let _ = tx.blocking_write(b"ADC TIMEOUT\r\n");
            }
        }

        heartbeat_ticks = heartbeat_ticks.wrapping_add(1);
        if heartbeat_ticks >= 5 {
            heartbeat_ticks = 0;
            let _ = tx.blocking_write(b"UART OK\r\n");
        }

        led.toggle();
        Timer::after(Duration::from_millis(200)).await;
    }
}

fn format_adc_line(value: u16, out: &mut [u8; 16]) -> usize {
    let mut digits = [0u8; 5];
    let mut n = value;
    let mut i = 0;

    if n == 0 {
        digits[0] = b'0';
        i = 1;
    } else {
        while n > 0 {
            digits[i] = b'0' + (n % 10) as u8;
            n /= 10;
            i += 1;
        }
    }

    let mut idx = 0;
    out[idx] = b'A';
    idx += 1;
    out[idx] = b'0';
    idx += 1;
    out[idx] = b':';
    idx += 1;
    out[idx] = b' ';
    idx += 1;

    while i > 0 {
        i -= 1;
        out[idx] = digits[i];
        idx += 1;
    }

    out[idx] = b'\r';
    idx += 1;
    out[idx] = b'\n';
    idx += 1;
    idx
}
