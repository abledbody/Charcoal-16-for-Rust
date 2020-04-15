use asm_19::memory;
use ggez::*;
use mint;
use std::cell::RefCell;
use std::rc::Rc;

const PADDING: u16 = 2;
const COLUMNS: u16 = 42;
const ROWS: u16 = 28;
const CHAR_WIDTH: u16 = 6;
const CHAR_HEIGHT: u16 = 9;
const WIDTH: u16 = CHAR_WIDTH * COLUMNS;
const HEIGHT: u16 = CHAR_HEIGHT * ROWS;
const CHARACTER_PATH: &str = "/font.png";
const VRAM: u16 = 0x0000;//0xFB68;

const PALETTE: [graphics::Color; 16] = [
	graphics::Color	{r:	0.078,	g:	0.082, b:	0.094,	a:	1.0}, // Black
	graphics::Color	{r:	0.122,	g:	0.192, b:	0.272,	a:	1.0}, // Dark blue
	graphics::Color	{r:	0.380,	g:	0.106, b:	0.157,	a:	1.0}, // Maroon
	graphics::Color	{r:	0.188,	g:	0.349, b:	0.031,	a:	1.0}, // Dark green

	graphics::Color	{r:	0.565,	g:	0.365, b:	0.263,	a:	1.0}, // Brown
	graphics::Color	{r:	0.298,	g:	0.298, b:	0.318,	a:	1.0}, // Dark grey
	graphics::Color	{r:	0.537,	g:	0.533, b:	0.510,	a:	1.0}, // Light grey
	graphics::Color	{r:	1.000,	g:	0.984, b:	0.918,	a:	1.0}, // White

	graphics::Color	{r:	0.733,	g:	0.106, b:	0.086,	a:	1.0}, // Red
	graphics::Color	{r:	1.000,	g:	0.435, b:	0.075,	a:	1.0}, // Orange
	graphics::Color	{r:	1.000,	g:	0.925, b:	0.384,	a:	1.0}, // Yellow
	graphics::Color	{r:	0.498,	g:	0.659, b:	0.275,	a:	1.0}, // Light green

	graphics::Color	{r:	0.553,	g:	0.722, b:	0.835,	a:	1.0}, // Light blue
	graphics::Color	{r:	0.361,	g:	0.278, b:	0.725,	a:	1.0}, // Periwinkle
	graphics::Color	{r:	0.945,	g:	0.388, b:	0.569,	a:	1.0}, // Pink
	graphics::Color	{r:	0.961,	g:	0.749, b:	0.549,	a:	1.0}, // Tan
];


pub struct Display {
	ram: Rc<RefCell<memory::Memory>>,
	canvas: graphics::Canvas,
	canvas_params: graphics::DrawParam,
	char_screen_params: graphics::DrawParam,
	character_set: graphics::Image,
	blank: graphics::Image,
	screen_size: mint::Point2<f32>,
}

impl Display {
	pub fn new(ctx: &mut ggez::Context, ram: Rc<RefCell<memory::Memory>>) -> Display {
		let mut canvas = graphics::Canvas::new(ctx, WIDTH, HEIGHT, ggez::conf::NumSamples::One).unwrap();

		canvas.set_filter(graphics::FilterMode::Nearest);

		let character_set = graphics::Image::new(ctx, CHARACTER_PATH).unwrap();
		let blank = graphics::Image::solid(ctx, 1, graphics::WHITE).unwrap();

		let canvas_params = graphics::DrawParam::new()
			.dest(mint::Point2 {x: 2.0, y: 2.0});
		let char_screen_params = graphics::DrawParam::new();

		Display {
			ram,
			canvas,
			canvas_params,
			char_screen_params,
			character_set,
			blank,
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

	pub fn render(&self, ctx: &mut ggez::Context) {
		use graphics::Drawable;
		
		let mut character_batch = graphics::spritebatch::SpriteBatch::new(self.character_set.clone());
		let mut background_batch = graphics::spritebatch::SpriteBatch::new(self.blank.clone());

		graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32)).unwrap();

		{
			let ram = self.ram.try_borrow().unwrap();
			for x in 0..COLUMNS {
				for y in 0..ROWS {
					let short_check = ram.read(VRAM + x + y * COLUMNS);
					let short = match short_check {
						Ok(value) => value,
						Err(err) => panic!(err.message),
					};

					let char_index = short & 0x00FF;
					let color_indexes = short >> 8;
					let fg_color_index = color_indexes & 0x0F;
					let bg_color_index = color_indexes >> 4;

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
						.color(PALETTE[bg_color_index as usize]);

					background_batch.add(background_rect);

					let new_character = graphics::DrawParam::new()
						.src(char_source)
						.dest(mint::Point2 {x: (x * CHAR_WIDTH) as f32, y: (y * CHAR_HEIGHT) as f32})
						.color(PALETTE[fg_color_index as usize]);
					character_batch.add(new_character);
				}
			}
		}

		graphics::set_canvas(ctx, Some(&self.canvas));
		graphics::clear(ctx, graphics::BLACK);
		background_batch.draw(ctx, self.char_screen_params).unwrap();
		character_batch.draw(ctx, self.char_screen_params).unwrap();

		graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, self.screen_size.x, self.screen_size.y)).unwrap();
		graphics::set_canvas(ctx, None);
		self.canvas.draw(ctx, self.canvas_params).unwrap();
		self.canvas.dimensions(ctx);
	}
}