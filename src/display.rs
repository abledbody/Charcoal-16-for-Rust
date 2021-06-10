use asm_19::memory::Memory;
use sdl2::*;
use sdl2::rect::Rect;
use sdl2::render::*;
use sdl2::video::Window;

const PADDING: u16 = 2;
const COLUMNS: u16 = 42;
const ROWS: u16 = 28;
const CHAR_WIDTH: u16 = 6;
const CHAR_HEIGHT: u16 = 9;
const WIDTH: u16 = CHAR_WIDTH * COLUMNS;
const HEIGHT: u16 = CHAR_HEIGHT * ROWS;
pub const CHARACTER_PATH: &str = "resources\\font.bmp";

pub struct Display {
	pub canvas: sdl2::render::Canvas<Window>,
	pub screen_layout: Rect,
	pub screen_scale: u32,
}

impl Display {
	pub fn new(sdl_ctx: &Sdl) -> Display {
		let video_subsys = sdl_ctx.video().unwrap();
		
		let w = (WIDTH + PADDING * 2) as u32;
		let h = (HEIGHT + PADDING * 2) as u32;
		
		let mut window = video_subsys.window("Charcoal-16", w * 2, h * 2)
			.position_centered()
			.resizable()
			.build()
			.unwrap();
		
		window.set_minimum_size(w, h)
			.unwrap();
		
		let canvas = window.into_canvas().build().unwrap();
		
		let mut display = Display {
			canvas,
			screen_layout: Rect::new(2, 2, WIDTH as u32, HEIGHT as u32),
			screen_scale: 1,
		};
		
		display.window_resize(w * 2, h * 2);
		
		display
	}
	
	pub fn window_resize(&mut self, width: u32, height: u32) {
		let width_scale	= width / (WIDTH + PADDING * 2) as u32;
		let height_scale = height / (HEIGHT + PADDING * 2) as u32;

		let max_scale = std::cmp::min(width_scale, height_scale);

		let canvas_width = WIDTH as u32 * max_scale;
		let canvas_height = HEIGHT as u32 * max_scale;

		let h_offset = ((width - canvas_width) / 2) as i32;
		let v_offset = ((height - canvas_height) / 2) as i32;
		
		self.screen_layout = rect::Rect::new(
			h_offset, v_offset,
			max_scale * WIDTH as u32, max_scale * HEIGHT as u32);
		self.screen_scale = max_scale;
	}
	
	pub fn render(&mut self, ram: &dyn Memory, font: &mut Texture) {
		let attributes = match ram.read(crate::VATTRIBUTES) {
			Ok(value) => value,
			Err(_err) => {println!("Could not read VATTRIBUTES, defaulting to 0"); 0},
		};
		
		let vram_offset = attributes >> 8;
		let vram_read_location = crate::VRAM - vram_offset * COLUMNS as u16;

		let palette = attributes >> 4 & 0b111; // 0000,0000,0111,0000
		let palette = crate::palettes::PALETTES[palette as usize];

		let padding_color_index = attributes & 0xF;
		self.canvas.set_draw_color(palette[padding_color_index as usize]);

		self.canvas.clear();
		
		{
			for x in 0..COLUMNS as i32 {
				for y in 0..ROWS as i32 {
					let short_check = ram.read(vram_read_location + x as u16 + y as u16 * COLUMNS);
					let short = match short_check {
						Ok(value) => value,
						Err(err) => panic!("{}", err.message),
					};

					let char_index = short & 0b11111111; // 0000,0000,1111,1111;
					let color_indexes = short >> 8; // 1111,1111,0000,0000;
					let fg_color_index = color_indexes & 0b1111; // 0000,1111,0000,0000
					let bg_color_index = color_indexes >> 4; // 1111,0000,0000,0000
					

					let target_rect = Rect::new(
						x * CHAR_WIDTH as i32 * self.screen_scale as i32 + self.screen_layout.x,
						y * CHAR_HEIGHT as i32 * self.screen_scale as i32 + self.screen_layout.y,
						CHAR_WIDTH as u32 * self.screen_scale,
						CHAR_HEIGHT as u32 * self.screen_scale,
					);
					
					self.canvas.set_draw_color(palette[bg_color_index as usize]);
					self.canvas.fill_rect(target_rect).unwrap();

					
					let char_source = Rect::new(
						char_index as i32 % 16 * CHAR_WIDTH as i32,
						char_index as i32 / 16 * CHAR_HEIGHT as i32,
						CHAR_WIDTH as u32,
						CHAR_HEIGHT as u32,
					);
					
					let color = palette[fg_color_index as usize];
					font.set_color_mod(color.r, color.g, color.b);
					
					self.canvas.copy(font, char_source, target_rect).unwrap();
				}
			}
		}
	}
}

/*
use asm_19::memory::Memory;
use ggez::*;
use mint;

const PADDING: u16 = 2;
const COLUMNS: u16 = 42;
const ROWS: u16 = 28;
const CHAR_WIDTH: u16 = 6;
const CHAR_HEIGHT: u16 = 9;
const WIDTH: u16 = CHAR_WIDTH * COLUMNS;
const HEIGHT: u16 = CHAR_HEIGHT * ROWS;
const CHARACTER_PATH: &str = "/font.png";

pub struct Display {
	canvas: graphics::Canvas,
	canvas_params: graphics::DrawParam,
	character_batch: graphics::spritebatch::SpriteBatch,
	background_batch: graphics::spritebatch::SpriteBatch,
	char_screen_params: graphics::DrawParam,
	screen_size: mint::Point2<f32>,
}

impl Display {
	pub fn new(ctx: &mut ggez::Context) -> Display {
		let mut canvas = graphics::Canvas::new(ctx, WIDTH, HEIGHT, ggez::conf::NumSamples::One).unwrap();

		canvas.set_filter(graphics::FilterMode::Nearest);

		let character_set = graphics::Image::new(ctx, CHARACTER_PATH).unwrap();
		let blank = graphics::Image::solid(ctx, 1, graphics::WHITE).unwrap();

		let canvas_params = graphics::DrawParam::new()
			.dest(mint::Point2 {x: 2.0, y: 2.0});
		let char_screen_params = graphics::DrawParam::new();

		let character_batch = graphics::spritebatch::SpriteBatch::new(character_set);
		let background_batch = graphics::spritebatch::SpriteBatch::new(blank);

		Display {
			canvas,
			canvas_params,
			char_screen_params,
			character_batch,
			background_batch,
			screen_size: mint::Point2 {x: 512.0, y: 512.0},
		}
	}

	pub fn window_resize(&mut self, width: f32, height: f32) {
		let width_scale	= width	as u16 / (WIDTH + PADDING * 2);
		let height_scale = height as u16 / (HEIGHT + PADDING * 2);

		

		let max_scale = std::cmp::min(width_scale, height_scale);

		let canvas_width = (WIDTH * max_scale) as f32;
		let canvas_height = (HEIGHT * max_scale) as f32;

		let h_offset = (width - canvas_width) / 2.0;
		let v_offset = (height - canvas_height) / 2.0;
		
		let canvas_params = self.canvas_params.dest(mint::Point2 {x: h_offset, y: v_offset})
			.scale(mint::Point2 {x: max_scale as f32, y: max_scale as f32});

		self.screen_size = mint::Point2 {x: width, y: height};
		
		self.canvas_params = canvas_params;
	}

	pub fn render(&mut self, ctx: &mut ggez::Context, ram: &dyn Memory) {
		use graphics::Drawable;

		let attributes = match ram.read(crate::VATTRIBUTES) {
			Ok(value) => value,
			Err(_err) => {println!("Could not read VATTRIBUTES, defaulting to 0"); 0},
		};

		let vram_offset = attributes >> 8;
		let vram_read_location = crate::VRAM - vram_offset * COLUMNS;

		let palette = attributes >> 4 & 0b111; // 0000,0000,0111,0000
		let palette = crate::palettes::PALETTES[palette as usize];

		self.character_batch.clear();
		self.background_batch.clear();

		graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32)).unwrap();

		{
			for x in 0..COLUMNS {
				for y in 0..ROWS {
					let short_check = ram.read(vram_read_location + x + y * COLUMNS);
					let short = match short_check {
						Ok(value) => value,
						Err(err) => panic!("{}", err.message),
					};

					let char_index = short & 0b11111111; // 0000,0000,1111,1111;
					let color_indexes = short >> 8; // 1111,1111,0000,0000;
					let fg_color_index = color_indexes & 0b1111; // 0000,1111,0000,0000
					let bg_color_index = color_indexes >> 4; // 1111,0000,0000,0000

					let char_source = graphics::Rect::new(
						(char_index % 16) as f32 / 16.0,
						(char_index / 16) as f32 / 16.0,
						1.0/16.0,
						1.0/16.0,
					);

					let background_rect = graphics::DrawParam::new()
						.src(graphics::Rect::new(1.0, 1.0, 1.0, 1.0))
						.dest(mint::Point2 {x: (x * CHAR_WIDTH) as f32, y: (y * CHAR_HEIGHT) as f32})
						.scale(mint::Point2 {x: CHAR_WIDTH as f32, y: CHAR_HEIGHT as f32})
						.color(palette[bg_color_index as usize]);

					self.background_batch.add(background_rect);

					let new_character = graphics::DrawParam::new()
						.src(char_source)
						.dest(mint::Point2 {x: (x * CHAR_WIDTH) as f32, y: (y * CHAR_HEIGHT) as f32})
						.color(palette[fg_color_index as usize]);
					self.character_batch.add(new_character);
				}
			}
		}

		graphics::set_canvas(ctx, Some(&self.canvas));
		graphics::clear(ctx, graphics::BLACK);
		self.background_batch.draw(ctx, self.char_screen_params).unwrap();
		self.character_batch.draw(ctx, self.char_screen_params).unwrap();

		graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, self.screen_size.x, self.screen_size.y)).unwrap();
		graphics::set_canvas(ctx, None);

		let background_color_index = attributes & 0xF;

		graphics::clear(ctx, palette[background_color_index as usize]);
		self.canvas.draw(ctx, self.canvas_params).unwrap();
		self.canvas.dimensions(ctx);
	}
}
*/