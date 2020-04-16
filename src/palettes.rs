use ggez::graphics;

pub const PALETTES: [[graphics::Color; 16]; 8] = [
	[ // High-Saturation CGA
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.667,	a:	1.0}, // Dark blue
	graphics::Color	{r:	0.000,	g:	0.667, b:	0.000,	a:	1.0}, // Dark green
	graphics::Color	{r:	0.298,	g:	0.667, b:	0.667,	a:	1.0}, // Dark cyan

	graphics::Color	{r:	0.667,	g:	0.000, b:	0.000,	a:	1.0}, // Dark red
	graphics::Color	{r:	0.667,	g:	0.000, b:	0.667,	a:	1.0}, // Dark magenta
	graphics::Color	{r:	0.667,	g:	0.667, b:	0.000,	a:	1.0}, // Dark yellow
	graphics::Color	{r:	0.667,	g:	0.667, b:	0.667,	a:	1.0}, // Light grey

	graphics::Color	{r:	0.333,	g:	0.333, b:	0.333,	a:	1.0}, // Dark grey
	graphics::Color	{r:	0.000,	g:	0.000, b:	1.000,	a:	1.0}, // Blue
	graphics::Color	{r:	0.000,	g:	1.000, b:	0.000,	a:	1.0}, // Green
	graphics::Color	{r:	0.000,	g:	1.000, b:	1.000,	a:	1.0}, // Cyan

	graphics::Color	{r:	1.000,	g:	0.000, b:	0.000,	a:	1.0}, // Red
	graphics::Color	{r:	1.000,	g:	0.000, b:	1.000,	a:	1.0}, // Magenta
	graphics::Color	{r:	1.000,	g:	1.000, b:	0.000,	a:	1.0}, // Yellow
	graphics::Color	{r:	1.000,	g:	1.000, b:	1.000,	a:	1.0}, // White
	],

	[ // CGA
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.667,	a:	1.0}, // Dark blue
	graphics::Color	{r:	0.000,	g:	0.667, b:	0.000,	a:	1.0}, // Dark green
	graphics::Color	{r:	0.298,	g:	0.667, b:	0.667,	a:	1.0}, // Dark cyan

	graphics::Color	{r:	0.667,	g:	0.000, b:	0.000,	a:	1.0}, // Dark red
	graphics::Color	{r:	0.667,	g:	0.000, b:	0.667,	a:	1.0}, // Dark magenta
	graphics::Color	{r:	0.667,	g:	0.667, b:	0.000,	a:	1.0}, // Dark yellow
	graphics::Color	{r:	0.667,	g:	0.667, b:	0.667,	a:	1.0}, // Light grey

	graphics::Color	{r:	0.333,	g:	0.333, b:	0.333,	a:	1.0}, // Dark grey
	graphics::Color	{r:	0.333,	g:	0.333, b:	1.000,	a:	1.0}, // Light Blue
	graphics::Color	{r:	0.333,	g:	1.000, b:	0.333,	a:	1.0}, // Light Green
	graphics::Color	{r:	0.333,	g:	1.000, b:	1.000,	a:	1.0}, // Light Cyan

	graphics::Color	{r:	1.000,	g:	0.333, b:	0.333,	a:	1.0}, // Light Red
	graphics::Color	{r:	1.000,	g:	0.333, b:	1.000,	a:	1.0}, // Light Magenta
	graphics::Color	{r:	1.000,	g:	1.000, b:	0.333,	a:	1.0}, // Light Yellow
	graphics::Color	{r:	1.000,	g:	1.000, b:	1.000,	a:	1.0}, // White
	],

	[ // Rayleigh
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
	],

	[ // Blank
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	],
	
	[ // Blank
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	],
	
	[ // Blank
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	],
	
	[ // Blank
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	],
	
	[ // Blank
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black

	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	graphics::Color	{r:	0.000,	g:	0.000, b:	0.000,	a:	1.0}, // Black
	],
];
