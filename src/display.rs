use asm_19::memory::Memory;
use sdl2::{*, rect::Rect, video::Window, render::Texture};

/// The minimum thickness of the borders of the screen in pixels.
const PADDING: u16 = 2;
/// How many columns of characters there are.
const COLUMNS: u16 = 42;
/// How many rows of characters there are.
const ROWS: u16 = 28;
/// The width of a character in the font in pixels.
const CHAR_WIDTH: u16 = 6;
/// The height of a character in the font in pixels.
const CHAR_HEIGHT: u16 = 9;
/// The width of the screen in pixels. Does not count borders.
const WIDTH: u16 = CHAR_WIDTH * COLUMNS;
/// The height of the screen in pixels. Does not count borders.
const HEIGHT: u16 = CHAR_HEIGHT * ROWS;
/// The path to the font bitmap.
pub const FONT_PATH: &str = "resources\\font.bmp";

/// The first address representing the VRAM at an offset of 0.
pub const VRAM: u16 = 0xFB65;
/// The address of VATTRIBUTES, describing screen behavior.
pub const VATTRIBUTES: u16 = 0xFFFD;

/// The screen peripheral.
pub struct Display {
	/// The canvas of the window. Display draws to this.
	pub canvas: sdl2::render::Canvas<Window>,
	/// Describes the position and size of the screen. Does not include borders.
	pub screen_layout: Rect,
	/// The size of a pixel on the screen as rendered to the window.
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
	
	/// Recalculates ```screen_layout``` and ```screen_scale``` based on the size of the window.
	pub fn window_resize(&mut self, width: u32, height: u32) {
		let width_scale	= width / (WIDTH + PADDING * 2) as u32;
		let height_scale = height / (HEIGHT + PADDING * 2) as u32;

		// The largest that screen_scale can get without overstepping the padding or spilling out of the window.
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
	
	/// Draws the state of VRAM to the window.
	pub fn render(&mut self, ram: &dyn Memory, font: &mut Texture) {
		let attributes = match ram.read(VATTRIBUTES) {
			Ok(value) => value,
			Err(_err) => {println!("Could not read VATTRIBUTES, defaulting to 0"); 0},
		};
		
		let padding_color_index = attributes & 0b1111;	// 0000,0000,0000,1111
		let palette_index = attributes >> 4 & 0b111;	// 0000,0000,0111,0000
		let vram_offset = attributes >> 8;				// 1111,1111,0000,0000
		
		// The last eight bits of VATTRIBUTES defines from where the VRAM is read, vertically.
		let vram_read_location = VRAM - vram_offset * COLUMNS as u16;
		
		let palette = crate::palettes::PALETTES[palette_index as usize];
		// This will define our border color.
		self.canvas.set_draw_color(palette[padding_color_index as usize]);
		self.canvas.clear();
		
		for y in 0..ROWS {
			let y_offset = y * COLUMNS;
			for x in 0..COLUMNS {
				let byte = match ram.read(vram_read_location + x + y_offset) {
					Ok(value) => value,
					Err(err) => panic!("{}", err.message),
				};
				
				let char_index = byte & 0b11111111;			// 0000,0000,1111,1111;
				let fg_color_index = byte >> 8 & 0b1111;	// 0000,1111,0000,0000
				let bg_color_index = byte >> 12;			// 1111,0000,0000,0000
				

				let target_rect = Rect::new(
					(x * CHAR_WIDTH * self.screen_scale as u16) as i32 + self.screen_layout.x,
					(y * CHAR_HEIGHT * self.screen_scale as u16) as i32 + self.screen_layout.y,
					CHAR_WIDTH as u32 * self.screen_scale,
					CHAR_HEIGHT as u32 * self.screen_scale,
				);
				
				let char_source = Rect::new(
					char_index as i32 % 16 * CHAR_WIDTH as i32,
					char_index as i32 / 16 * CHAR_HEIGHT as i32,
					CHAR_WIDTH as u32,
					CHAR_HEIGHT as u32,
				);
				
				let bg_color = palette[bg_color_index as usize];
				let fg_color = palette[fg_color_index as usize];
				
				self.canvas.set_draw_color(bg_color);
				self.canvas.fill_rect(target_rect).unwrap();
				
				// We can't change the palette of the texture at this stage, so we have to settle for tinting it.
				font.set_color_mod(fg_color.r, fg_color.g, fg_color.b);
				self.canvas.copy(&font, char_source, target_rect).unwrap();
			}
		}
	}
}