extern crate sdl2;

use asm_19::memory::Memory;
use asm_19::processor::Processor;
use std::time::Duration;
use std::fs;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::{Sdl, pixels, surface::Surface, render::Texture};

mod display;
mod palettes;
mod gamepads;
mod charcoal_mem;

const TIMESTEP_NANOS: i128 = 1000000000 / 60;
const BLACK: pixels::Color = pixels::Color {r: 0, g: 0, b: 0, a: 255};
const VRAM: u16 = 0xFB65;
const VATTRIBUTES: u16 = 0xFFFD;
const GAMEPADS: u16 = 0xFFFF;
const CLOCK_SPEED: f64 = 1000000.0;
const MAX_TIME_STEP: f64 = 0.5;

struct State {
	cycle_error: f64,
	screen: display::Display,
	cpu: Processor,
	ram: charcoal_mem::CharcoalMem,
}

impl State {
	fn update(&mut self, dt: Duration) {
		let seconds = dt.as_secs_f64();
		
		let clock_cycles = if seconds <= MAX_TIME_STEP {
			let clock_cycles = seconds * CLOCK_SPEED - self.cycle_error;
			self.cycle_error = clock_cycles % 1.0;
			clock_cycles
		} else {
			self.cycle_error = 0.0;
			MAX_TIME_STEP * CLOCK_SPEED
		};

		for _ in 0..clock_cycles as u64 {
			self.cpu.tick(&mut self.ram, false);
		}
	}

	fn draw(&mut self, font: &mut Texture) {
		self.screen.canvas.set_draw_color(BLACK);
		self.screen.canvas.clear();
		self.screen.render(&self.ram, font);

		self.screen.canvas.present();
		
		let attributes = match self.ram.read(crate::VATTRIBUTES) {
			Ok(value) => value,
			Err(_err) => {println!("Could not read VATTRIBUTES, defaulting to 0"); 0},
		};
		
		let vblanked_attributes = attributes | 0b0000000010000000;

		match self.ram.write(crate::VATTRIBUTES, vblanked_attributes) {
			Ok(_) => (),
			Err(err) => println!("While setting V-blank bit: {}", err.message),
		};
	}
}

fn load_rom(path: &String, ram: &mut charcoal_mem::CharcoalMem) {
	let result = fs::read(path);

	let rom = match result {
		Ok(data) => data,
		Err(error) => {
			panic!("Invalid path to .bin file. \n{:?}", error);
		},
	};

	ram.load(rom)
}

fn main() {
	let args: Vec<String> = std::env::args().collect();

	if args.len() < 2 {
		println!("Please provide a path to the binary file for Charcoal-16 to execute.");
	}
	else {
		let mut ram = charcoal_mem::CharcoalMem::new();
		load_rom(&args[1], &mut ram);
		let cpu = asm_19::processor::Processor::new();

		println!("Successfully loaded ROM from {}", args[1]);
		
		let sdl_ctx = sdl2::init().unwrap();
		let new_screen = display::Display::new(&sdl_ctx);
		
		let state = State {
			cycle_error: 0.0,
			screen: new_screen,
			cpu,
			ram,
		};

		game_loop(sdl_ctx, state)
	}
}

fn game_loop(sdl_ctx: Sdl, mut state: State) {
	let mut event_pump = sdl_ctx.event_pump().unwrap();
	
	let mut last_frame: std::time::Instant = std::time::Instant::now();
	let mut prev_error: i128 = 0;
	let font_surface = Surface::load_bmp(std::path::Path::new(display::CHARACTER_PATH));
	
	let mut font = state.screen.canvas.create_texture_from_surface(font_surface.as_ref().unwrap()).unwrap();
	
	'running: loop {
		let this_frame = std::time::Instant::now();
		let delta_time = this_frame.duration_since(last_frame);
		last_frame = this_frame;
		
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					break 'running;
				},
				Event::KeyDown {keycode, ..} => {
					match keycode {
						Some(keycode) => {
							match keycode {
								Keycode::Escape => {
									break 'running;
								}
								_ => {
									gamepads::input_changed(&mut state.ram, keycode, true);
								}
							}
						},
						_ => ()
					}
				},
				Event::KeyUp {keycode, ..} => {
					match keycode {
						Some(keycode) => {
							gamepads::input_changed(&mut state.ram, keycode, false);
						}
						_ => ()
					}
				}
				Event::Window {win_event, ..} => {
					match win_event {
						WindowEvent::Resized(w, h) => {
							state.screen.window_resize(w as u32, h as u32);
						}
						_ => ()
					}
				}
				_ => ()
			}
		}
		
		state.update(delta_time);
		state.draw(&mut font);
		
		let time_error = std::cmp::min(delta_time.as_nanos() as i128 - TIMESTEP_NANOS + prev_error, TIMESTEP_NANOS);
		prev_error = time_error;
		
        std::thread::sleep(std::time::Duration::from_nanos((TIMESTEP_NANOS - time_error) as u64));
    }
}