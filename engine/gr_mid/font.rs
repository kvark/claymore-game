extern mod freetype;

use std;
use std::{ptr,vec};

use freetype::freetype;

use gr_low::{context,texture};
use gr_low::context::GLType;
use journal;


static SHIFT : int = 6;

trait FontError	{
	fn check( &self, &str );
}

impl FontError for freetype::FT_Error	{
	fn check( &self, s: &str )	{
		if (*self as int)!=0	{
			fail!("Freetype {:s} failed with code {:i}", s, *self as int)
		}
	}
}


pub struct Context	{
	priv lib	: freetype::FT_Library,
}

impl Drop for Context	{
	fn drop( &mut self )	{
		assert!( self.lib.is_not_null() );
		unsafe{
			freetype::FT_Done_FreeType( self.lib )
		}.check( "Done_FreeType" );
	}
}

impl Context	{
	pub fn create()-> Context	{
		let lib : freetype::FT_Library = ptr::null();
		unsafe{
			freetype::FT_Init_FreeType( ptr::to_unsafe_ptr(&lib) )
		}.check( "Init_FreeType" );
		Context{ lib:lib }
	}
}


/*//TODO: enable when supported
struct Glyph	{
	slot	: freetype::FT_GlyphSlot,
}
impl Drop for Glyph	{
	fn finalize( &self )	{
		freetype::freetype::FT_Done_Glyph( self.slot );
	}
}
*/

struct FaceHandle( freetype::FT_Face );

pub struct Font	{
	priv face		: FaceHandle,
	kern_offset		: int,
	line_offset		: int,
	//priv mut cache	: send_map::linear::HashMap<char,Glyph>,
}

impl Drop for FaceHandle	{
	fn drop( &mut self )	{
		let &FaceHandle(h) = self;
		unsafe{
			freetype::FT_Done_Face(h)
		}.check( "Done_Face" );
	}
}


impl Font	{
	/*
	pub fn clear_cache()	{
		self.cache.clear();
	}
	//TODO: enable when supported by FreeType
	pub fn load_glyph( &self, c: char )-> &self/Glyph	{
		match copy self.cache.find(c)	{
			Some(g) => &g,
			None =>	{
				freetype::freetype::FT_Load_Char( self.face, c as freetype::FT_ULong,
					freetype::FT_LOAD_DEFAULT as freetype::FT_Int32 )
					.check( "Load_Char" );
				let face  = unsafe { &*(self.face) };
				freetype::freetype::FT_Render_Glyph(
					face.glyph as freetype::FT_GlyphSlot,
					freetype::FT_RENDER_MODE_NORMAL )
					.check( "Render_Glyph" );
				let mut slot : freetype::FT_GlyphSlot = ptr::null();
				freetype::freetype::FT_Get_Glyph( self.face,	ptr::to_unsafe_ptr(&slot))
					.check( "Get_Glyph" );
				let g = Glyph{ slot:slot };
				self.cache.insert( c, slot );
				g
			}
		}
	}*/

	pub fn set_char_size( &self, xs: f32, ys: f32, xdpi: uint, ydpi: uint )	{
		let FaceHandle(fh) = self.face;
		unsafe{
			freetype::FT_Set_Char_Size( fh,
				(64.0*xs) as freetype::FT_F26Dot6, (64.0*ys) as freetype::FT_F26Dot6,
				xdpi as freetype::FT_UInt, ydpi as freetype::FT_UInt )
		}.check( "Set_Char_Size" );
	}

	pub fn set_pixel_size( &self, xpix: uint, ypix: uint )	{
		let FaceHandle(fh) = self.face;
		unsafe{
			freetype::FT_Set_Pixel_Sizes( fh,
				xpix as freetype::FT_UInt, ypix as freetype::FT_UInt )
		}.check( "Set_Pixel_Sizes" );
	}

	fn draw( &self, bm: &freetype::FT_Bitmap, target: &mut [u8], offset: uint, pitch: uint )	{
		let height = bm.rows as uint;
		for y in range(0u,height)	{
			unsafe	{
				let src = ptr::offset( bm.buffer, (y*(bm.pitch as uint)) as int );
				let dst = ptr::mut_offset( target.as_mut_ptr(), (y*pitch+offset) as int );
				ptr::copy_memory( dst, src, bm.width as uint );
			};
		}
	}

	pub fn bake( &self, gr: &mut context::Context, s: &str, max_size: (uint,uint), lg: &journal::Log )-> @texture::Texture	{
		lg.add(format!( "Font baking text: {:s}", s ));
		if s.is_empty()	{
			let tex = gr.create_texture( "2D", 1u, 1u, 1u, 0u );
			let image = vec::from_elem( 1u, 0u8 );
			gr.texture.load_2D( tex, 0u, texture::map_int_format("r8"),
				texture::map_pix_format("red"), image[0].to_gl_type(), image );
			return tex
		}
		struct Target	{ c:char, x:int, y:int }
		let (limit_x,limit_y) = max_size;
		let FaceHandle(fh) = self.face;
		let face = unsafe{&*fh};
		let line_gap = (self.line_offset as int) + (face.height as int);
		let mut position = 0;	// in font units
		let mut baseline = face.ascender as int;
		lg.add(format!( "\tFace height={:i}, up={:i}, down={:i}", face.height as int,
			face.ascender as int, face.descender as int ));
		let mut prev_index = 0 as freetype::FT_UInt;	// font char index
		let BIG		= 999999;
		let BORDER	= 1;
		let ALIGN	= 3;
		let mut max_x = -BIG;	// in font units
		let mut max_y = -BIG;
		let mut min_x = BIG;
		let mut min_y = BIG;
		let mut start_word = 0u;
		let width_capacity = limit_x as int << SHIFT;
		let mut pos_array = vec::with_capacity::<Target>( s.len() );
		for c in s.chars()	{
			if c == '\n'	{
				baseline += line_gap;
				position = 0;
				prev_index = 0 as freetype::FT_UInt;
			}else	{
				if std::char::is_whitespace(c)	{
					start_word = pos_array.len();
				}
				let index = unsafe{
					freetype::FT_Get_Char_Index( fh, c as freetype::FT_ULong )
				};
				unsafe{
					freetype::FT_Load_Glyph( fh, index, freetype::FT_LOAD_DEFAULT as freetype::FT_Int32 )
				}.check( "Load_Glyph" );
				position += unsafe{
					let delta = freetype::struct_FT_Vector_{ x:0, y:0 };
					freetype::FT_Get_Kerning( fh, prev_index, index,
						freetype::FT_KERNING_DEFAULT, ptr::to_unsafe_ptr(&delta) ).
						check( "Get_Kerning" );
					//lg.add(format!( "\tKerning {:i}-{:i} is {:i}",
					//	prev_index as int, index as int, delta.x as int ));
					delta.x as int + self.kern_offset
				};
				prev_index = index;
				let glyph = unsafe { &*(face.glyph as freetype::FT_GlyphSlot) };
				assert!( fh as uint == glyph.face as uint );
				let cx = position + glyph.metrics.horiBearingX as int;
				let cy = baseline - glyph.metrics.horiBearingY as int;
				pos_array.push( Target{ c:c, x:cx, y:cy });
				min_x = std::num::min( min_x, cx );
				min_y = std::num::min( min_y, cy );
				let mut ex = cx + glyph.metrics.width	as int;
				let mut ey = cy + glyph.metrics.height	as int;
				let e_border = (ex - min_x + (2*BORDER<<SHIFT)) | (((ALIGN+1)<<SHIFT)-1);
				if e_border >= width_capacity	{
					lg.add(format!( "\tMoving the word: {:u}-{:u}", start_word, pos_array.len() ));
					let word_offset = pos_array[start_word].x - min_x;
					if e_border - word_offset >= width_capacity	{
						fail!("Text exceeds horisontal bound: {:s}", s)
					}
					lg.add(format!( "\tHor:{:i} Ver:{:i}", -word_offset, line_gap ));
					prev_index = 0 as freetype::FT_UInt;
					for i in range( start_word, pos_array.len() )	{
						pos_array[i].x -= word_offset;
						pos_array[i].y += line_gap;
					}
					position -= word_offset;
					baseline += line_gap;
					ex -= word_offset;
					ey += line_gap;
				}
				max_x = std::num::max( max_x, ex );
				max_y = std::num::max( max_y, ey );
				//lg.add(format!( "\tsymbol:%c cx:{:i} cy:{:i}", c, cx, cy ));
				/*lg.add(format!( "\tSymbol '%c' (id={:u}) at ({:u},{:u}): size=({:i},{:i}) bearing=({:i},{:i})",
					c, self.get_char_index(c), position, baseline,
					glyph.metrics.width as int, glyph.metrics.height as int,
					glyph.metrics.horiBearingX as int, glyph.metrics.horiBearingY as int ));*/
				position += glyph.advance.x as int;
			}
		}
		// add border and align to 4 bytes
		let width = ((((SHIFT+max_x-min_x)>>SHIFT)+ALIGN+2*BORDER) & !ALIGN) as uint;
		let height= ((((SHIFT+max_y-min_y)>>SHIFT)+ALIGN+2*BORDER) & !ALIGN) as uint;
		min_x -= BORDER<<SHIFT; min_y -= BORDER<<SHIFT; 
		lg.add(format!( "\tBox at ({:i},{:i}) of size {:u}x{:u}", min_x, min_y, width, height ));
		assert!( width<=limit_x && height<=limit_y );
		let mut image = vec::from_elem( width*height, 0u8 );
		for slice in pos_array.iter()	{
			unsafe{
				freetype::FT_Load_Char( fh,
					slice.c as freetype::FT_ULong, freetype::FT_LOAD_DEFAULT as freetype::FT_Int32 )
			}.check( "Load_Char" );
			unsafe{
				freetype::FT_Render_Glyph(
					face.glyph as freetype::FT_GlyphSlot, freetype::FT_RENDER_MODE_NORMAL )
			}.check( "Render_Glyph" );
			let glyph = unsafe { &*(face.glyph as freetype::FT_GlyphSlot) };
			let bmp = &(glyph.bitmap);
			let bw = bmp.width as uint;
			let bh = bmp.rows as uint;
			assert!( bw == glyph.metrics.width	as uint >> SHIFT );
			assert!( bh == glyph.metrics.height	as uint >> SHIFT );
			let x = ((slice.x - min_x) >>SHIFT) as uint;
			let y = ((slice.y - min_y) >>SHIFT) as uint;
			//lg.add(format!( "\tx:{:u} bw:{:u}, width:{:u}, y:{:u} bh:{:u} height:{:u}", x,bw,width, y,bh,height ));
			assert!( x + bw <= width && y + bh <= height );
			self.draw( bmp, image, y*width + x, width );
		}
		let tex = gr.create_texture( "2D", width, height, 1u, 0u );
		gr.texture.load_2D( tex, 0u, texture::map_int_format("r8"),
			texture::map_pix_format("red"), image[0].to_gl_type(), image );
		tex
	}
}


impl Context	{
	pub fn load( &self, path: &str, index: uint, size: [uint,..2], kern: [int,..2],
			lg: &journal::Log )-> Font	{
		let mut face : freetype::FT_Face = ptr::null();
		lg.add(format!( "Loading font: {:s} with size {:u}x{:u}", path, size[0], size[1] ));
		path.with_c_str( |text|	{
			unsafe{
				freetype::FT_New_Face( self.lib, text, 
					index as freetype::FT_Long, ptr::to_mut_unsafe_ptr(&mut face) )
			}.check( "New_Face" );
		});
		unsafe{
			freetype::FT_Set_Pixel_Sizes( face,
				size[0] as freetype::FT_UInt, size[1] as freetype::FT_UInt )
		}.check( "Set_Pixel_Sizes" );
		Font{ face:FaceHandle(face),
			kern_offset	: (kern[0] << SHIFT) as int,
			line_offset	: (kern[1] << SHIFT) as int,
			//cache	:send_map::linear::HashMap::<char,Glyph>(),
		}
	}
}
