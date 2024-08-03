// SPDX-License-Identifier: GPL-2.0

//! pl011 driver based on Rust for linux

use kernel::{
    prelude::*,
    amba,
    module_amba_driver,
    device,
    io_mem,
    serial::uart_port,
};

const UART_SIZE: usize = 0x200;
const UPIO_MEM: u32 = 2;
pub(crate) const UART_NR: usize = 14;

pub const UPF_BOOT_AUTOCONF: u64 = 1_u64 << 28;

pub(crate) static mut PORTS: [Option<uart_port::UartPort>; UART_NR] = [None; UART_NR];

module_amba_driver! {
    type: Pl011Driver,
    name: "r4l_uart",
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
    fn probe(adev: &mut amba::Device, _data: Option<&Self::IdInfo>) -> Result {
        // dev_info!(adev,"{} PL061 GPIO chip (probe)\n",adev.name());
        // dbg!("********** PL061 GPIO chip (probe) *********\n");

        let dev = device::Device::from_dev(adev);

        let portnr = pl011_find_free_port()?;
        // dev_info!(adev,"portnr is {}\n",portnr);
        let clk = dev.clk_get().unwrap();  // 获得clk
        let fifosize = if adev.revision_get().unwrap() < 3 {16} else {32};
        let iotype = UPIO_MEM as u8;
        let reg_base = adev.take_resource().ok_or(ENXIO)?;
        let reg_mem : io_mem::IoMem<UART_SIZE>= unsafe { io_mem::IoMem::try_new(&reg_base)?};
        let mapbase = reg_base.get_offset();
        let membase = reg_mem.get();
        let irq = adev.irq(0).ok_or(ENXIO)?;

        // dev_info!(adev,"fifosize is {}\n",fifosize);
        // dev_info!(adev,"mapbase is 0x{:x}\n", mapbase);
        // dev_info!(adev,"membase is 0x{:p}\n",membase);
        // dev_info!(adev,"irq is {}\n",irq);
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

        // dbg!("********* PL061 GPIO chip registered *********\n");
        Ok(())
    }
}

impl Drop for Pl011Driver {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("UART driver module (exit)\n");
    }
}
