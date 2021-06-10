use asm_19::memory::Memory;
use crate::charcoal_mem::CharcoalMem;
use sdl2::keyboard::Keycode;

/// The address for gamepad input
pub const GAMEPADS: u16 = 0xFFFF;

/// When an input is changed, the value at the GAMEPADS address will update to reflect it.
pub fn input_changed(ram: &mut CharcoalMem, keycode: Keycode, down: bool) {
	let bit: u16 = match keycode {
		Keycode::Z =>		0b0000_0001,
		Keycode::X =>		0b0000_0010,
		Keycode::C =>		0b0000_0100,
		Keycode::V =>		0b0000_1000,
		Keycode::Up =>		0b0001_0000,
		Keycode::Down =>	0b0010_0000,
		Keycode::Left =>	0b0100_0000,
		Keycode::Right =>	0b1000_0000,
		_ => 0x00,
	};
	
	// We need to combine this bit with the existing bits, so we first have to read the RAM.
	let mut mem_value: u16 = match ram.read(GAMEPADS) {
		Ok(value) => value,
		Err(_) => 0,
	};
	
	mem_value = if down {
		// Combine GAMEPADS byte with this bit.
		mem_value | bit
	}
	else {
		// Keep everything in GAMEPADS byte but this bit.
		mem_value & !bit
	};
	
	// The RAM doesn't let the CPU write to this address. We have a special function to do it externally.
	let write_result = ram.peripheral_write(GAMEPADS, mem_value);
	match write_result {
		Err(value) => {panic!("{}", value.message);}
		_ => ()
	}
}