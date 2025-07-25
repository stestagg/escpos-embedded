# escpos-embedded

A `no_std` ESC/POS printer driver for embedded systems, built in Rust.

This library provides a high-level interface to communicate with ESC/POS-compatible thermal printers over any transport implementing `Read` and `Write`, without requiring a standard library.

## Features

- Compatible with `#![no_std]`
- High-level API for text, formatting, images, barcodes, and queries
- Works over any `Read + Write` transport (e.g., serial, USB, etc.)
- Lightweight, zero-alloc core for constrained devices
- Optional `image` feature for printing bitmaps

## Example

```rust
use escpos_embedded::Printer;
use some_hal::{Serial}; // replace with your HAL

let serial = Serial::new(...);
let mut printer = Printer::new(serial);

printer.set_bold(true)?;
printer.write_line("Hello, world!")?;
printer.feed(2)?;
```

### Using with `embedded-io`

Enable the `embedded_io` feature and wrap transports that implement
`embedded_io::Read`/`Write` with `FromEmbeddedIo`:

```rust
use escpos_embedded::{Printer, FromEmbeddedIo};
use some_hal::Uart; // your HAL UART implementing embedded-io

let uart = Uart::new(...);
let mut printer = Printer::new(FromEmbeddedIo(uart));
```

### Printing Images

Enable the `image` feature and call `print_image`:

```rust
use escpos_embedded::{Printer, Image};

let img = Image {
    width: 8,
    height: 1,
    data: &[0xFF],
};
printer.print_image(&img)?;
```
 
## Design Overview

Generic Transport: The driver is generic over a transport implementing core::io::{Read, Write} (or embedded-compatible traits via feature flags). This allows use on UART, USB, or any custom protocol.

Command Abstraction: ESC/POS commands are abstracted into a safe API. Users don’t build byte sequences manually.

Minimal Dependencies: The core crate avoids allocation and uses only core traits. Optional features may enable heap or image processing functionality.
Error Handling: Uses custom error type to wrap I/O and protocol-level issues.

Based on embassy.dev, initial version is Sync only (async impl welcome)
