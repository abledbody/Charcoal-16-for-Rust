use asm_19::memory::*;

/// The width of a memory address. Technically this is still called a "Byte" even though it's 16 bits wide.
const WIDTH: usize = 16;
/// How many addresses are in the RAM.
const ADDRESSES: usize = 1 << WIDTH;

/// The RAM for Charocal-16. Has no special features. It just holds 2^16 addresses for use by the CPU and peripherals.
pub struct CharcoalMem {
	data: [u16; ADDRESSES]
}

impl CharcoalMem {
	pub fn new() -> CharcoalMem {
		CharcoalMem {
			data: vec![0; ADDRESSES],
		}
	}
	
	/// Replaces the first bytes in memory with the provided ```byte_data```
	pub fn load(&mut self, byte_data: Vec<u8>) {
		if byte_data.len() > ADDRESSES {
			panic!("Could not fit data into memory. There are {} bytes of data, but only {} bytes of RAM", byte_data.len(), ADDRESSES * WIDTH / 8)
		}
		
		// Will not write to addresses higher than the length of byte_data
		for address in 0..(byte_data.len() / 2) {
			let index = address * 2;
			let slice = &byte_data[index..(index + 2)];
			// if we only have one octet, we don't have enough data to convert that into a hexadecet.
			if slice.len() != 2 {
				panic!("Short at address {:04X} is {} byte(s) long", address, slice.len());
			}
			
			// We want to write to the RAM verbatim, so we use periperal_write to get around CharcoalMem's restrictions.
			match self.peripheral_write(address as u16, merge_16(slice)) {
				Ok(value) => value,
				Err(error) => {println!("{}", error.message);},
			}
		}
	}
}

impl CharcoalMem {
	/// Allows peripherals to write to restricted addresses.
	pub fn peripheral_write(&mut self, address: u16, value: u16) -> Result<(), MemoryWriteError> {
		if address as usize >= self.data.len() {
			Err(MemoryWriteError {
				message: format!("Attempted to write {:04X} to out of bounds address {:04X}", value, address),
				address,
				value,
			})
		}
		else {
			self.data[address as usize] =value;
			Ok(())
		}
	}
}

impl Memory for CharcoalMem {
	/// Reads from any address in memory.
	fn read(&self, address: u16) -> Result<u16, MemoryReadError> {
		Ok(self.data[address as usize])
	}
	
	/// Allows the CPU to write to any address except GAMEPADS.
	fn write(&mut self, address: u16, value: u16) -> Result<(), MemoryWriteError> {
		match address {
			crate::gamepads::GAMEPADS => {
				Err(MemoryWriteError {
					message: format!("Attempted to write {:04X} to GAMEPADS at {:04X}", value, address),
					address,
					value,
				})
			},
			_ => {
				self.data[address as usize] = value;
				Ok(())
			},
		}
	}
}