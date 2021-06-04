use asm_19::memory::*;
use std::cell::Cell;

const WIDTH: usize = 16;
const ADDRESSES: usize = 1 << WIDTH;

pub struct CharcoalMem {
	data: Vec<Cell<u16>>
}

impl CharcoalMem {
	pub fn new() -> CharcoalMem {
		CharcoalMem {
			data: vec![Cell::new(0); ADDRESSES],
		}
	}
	
	pub fn load(&self, byte_data: Vec<u8>) {
		if byte_data.len() > ADDRESSES {
			panic!("Could not fit data into memory. There are {} bytes of data, but only {} bytes of RAM", byte_data.len(), ADDRESSES * WIDTH / 8)
		}
	
		for address in 0..(byte_data.len() / 2) {
			let index = address * 2;
			let slice = &byte_data[index..(index + 2)];
			if slice.len() != 2 {
				panic!("Short at address {:04X} is {} byte(s) long", address, slice.len());
			}
			
			match self.write(address as u16, merge_16(slice)) {
				Ok(value) => value,
				Err(error) => {println!("{}", error.message);},
			}
		}
	}
}
impl Memory for CharcoalMem {
	fn read(&self, address: u16) -> Result<u16, MemoryReadError> {
		if address as usize >= self.data.len() {
			Err(MemoryReadError {
				message: format!("Attempted to read from out of bounds address {:04X}", address),
				address,
			})
		}
		else {
			Ok(self.data[address as usize].get())
		}
	}
	
	fn write(&self, address: u16, value: u16) -> Result<(), MemoryWriteError> {
		if address as usize >= self.data.len() {
			Err(MemoryWriteError {
				message: format!("Attempted to write {:04X} to out of bounds address {:04X}", value, address),
				address,
				value,
			})
		}
		else {
			self.data[address as usize].set(value);
			Ok(())
		}
	}
}