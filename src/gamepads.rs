use asm_19::memory::Memory;
use crate::charcoal_mem::CharcoalMem;
use sdl2::keyboard::Keycode;

pub fn input_changed(ram: &mut CharcoalMem, keycode: Keycode, down: bool) {
	let bit: u16 = match keycode {
		Keycode::Z =>		0b00000001,
		Keycode::X =>		0b00000010,
		Keycode::C =>		0b00000100,
		Keycode::V =>		0b00001000,
		Keycode::Up =>		0b00010000,
		Keycode::Down =>	0b00100000,
		Keycode::Left =>	0b01000000,
		Keycode::Right =>	0b10000000,
		_ => 0x00,
	};
	
	let mut mem_value: u16 = match ram.read(crate::GAMEPADS) {
		Ok(value) => value,
		Err(_) => 0,
	};
	
	mem_value = if down {
		mem_value | bit
	}
	else {
		mem_value & !bit
	};

	let write_result = ram.peripheral_write(crate::GAMEPADS, mem_value);
	match write_result {
		Err(value) => {panic!("{}", value.message);}
		_ => ()
	}
}