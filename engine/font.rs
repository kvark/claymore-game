extern mod freetype;
use context::GLType;


trait FontError	{
	fn check( s : &str );
}
impl freetype::FT_Error : FontError	{
	fn check( s : &str )	{
		if (self as int)!=0	{
			fail(fmt!( "Freetype %s failed with code %d", s, self as int ));
		}
	}
}


pub struct Context	{
	priv lib	: freetype::FT_Library,

	drop	{
		assert self.lib.is_not_null();
		freetype::bindgen::FT_Done_FreeType( self.lib )
			.check( "Done_FreeType" );
	}
}

pub fn create_context()-> Context	{
	let mut lib : freetype::FT_Library = ptr::null();
	unsafe	{
		freetype::bindgen::FT_Init_FreeType( ptr::addr_of(&lib) )
			.check( "Init_FreeType" );
		
	}
	Context{ lib:lib }
}


pub struct Font	{
	priv face	: freetype::FT_Face,

	drop	{
		freetype::bindgen::FT_Done_Face( self.face )
			.check( "Done_Face" );
	}
}

impl Font	{
	priv fn set_char_size( xs : float, ys : float, xdpi : uint, ydpi : uint )	{
		freetype::bindgen::FT_Set_Char_Size( self.face,
			64f*xs as freetype::FT_F26Dot6,
			64f*ys as freetype::FT_F26Dot6,
			xdpi as freetype::FT_UInt,
			ydpi as freetype::FT_UInt
			).check( "Set_Char_Size" );
	}
	priv fn set_pixel_size( xpix : uint, ypix : uint )	{
		freetype::bindgen::FT_Set_Pixel_Sizes( self.face,
			xpix as freetype::FT_UInt,
			ypix as freetype::FT_UInt
			).check( "Set_Pixel_Sizes" );
	}
	priv fn get_char_index( c : char )-> uint	{
		freetype::bindgen::FT_Get_Char_Index( self.face, c as freetype::FT_ULong ) as uint
	}
	priv fn load_char( c : char )	{
		freetype::bindgen::FT_Load_Glyph( self.face,
			c as freetype::FT_UInt,
			freetype::FT_LOAD_RENDER as freetype::FT_Int32 )
			.check( "Load_Glyph" );
	}
	priv fn draw( bm : &freetype::FT_Bitmap, target : &mut ~[u8], offset : uint, pitch : uint )	{
		//TODO: use slice copy
		for uint::range(0,bm.rows as uint) |y|	{
			for uint::range(0,bm.width as uint) |x|	{
				target[y*pitch + offset + x] = unsafe	{
					*ptr::offset(bm.buffer, y*(bm.pitch as uint) + x) as u8
					};
			}
		}
	}
	pub fn bake( gr : &context::Context, s : ~str, mx : uint, my : uint )-> texture::Texture	{
		io::println(fmt!( "Font baking text: %s", s ));
		let mut cx = 0u, cy = 0u;
		let mut image = vec::from_elem( mx*my, 0u8 );
		for s.each_char() |c|	{
			freetype::bindgen::FT_Load_Char( self.face,
				c as freetype::FT_ULong,
				freetype::FT_LOAD_DEFAULT as freetype::FT_Int32 )	//RENDER doesn't work!
				.check( "Load_Char" );
			let face  = unsafe { &*(self.face) };
			freetype::bindgen::FT_Render_Glyph(
				face.glyph as freetype::FT_GlyphSlot,
				freetype::FT_RENDER_MODE_NORMAL )
				.check( "Render_Glyph" );
			let glyph = unsafe { &*(face.glyph as freetype::FT_GlyphSlot) };
			assert self.face as uint == glyph.face as uint;
			let bmp = &(glyph.bitmap);
			self.draw( bmp, &mut image, cx, mx );
			io::println(fmt!( "\tSymbol '%c' (id=%u) on (%u,%u): width=%d height=%d",
				c, self.get_char_index(c), cx, cy,
				bmp.width as int, bmp.rows as int ));
				//glyph.metrics.width as int, glyph.metrics.height as int ));
			cx += (glyph.advance.x as uint)>>6u;
			assert cx < mx;
		}
		let tex = gr.create_texture( ~"Rect", mx, my, 1u, 0u );
		gr.texture.load_2D( &tex, 0u, texture::map_int_format(~"r8"),
			texture::map_pix_format(~"red"), image[0].to_gl_type(), &image );
		gr.texture.wrap( &tex, 0 );
		gr.texture.filter( &tex, 1u );
		tex
	}
}


impl Context	{
	pub fn load_font( path : ~str, index : uint, xs : uint, ys : uint )-> Font	{
		let mut face : freetype::FT_Face = ptr::null();
		do str::as_c_str(path) |text|	{
			unsafe	{
				freetype::bindgen::FT_New_Face( self.lib, text, 
					index as freetype::FT_Long, ptr::addr_of(&face) )
			}.check( "New_Face" );
		}
		freetype::bindgen::FT_Set_Pixel_Sizes( face,
			xs as freetype::FT_UInt, ys as freetype::FT_UInt )
			.check( "Set_Pixel_Sizes" );
		Font{ face:face }
	}
}