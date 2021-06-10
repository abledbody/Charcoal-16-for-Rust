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
	/// The offset of the previous frame from the desired timestep.
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
		
		// We only want to change one bit in a byte, so we need to know what's actually at the address before we write.
		let attributes = match self.ram.read(display::VATTRIBUTES) {
			Ok(value) => value,
			Err(_err) => {println!("Could not read VATTRIBUTES, defaulting to 0"); 0},
		};
		
		// The 8th bit of VATTRIBUTES is the vblank bit. Whatever state it was before, we're setting it to on right now.
		let vblanked_attributes = attributes | 0b0000_0000_1000_0000;

		match self.ram.write(display::VATTRIBUTES, vblanked_attributes) {
			Ok(_) => (),
			Err(err) => println!("While setting V-blank bit: {}", err.message),
		};
	}
}

/// Verifies that a ROM exists at the provided path, and tells the CharcoalMem to load everything in it.
fn load_rom(path: &String, ram: &mut charcoal_mem::CharcoalMem) -> Result<(), std::io::Error>{
	let rom = match fs::read(path) {
		Ok(data) => data,
		Err(error) => {return Err(error)},
	};

	ram.load(rom);
	Ok(())
}

/// Checks if a binary file has been provided to the program, and loads it, before starting the game loop.
fn main() {
	let args: Vec<String> = std::env::args().collect();

	// For whatever reason the first argument is the original command.
	// If it's less than 2, we must not have a ROM argument.
	if args.len() < 2 {
		println!("Please provide a path to the binary file for Charcoal-16 to execute.");
	}
	else {
		let mut ram = charcoal_mem::CharcoalMem::new();
		
		match load_rom(&args[1], &mut ram) {
			Ok(_) => (println!("Successfully loaded ROM from {}", args[1])),
			Err(error) => {
				panic!("Could not find valid .bin file: {}", error)
			}
		}
		
		let cpu = asm_19::processor::Processor::new();
		
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
	
	// I'm disappointed that we can't use the palette features on Surface in the display draw,
	// because of how expensive (and memory leak-y) converting to textures is.
	// But, we can still use an indexed bitmap by making converting black to transparent, and using
	// set_color_mod to recolor the white parts of the texture before drawing.
	let mut font_surface = Surface::load_bmp(display::FONT_PATH).unwrap();
	
	
	let alpha_white_pal = &pixels::Palette::with_colors(
		&[pixels::Color{r:255,g:255,b:255,a:0}, pixels::Color::WHITE])
		.unwrap();
	font_surface.set_palette(alpha_white_pal).unwrap();
	// Indexed bitmaps have no alpha by defualt, so we need to tell the surface here that it has transparency capabilities.
	font_surface.set_blend_mode(sdl2::render::BlendMode::Blend).unwrap();
	
	let texture_creator = state.screen.canvas.texture_creator();
	// Using textures is a pain in the butt, so instead of trying to coax a reference to the texture into a struct,
	// we're just going to pass it down the chain of functions to display.
	let mut font = font_surface.as_texture(&texture_creator).unwrap();
	
	'running: loop {
		// Calculating delta_time
		let this_frame = std::time::Instant::now();
		let delta_time = this_frame.duration_since(last_frame);
		last_frame = this_frame;
		
		// Handling events
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					break 'running;
				},
				Event::KeyDown {keycode, ..} => {
					match keycode {
						Some(keycode) => {
							match keycode {
								// Special case. If we press escape, close the program. Nevermind sending it to the gamepad.
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
		
		// We need to accumulate error so that we can accurately predict how long we need to sleep the thread for to get back in step after lag.
		let time_error = std::cmp::min(delta_time.as_nanos() as i128 - TIMESTEP_NANOS + prev_error, TIMESTEP_NANOS);
		prev_error = time_error;
		
		// Probably not the best way to handle frame delays.
        std::thread::sleep(std::time::Duration::from_nanos((TIMESTEP_NANOS - time_error) as u64));
    }
}