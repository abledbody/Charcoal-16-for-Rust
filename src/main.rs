extern crate sdl2;

use asm_19::{memory::Memory, processor::Processor};
use std::{fs, time::Duration};
use sdl2::{Sdl, pixels, surface::Surface, render::Texture, keyboard::Keycode,
	event::{Event, WindowEvent}};

mod display;
mod palettes;
mod gamepads;
mod charcoal_mem;

/// How many nanoseconds each frame lasts
const TIMESTEP_NANOS: i128 = 1000000000 / 60;
/// CPU clock speed measured in hertz
const CPU_CLOCK_SPEED: f64 = 1000000.0;
/// When lagging, the machine will never extrapolate further than this many seconds
const MAX_TIME_STEP: f64 = 0.5;

/// Contains the current state of the machine. Responsible for peripherals and hardware, as well as keeping track of time.
struct State {
	cycle_error: f64,
	screen: display::Display,
	cpu: Processor,
	ram: charcoal_mem::CharcoalMem,
}

impl State {
	/// This triggers the CPU to run as though ```dt``` has elapsed.
	fn update(&mut self, dt: Duration) {
		let seconds = dt.as_secs_f64();
		
		// This if statement restricts the maximum lag compensation to MAX_TIME_STEP
		let clock_cycles = if seconds <= MAX_TIME_STEP {
				let clock_cycles = seconds * CPU_CLOCK_SPEED - self.cycle_error;
				
				// If we're partially in between clock cycles, we'll record it and compensate for it in the next frame.
				// There's no imperative to make this *super* accurate, but we might as well, just in case.
				// Who knows, maybe it'll be important for speedrunning or something.
				self.cycle_error = clock_cycles % 1.0;
				clock_cycles
			} else {
				self.cycle_error = 0.0;
				MAX_TIME_STEP * CPU_CLOCK_SPEED
			};
		
		// We need an actual discrete number of clock cycles, so we'll convert it to a u64.
		// Any potential error is already handled.
		for _ in 0..clock_cycles as u64 {
			self.cpu.tick(&mut self.ram, false);
		}
	}

	/// Tells the screen to draw based on the current state of RAM, then turns on the V-blank bit in VATTRIBUTES
	fn draw(&mut self, font: &mut Texture) {
		self.screen.render(&self.ram, font);

		self.screen.canvas.present();
		
		let attributes = match self.ram.read(display::VATTRIBUTES) {
			Ok(value) => value,
			Err(_err) => {println!("Could not read VATTRIBUTES, defaulting to 0"); 0},
		};
		
		let vblanked_attributes = attributes | 0b0000000010000000;

		match self.ram.write(display::VATTRIBUTES, vblanked_attributes) {
			Ok(_) => (),
			Err(err) => println!("While setting V-blank bit: {}", err.message),
		};
	}
}

/// Verifies that a ROM exists at the provided path, and tells the CharcoalMem to load everything in it.
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

/// Checks if a binary file has been provided to the program, and loads it, before starting the game loop.
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

/// This function handles keeping the program running, events, and frame timing.
fn game_loop(sdl_ctx: Sdl, mut state: State) {
	let mut event_pump = sdl_ctx.event_pump().unwrap();
	
	let mut last_frame: std::time::Instant = std::time::Instant::now();
	let mut prev_error: i128 = 0;
	let mut font_surface = Surface::load_bmp(display::CHARACTER_PATH).unwrap();
	
	let alpha_white = &[pixels::Color{r:0,g:0,b:0,a:0}, pixels::Color::WHITE];
	font_surface.set_palette(&pixels::Palette::with_colors(alpha_white).unwrap()).unwrap();
	font_surface.set_blend_mode(sdl2::render::BlendMode::Blend).unwrap();
	
	let mut font = font_surface.as_texture(&state.screen.canvas.texture_creator()).unwrap();
	
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