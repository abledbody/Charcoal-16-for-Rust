use asm_19::memory::Memory;
use sdl2::{*, rect::Rect, video::Window, render::Texture};

const PADDING: u16 = 2;
const COLUMNS: u16 = 42;
const ROWS: u16 = 28;
const CHAR_WIDTH: u16 = 6;
const CHAR_HEIGHT: u16 = 9;
const WIDTH: u16 = CHAR_WIDTH * COLUMNS;
const HEIGHT: u16 = CHAR_HEIGHT * ROWS;
pub const CHARACTER_PATH: &str = "resources\\font.bmp";

pub const VRAM: u16 = 0xFB65;
pub const VATTRIBUTES: u16 = 0xFFFD;

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
		let attributes = match ram.read(VATTRIBUTES) {
			Ok(value) => value,
			Err(_err) => {println!("Could not read VATTRIBUTES, defaulting to 0"); 0},
		};
		
		let vram_offset = attributes >> 8;
		let vram_read_location = VRAM - vram_offset * COLUMNS as u16;

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
					
					let char_source = Rect::new(
						char_index as i32 % 16 * CHAR_WIDTH as i32,
						char_index as i32 / 16 * CHAR_HEIGHT as i32,
						CHAR_WIDTH as u32,
						CHAR_HEIGHT as u32,
					);
					
					let bg_color = palette[bg_color_index as usize];
					let fg_color = palette[fg_color_index as usize];
					
					//let char_palette = Palette::with_colors(&[bg_color, fg_color]).unwrap();
					//font_surface.set_palette(&char_palette).unwrap();
					
					//let font = self.canvas.create_texture_from_surface(&font_surface).unwrap();
					
					
					self.canvas.set_draw_color(bg_color);
					self.canvas.fill_rect(target_rect).unwrap();
					
					
					font.set_color_mod(fg_color.r, fg_color.g, fg_color.b);
					
					self.canvas.copy(&font, char_source, target_rect).unwrap();
				}
			}
		}
	}
}