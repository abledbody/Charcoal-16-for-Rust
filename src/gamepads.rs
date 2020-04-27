use asm_19::memory;
use ggez::*;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Gamepads {
	ram: Rc<RefCell<memory::Memory>>,
}

impl Gamepads {
	pub fn new(ram: Rc<RefCell<memory::Memory>>) -> Gamepads {
		Gamepads {
			ram,
		}
	}

	pub fn update(&self, ctx: &mut Context) {
		let a =			keyboard::is_key_pressed(ctx, KeyCode::Z)		as u16;
		let b =			keyboard::is_key_pressed(ctx, KeyCode::X)		as u16;
		let start =		keyboard::is_key_pressed(ctx, KeyCode::V)		as u16;
		let select =	keyboard::is_key_pressed(ctx, KeyCode::C)		as u16;
		let up =		keyboard::is_key_pressed(ctx, KeyCode::Up)		as u16;
		let down =		keyboard::is_key_pressed(ctx, KeyCode::Down)	as u16;
		let left =		keyboard::is_key_pressed(ctx, KeyCode::Left)	as u16;
		let right =		keyboard::is_key_pressed(ctx, KeyCode::Right)	as u16;
		
		let compiled =	a | (b << 1) | (start << 2) | (select << 3) | (up << 4) | (down << 5) | (left << 6) | (right << 7);

		let ram = self.ram.try_borrow_mut().unwrap();
		let write_check = ram.write(crate::GAMEPADS, compiled);
		match write_check {
			Ok(_) => (),
			Err(error) => println!("{}", error.message),
		}
	}
}