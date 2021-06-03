use ggez::*;
use asm_19;
use std::time::Duration;
use std::fs;

mod display;
mod palettes;
mod gamepads;
mod charcoal_mem;

const BLACK: graphics::Color = graphics::Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0};
const VRAM: u16 = 0xFB65;
const VATTRIBUTES: u16 = 0xFFFD;
const GAMEPADS: u16 = 0xFFFF;
const CLOCK_SPEED: f64 = 1000000.0;
const MAX_TIME_STEP: f64 = 0.5;

struct State {
	dt: Duration,
	cycle_error: f64,
	screen: display::Display,
	computer: asm_19::Computer,
}

impl event::EventHandler for State {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.dt = timer::delta(ctx);
		let seconds = self.dt.as_secs_f64();
		
		let clock_cycles = if seconds <= MAX_TIME_STEP {
			let clock_cycles = seconds * CLOCK_SPEED - self.cycle_error;
			self.cycle_error = clock_cycles % 1.0;
			clock_cycles
		} else {
			self.cycle_error = 0.0;
			MAX_TIME_STEP * CLOCK_SPEED
		};

		for _i in 0..clock_cycles as u64 {
			self.computer.tick(false);
		}

		gamepads::update(ctx, &mut self.computer.ram);

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, BLACK);
		self.screen.render(ctx, &self.computer.ram);

		graphics::present(ctx)?;
		
		let attributes = match self.computer.ram.read(crate::VATTRIBUTES) {
			Ok(value) => value,
			Err(_err) => {println!("Could not read VATTRIBUTES, defaulting to 0"); 0},
		};
		
		let vblanked_attributes = attributes | 0b0000000010000000;

		match self.computer.ram.write(crate::VATTRIBUTES, vblanked_attributes) {
			Ok(_) => (),
			Err(err) => println!("While setting V-blank bit: {}", err.message),
		};
		
		Ok(())
	}
	
	fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
		self.screen.window_resize(width, height);
	}
}

fn load_rom(path: &String, ram: &mut Box<charcoal_mem::CharcoalMem>) {
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
		let mut new_ram = charcoal_mem::CharcoalMem::new();
		load_rom(&args[1], &mut new_ram);
		let computer = asm_19::Computer::new(new_ram);

		println!("Successfully loaded ROM from {}", args[1]);

		let mut window_mode = ggez::conf::WindowMode::default();
		window_mode.width = 512.0;
		window_mode.height = 512.0;
		window_mode.min_width = 256.0;
		window_mode.min_height = 256.0;
		window_mode.resizable = true;

		let window_conf = conf::WindowSetup {
			title: "Charcoal-16".to_owned(),
			samples: ggez::conf::NumSamples::Zero,
			vsync: true,
			icon: "".to_owned(),
			srgb: true
		};

		let c_conf = conf::Conf::new();
		
		
		let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Charcoal-16", "abledbody")
			.conf(c_conf)
			.window_mode(window_mode)
			.window_setup(window_conf)
			.build()
			.unwrap();
		
		let new_screen = display::Display::new(ctx);
		
		let state = &mut State {
			dt: std::time::Duration::new(0, 0),
			cycle_error: 0.0,
			screen: new_screen,
			computer,
		};

		event::run(ctx, event_loop, state).unwrap();
	}
}
