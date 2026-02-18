# rust_ADC_asyc
# STM32 Blue Pill ADC + UART using Rust Embassy

This project demonstrates asynchronous analog-to-digital conversion (ADC) on the **STM32F103C8T6 (Blue Pill)** using the Rust embedded ecosystem and the Embassy async framework.

The firmware periodically samples an analog signal from **PA0 (ADC Channel 0)** and sends the measured value over **USART1 (PA9)** while running a heartbeat LED on the onboard **PC13** pin. The design focuses on reliability, zero-allocation execution, and non-blocking embedded programming.

## Features

* Async embedded firmware powered by Embassy executor
* ADC sampling with timeout protection using async `select`
* UART serial output at 115200 baud
* Active-low onboard LED heartbeat indicator
* `no_std` and `no_main` bare-metal Rust application
* Heap-free numeric formatting (no dynamic allocation)
* Cooperative multitasking with timers

## Hardware Requirements

* STM32F103C8T6 “Blue Pill”
* ST-Link or compatible debugger
* USB-to-TTL serial adapter
* Potentiometer or analog sensor (optional)

### Pin Configuration

| Function  | Pin  |
| --------- | ---- |
| ADC Input | PA0  |
| UART TX   | PA9  |
| LED       | PC13 |

## Example Serial Output

```
UART OK
A0: 1832
A0: 1840
A0: 1827
```

## Build & Flash

Install cargo-embed:

```
cargo install cargo-embed
```

Build and flash firmware:

```
cargo embed
```

## Project Structure

```
src/
 ├── main.rs      # Main firmware logic
 └── fmt.rs       # Lightweight logging utilities
Embed.toml        # Probe configuration
Cargo.toml        # Dependencies
```

## Goal

This repository provides a minimal, practical example of async embedded Rust on STM32, suitable for learning Embassy, sensor interfacing, and reliable UART debugging.

## License

MIT License
