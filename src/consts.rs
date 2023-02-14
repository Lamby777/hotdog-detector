pub const LINE_SEPARATOR:		&str		= "--------------------------------------------------";
pub const INPUT_IMAGE_SIZE:		(u32, u32)	= (426, 240);
pub const INTERMEDIATE_LAYERS:	usize		= 126;
pub const INPUT_NODES:			usize		= (INPUT_IMAGE_SIZE.0 * INPUT_IMAGE_SIZE.1) as usize;
pub const ILAYER_NODES:			usize		= (INPUT_IMAGE_SIZE.0 * INPUT_IMAGE_SIZE.1) as usize;
pub const OUTPUT_NODES:			usize		= 1;
