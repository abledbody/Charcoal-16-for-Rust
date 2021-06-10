use sdl2::pixels::Color;

/// The different palettes available to Charcoal-16. Currently only the first 3 are anything other than black.
pub const PALETTES: [[Color; 16]; 8] = [
	[ // High-Saturation CGA
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	170,	a:	255}, // Dark blue
	Color	{r:	000,	g:	170,	b:	000,	a:	255}, // Dark green
	Color	{r:	085,	g:	170,	b:	170,	a:	255}, // Dark cyan

	Color	{r:	085,	g:	000,	b:	000,	a:	255}, // Dark red
	Color	{r:	085,	g:	000,	b:	085,	a:	255}, // Dark magenta
	Color	{r:	085,	g:	085,	b:	000,	a:	255}, // Dark yellow
	Color	{r:	085,	g:	085,	b:	085,	a:	255}, // Light grey

	Color	{r:	085,	g:	085,	b:	085,	a:	255}, // Dark grey
	Color	{r:	000,	g:	000,	b:	255,	a:	255}, // Blue
	Color	{r:	000,	g:	255,	b:	000,	a:	255}, // Green
	Color	{r:	000,	g:	255,	b:	255,	a:	255}, // Cyan

	Color	{r:	255,	g:	000, 	b:	000,	a:	255}, // Red
	Color	{r:	255,	g:	000, 	b:	255,	a:	255}, // Magenta
	Color	{r:	255,	g:	255, 	b:	000,	a:	255}, // Yellow
	Color	{r:	255,	g:	255, 	b:	255,	a:	255}, // White
	],

	[ // CGA
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	170,	a:	255}, // Dark blue
	Color	{r:	000,	g:	170,	b:	000,	a:	255}, // Dark green
	Color	{r:	085,	g:	170,	b:	170,	a:	255}, // Dark cyan

	Color	{r:	170,	g:	000,	b:	000,	a:	255}, // Dark red
	Color	{r:	170,	g:	000,	b:	170,	a:	255}, // Dark magenta
	Color	{r:	170,	g:	170,	b:	000,	a:	255}, // Dark yellow
	Color	{r:	170,	g:	170,	b:	170,	a:	255}, // Light grey

	Color	{r:	085,	g:	085,	b:	085,	a:	255}, // Dark grey
	Color	{r:	085,	g:	085,	b:	255,	a:	255}, // Light Blue
	Color	{r:	085,	g:	255,	b:	085,	a:	255}, // Light Green
	Color	{r:	085,	g:	255,	b:	255,	a:	255}, // Light Cyan

	Color	{r:	255,	g:	085,	b:	085,	a:	255}, // Light Red
	Color	{r:	255,	g:	085,	b:	255,	a:	255}, // Light Magenta
	Color	{r:	255,	g:	255,	b:	085,	a:	255}, // Light Yellow
	Color	{r:	255,	g:	255,	b:	255,	a:	255}, // White
	],

	[ // Rayleigh
	Color	{r:	005,	g:	005,	b:	006,	a:	255}, // Black
	Color	{r:	025,	g:	039,	b:	057,	a:	255}, // Dark blue
	Color	{r:	085,	g:	024,	b:	035,	a:	255}, // Maroon
	Color	{r:	036,	g:	076,	b:	007,	a:	255}, // Dark green

	Color	{r:	136,	g:	081,	b:	053,	a:	255}, // Brown
	Color	{r:	069,	g:	069,	b:	076,	a:	255}, // Dark grey
	Color	{r:	144,	g:	143,	b:	136,	a:	255}, // Light grey
	Color	{r:	255,	g:	251,	b:	232,	a:	255}, // White

	Color	{r:	182,	g:	010,	b:	004,	a:	255}, // Red
	Color	{r:	255,	g:	110,	b:	017,	a:	255}, // Orange
	Color	{r:	255,	g:	236,	b:	098,	a:	255}, // Yellow
	Color	{r:	112,	g:	161,	b:	067,	a:	255}, // Light green

	Color	{r:	139,	g:	182,	b:	210,	a:	255}, // Light blue
	Color	{r:	090,	g:	069,	b:	180,	a:	255}, // Periwinkle
	Color	{r:	240,	g:	099,	b:	145,	a:	255}, // Pink
	Color	{r:	244,	g:	190,	b:	139,	a:	255}, // Tan
	],

	[ // Blank
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	],
	
	[ // Blank
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	],
	
	[ // Blank
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	],
	
	[ // Blank
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	],
	
	[ // Blank
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black

	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	Color	{r:	000,	g:	000,	b:	000,	a:	255}, // Black
	],
];
