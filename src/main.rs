use ggez::*;
use asm_19;
use std::time::Duration;

mod display;

const BLACK: graphics::Color = graphics::Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0};

struct State {
	dt: Duration,
	screen: display::Display,
	computer: asm_19::Computer,
}

impl event::EventHandler for State {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.dt = timer::delta(ctx);
		//ggez::timer::sleep(Duration::new(0, 500000000));
		self.computer.cpu.tick();
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, BLACK);
		self.screen.render(ctx);

		graphics::present(ctx)?;
		Ok(())
	}
	
	fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
		self.screen.window_resize(width, height);
	}
}

fn main() {
	let args: Vec<String> = std::env::args().collect();

	if args.len() < 2 {
		println!("Please provide a path to the binary file for Charcoal-16 to execute.");
	}
	else{
		let computer = asm_19::Computer::new();
		asm_19::load_rom(&computer, &args[1]);

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
		
		let new_screen = display::Display::new(ctx, computer.ram.clone());
		
		let state = &mut State {
			dt: std::time::Duration::new(0, 0),
			screen: new_screen,
			computer,
		};

		event::run(ctx, event_loop, state).unwrap();
	}
}
