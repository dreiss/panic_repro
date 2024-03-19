pub struct Uart {
    base_address: usize,
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, out: &str) -> core::fmt::Result {
        for c in out.bytes() {
            self.put_blocking(c);
        }
        Ok(())
    }
}

impl Uart {
    pub unsafe fn new(base_address: usize) -> Self {
        let uart = Uart { base_address };

        // Set word length to 8 bits.
        uart.reg(3).write_volatile(0b11);
        // Enable FIFO.
        uart.reg(2).write_volatile(0b1);
        // Enable receiver interrupt.
        uart.reg(1).write_volatile(0b1);

        uart
    }

    unsafe fn reg(&self, offset: usize) -> *mut u8 {
        (self.base_address as *mut u8).add(offset)
    }

    pub fn put_blocking(&mut self, c: u8) {
        unsafe {
            // Wait for holding register to be empty.
            while self.reg(5).read_volatile() & (1 << 5) == 0 {}
            // Write value.
            self.reg(0).write_volatile(c);
        }
    }

    pub fn get(&mut self) -> Option<u8> {
        unsafe {
            if self.reg(5).read_volatile() & (1 << 0) == 0 {
                // No data available.
                None
            } else {
                // Get data.
                Some(self.reg(0).read_volatile())
            }
        }
    }
}
