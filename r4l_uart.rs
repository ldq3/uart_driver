// SPDX-License-Identifier: GPL-2.0

//! pl011 driver based on Rust for linux

use kernel::{
    prelude::*,
    amba,
    module_amba_driver,
    device,
    io_mem,
    serial::uart_port,
    error::code::*, // dev_info! 和 dbg!
};

const UART_SIZE: usize = 0x200;
const UPIO_MEM: u32 = 2;
pub(crate) const UART_NR: usize = 14;

pub const UPF_BOOT_AUTOCONF: u64 = 1_u64 << 28;

pub(crate) static mut PORTS: [Option<uart_port::UartPort>; UART_NR] = [None; UART_NR];

module_amba_driver! {
    type: Pl011Driver,
    name: "pl011_driver",
    author: "ldq3",
    description: "pl011 driver based on Rust for Linux",
    license: "GPL",
    initcall: "arch",
}

struct Pl011Driver {
    numbers: Vec<i32>,
}

// Linux Raw id table
kernel::define_amba_id_table! {MY_AMDA_ID_TABLE, (), [
    ({id: 0x00041011, mask: 0x000fffff}, None),
]}

kernel::module_amba_id_table!(UART_MOD_TABLE, MY_AMDA_ID_TABLE);

/// Find available driver ports sequentially.
fn pl011_find_free_port() -> Result<usize>{
    for (index, port) in unsafe { PORTS.iter().enumerate() } {
        if let None = port {
            return Ok(index)
        }
    }
    return Err(EBUSY);
}

impl amba::Driver for Pl011Driver {
    // type Data = Arc<DeviceData>;

    kernel::driver_amba_id_table!(MY_AMDA_ID_TABLE);
    fn probe(adev: &mut amba::Device, _id_info: Option<&Self::IdInfo>) -> Result {
        dev_info!(adev, "{} PL011 GPIO chip (probe)\n", adev.name());

        let dev = device::Device::from_dev(adev);
        // TODO: vendor, id_info

        let portnr = pl011_find_free_port()?;
        dev_info!(adev,"portnr is {}\n",portnr);

        // uap
        let clk = dev.clk_get().unwrap();
        let reg_base   = adev.take_resource().ok_or(ENXIO)?;
        let reg_mem : io_mem::IoMem<UART_SIZE>= unsafe { io_mem::IoMem::try_new(&reg_base)?};
        let mapbase = reg_base.get_offset();
        let membase = reg_mem.get();
        let fifosize = if adev.revision_get().unwrap() < 3 {16} else {32};
        let iotype = UPIO_MEM as u8;
        let irq = adev.irq(0).ok_or(ENXIO)?;
        // TODO: ops

        let has_sysrq = 1;
        let flags = UPF_BOOT_AUTOCONF;
        let port =  uart_port::UartPort::new().setup(
                membase, 
                mapbase, 
                irq,
                iotype,
                flags, 
                has_sysrq,
                fifosize,
             portnr as _,
            );

        dbg!("********* PL011 GPIO chip registered *********\n");
        Ok(())
    }
}

impl Drop for Pl011Driver {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("pl011 driver module (exit)\n");
    }
}
