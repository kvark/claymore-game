extern mod freetype;

use core::ptr;
use core::vec;

use freetype::freetype::*;

use context;
use context::GLType;
use texture;


static SHIFT : int = 6;

trait FontError	{
	fn check( &self, s : &str );
}

impl FontError for FT_Error	{
	fn check( &self, s : &str )	{
		if (*self as int)!=0	{
			fail!(fmt!( "Freetype %s failed with code %d", s, *self as int ))
		}
	}
}


pub struct Context	{
	priv lib	: FT_Library,
}

impl Drop for Context	{
	fn finalize( &self )	{
		assert!( self.lib.is_not_null() );
		bindgen::FT_Done_FreeType( self.lib ).
			check( "Done_FreeType" );
	}
}

pub impl Context	{
	fn create()-> Context	{
		let mut lib : FT_Library = ptr::null();
		bindgen::FT_Init_FreeType( ptr::addr_of(&lib) ).
			check( "Init_FreeType" );
		Context{ lib:lib }
	}
}


/*//TODO: enable when supported
struct Glyph	{
	slot	: freetype::FT_GlyphSlot,
}
impl Drop for Glyph	{
	fn finalize( &self )	{
		freetype::bindgen::FT_Done_Glyph( self.slot );
	}
}
*/


pub struct Font	{
	priv face		: FT_Face,
	kern_offset		: int,
	line_offset		: int,
	priv context	: @Context,
	//priv mut cache	: send_map::linear::LinearMap<char,Glyph>,
}

#[unsafe_destructor]
impl Drop for Font	{
	fn finalize( &self )	{
		bindgen::FT_Done_Face( self.face ).
			check( "Done_Face" );
	}
}


impl Font	{
	/*
	pub fn clear_cache()	{
		self.cache.clear();
	}
	//TODO: enable when supported by FreeType
	pub fn load_glyph( &self, c : char )-> &self/Glyph	{
		match copy self.cache.find(c)	{
			Some(g) => &g,
			None =>	{
				freetype::bindgen::FT_Load_Char( self.face, c as freetype::FT_ULong,
					freetype::FT_LOAD_DEFAULT as freetype::FT_Int32 )
					.check( "Load_Char" );
				let face  = unsafe { &*(self.face) };
				freetype::bindgen::FT_Render_Glyph(
					face.glyph as freetype::FT_GlyphSlot,
					freetype::FT_RENDER_MODE_NORMAL )
					.check( "Render_Glyph" );
				let mut slot : freetype::FT_GlyphSlot = ptr::null();
				freetype::bindgen::FT_Get_Glyph( self.face,	ptr::addr_of(&slot))
					.check( "Get_Glyph" );
				let g = Glyph{ slot:slot };
				self.cache.insert( c, slot );
				g
			}
		}
	}*/

	priv fn set_char_size( &self, xs : float, ys : float, xdpi : uint, ydpi : uint )	{
		bindgen::FT_Set_Char_Size( self.face,
			64f*xs as FT_F26Dot6, 64f*ys as FT_F26Dot6,
			xdpi as FT_UInt, ydpi as FT_UInt
			).check( "Set_Char_Size" );
	}

	priv fn set_pixel_size( &self, xpix : uint, ypix : uint )	{
		bindgen::FT_Set_Pixel_Sizes( self.face,
			xpix as FT_UInt, ypix as FT_UInt
			).check( "Set_Pixel_Sizes" );
	}

	priv fn draw( &self, bm : &FT_Bitmap, target : &mut ~[u8], offset : uint, pitch : uint )	{
		//TODO: use &[mut u8] argument
		let height = bm.rows as uint;
		for uint::range(0,height) |y|	{
			unsafe	{
				let src = ptr::offset( bm.buffer, y*(bm.pitch as uint)) as *u8;
				let dst = ptr::offset( vec::raw::to_ptr(*target), y*pitch+offset ) as *mut u8;
				ptr::copy_memory( dst, src, bm.width as uint );
			};
		}
	}

	pub fn bake( &self, gr : &mut context::Context, s : &str, max_size : (uint,uint), lg : &context::Log )-> texture::Texture	{
		lg.add(fmt!( "Font baking text: %s", s ));
		if s.is_empty()	{
			let mut tex = gr.create_texture( ~"2D", 1u, 1u, 1u, 0u );
			let mut image = vec::from_elem( 1u, 0u8 );
			gr.texture.load_2D( &mut tex, 0u, texture::map_int_format(~"r8"),
				texture::map_pix_format(~"red"), image[0].to_gl_type(), &image );
			return tex
		}
		struct Target	{ c:char, x:int, y:int }
		let (limit_x,limit_y) = max_size;
		let face  = unsafe { &*(self.face) };
		let line_gap = (self.line_offset as int) + (face.height as int);
		let mut position = 0, baseline = face.ascender as int;	// in font units
		lg.add(fmt!( "\tFace height=%d, up=%d, down=%d", face.height as int,
			face.ascender as int, face.descender as int ));
		let mut prev_index = 0 as FT_UInt;	// font char index
		let BIG		= 999999;
		let BORDER	= 1;
		let ALIGN	= 3;
		let mut max_x = -BIG, max_y = -BIG, min_x = BIG, min_y = BIG;	// in font units
		let mut start_word = 0u;
		let width_capacity = limit_x as int << SHIFT;
		let mut pos_array = vec::with_capacity::<Target>( s.len() );
		do str::each_char(s) |c|	{
			if c == '\n'	{
				baseline += line_gap;
				position = 0;
				prev_index = 0 as FT_UInt;
			}else	{
				if char::is_whitespace(c)	{
					start_word = pos_array.len();
				}
				let index = bindgen::FT_Get_Char_Index( self.face, c as FT_ULong );
				bindgen::FT_Load_Glyph( self.face, index, FT_LOAD_DEFAULT as FT_Int32 ).
					check( "Load_Glyph" );
				position += unsafe{
					let delta = struct_FT_Vector_{ x:0, y:0 };
					bindgen::FT_Get_Kerning( self.face, prev_index, index,
						FT_KERNING_DEFAULT, ptr::addr_of(&delta) ).
						check( "Get_Kerning" );
					//lg.add(fmt!( "\tKerning %d-%d is %d",
					//	prev_index as int, index as int, delta.x as int ));
					delta.x as int + self.kern_offset
				};
				prev_index = index;
				let glyph = unsafe { &*(face.glyph as FT_GlyphSlot) };
				assert!( self.face as uint == glyph.face as uint );
				let cx = position + glyph.metrics.horiBearingX as int;
				let cy = baseline - glyph.metrics.horiBearingY as int;
				pos_array.push( Target{ c:c, x:cx, y:cy });
				min_x = int::min( min_x, cx );
				min_y = int::min( min_y, cy );
				let mut ex = cx + glyph.metrics.width	as int;
				let mut ey = cy + glyph.metrics.height	as int;
				let e_border = (ex - min_x + (2*BORDER<<SHIFT)) | (((ALIGN+1)<<SHIFT)-1);
				if e_border >= width_capacity	{
					lg.add(fmt!( "\tMoving the word: %u-%u", start_word, pos_array.len() ));
					let word_offset = pos_array[start_word].x - min_x;
					if e_border - word_offset >= width_capacity	{
						fail!(fmt!( "Text exceeds horisontal bound: %s", s ))
					}
					lg.add(fmt!( "\tHor:%d Ver:%d", -word_offset, line_gap ));
					prev_index = 0 as FT_UInt;
					for uint::range( start_word, pos_array.len() ) |i|	{
						pos_array[i].x -= word_offset;
						pos_array[i].y += line_gap;
					}
					position -= word_offset;
					baseline += line_gap;
					ex -= word_offset;
					ey += line_gap;
				}
				max_x = int::max( max_x, ex );
				max_y = int::max( max_y, ey );
				//lg.add(fmt!( "\tsymbol:%c cx:%d cy:%d", c, cx, cy ));
				/*lg.add(fmt!( "\tSymbol '%c' (id=%u) at (%u,%u): size=(%d,%d) bearing=(%d,%d)",
					c, self.get_char_index(c), position, baseline,
					glyph.metrics.width as int, glyph.metrics.height as int,
					glyph.metrics.horiBearingX as int, glyph.metrics.horiBearingY as int ));*/
				position += glyph.advance.x as int;
			}
			true
		}
		// add border and align to 4 bytes
		let width = ((((SHIFT+max_x-min_x)>>SHIFT)+ALIGN+2*BORDER) & !ALIGN) as uint;
		let height= ((((SHIFT+max_y-min_y)>>SHIFT)+ALIGN+2*BORDER) & !ALIGN) as uint;
		min_x -= BORDER<<SHIFT; min_y -= BORDER<<SHIFT; 
		lg.add(fmt!( "\tBox at (%d,%d) of size %ux%u", min_x, min_y, width, height ));
		assert!( width<=limit_x && height<=limit_y );
		let mut image = vec::from_elem( width*height, 0u8 );
		for pos_array.each |slice|	{
			bindgen::FT_Load_Char( self.face,
				slice.c as FT_ULong, FT_LOAD_DEFAULT as FT_Int32 ).
				check( "Load_Char" );
			bindgen::FT_Render_Glyph(
				face.glyph as FT_GlyphSlot, FT_RENDER_MODE_NORMAL ).
				check( "Render_Glyph" );
			let glyph = unsafe { &*(face.glyph as FT_GlyphSlot) };
			let bmp = &(glyph.bitmap);
			let bw = bmp.width as uint, bh = bmp.rows as uint;
			assert!( bw == glyph.metrics.width	as uint >> SHIFT );
			assert!( bh == glyph.metrics.height	as uint >> SHIFT );
			let x = ((slice.x - min_x) >>SHIFT) as uint;
			let y = ((slice.y - min_y) >>SHIFT) as uint;
			//lg.add(fmt!( "\tx:%u bw:%u, width:%u, y:%u bh:%u height:%u", x,bw,width, y,bh,height ));
			assert!( x + bw <= width && y + bh <= height );
			self.draw( bmp, &mut image, y*width + x, width );
		}
		let mut tex = gr.create_texture( ~"2D", width, height, 1u, 0u );
		gr.texture.load_2D( &mut tex, 0u, texture::map_int_format(~"r8"),
			texture::map_pix_format(~"red"), image[0].to_gl_type(), &image );
		tex
	}
}


pub impl Context	{
	fn load_font( @self, path : &str, index : uint, xs : uint, ys : uint,
			kerning : float, line_gap : float )-> Font	{
		let mut face : FT_Face = ptr::null();
		do str::as_c_str(path) |text|	{
			bindgen::FT_New_Face( self.lib, text, 
				index as FT_Long, ptr::addr_of(&face) ).
				check( "New_Face" );
		}
		bindgen::FT_Set_Pixel_Sizes( face,
			xs as FT_UInt, ys as FT_UInt ).
			check( "Set_Pixel_Sizes" );
		Font{ face:face, context:self,
			kern_offset: kerning * ((1<<SHIFT) as float) as int,
			line_offset: line_gap* ((1<<SHIFT) as float) as int,
			//cache	:send_map::linear::LinearMap::<char,Glyph>(),
		}
	}
}
