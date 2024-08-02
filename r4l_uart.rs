// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.

use kernel::prelude::*;

module! {
    type: RustMinimal,
    name: "r4l_uart",
    author: "ldq3",
    description: "UART driver based on Rust for Linux.",
    license: "GPL",
}

struct UartDriverMod {
    numbers: Vec<i32>,
}

impl kernel::Module for UartDriverMod {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("UART driver module (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));

        let mut numbers = Vec::new();
        numbers.try_push(72)?;
        numbers.try_push(108)?;
        numbers.try_push(200)?;

        Ok(RustMinimal { numbers })
    }
}

impl Drop for UartDriverMod {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("UART driver module (exit)\n");
    }
}
