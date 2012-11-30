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


/*//TODO: enable when supported
struct Glyph	{
	slot	: freetype::FT_GlyphSlot,
	drop	{
		freetype::bindgen::FT_Done_Glyph( self.slot );
	}
}*/


pub struct Font	{
	priv face		: freetype::FT_Face,
	//priv mut cache	: send_map::linear::LinearMap<char,Glyph>,

	drop	{
		freetype::bindgen::FT_Done_Face( self.face )
			.check( "Done_Face" );
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
			Some(ref g) => &g,
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
				freetype::bindgen::FT_Get_Glyph( self.face,
					unsafe{ ptr::addr_of(&slot) })
					.check( "Get_Glyph" );
				let g = Glyph{ slot:slot };
				self.cache.insert( c, slot );
				g
			}
		}
	}*/

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

	priv fn draw( bm : &freetype::FT_Bitmap, target : &mut ~[u8], offset : uint, pitch : uint )	{
		//TODO: use &[mut u8] argument
		let height = bm.rows as uint;
		for uint::range(0,height) |y|	{
			unsafe	{
				let src = ptr::offset( bm.buffer, y*(bm.pitch as uint)) as *u8;
				let dst = ptr::offset( vec::raw::to_ptr(*target), y*pitch+offset ) as *mut u8;
				ptr::memcpy( dst, src, bm.width as uint );
			};
		}
	}

	pub fn bake( gr : &context::Context, s : ~str, limit_x : uint, limit_y : uint )-> texture::Texture	{
		io::println(fmt!( "Font baking text: %s", s ));
		type Target = (char,uint,uint);
		let mut pos_array = vec::with_capacity::<Target>( s.len() );
		let face  = unsafe { &*(self.face) };
		let mut position = 0u;
		let mut baseline = face.ascender as uint;
		io::println(fmt!( "\tFace up %d down %d", face.ascender as int, face.descender as int ));
		let mut prev_index = 0 as freetype::FT_UInt;
		let mut max_x = 0u, max_y = 0u;
		for s.each_char() |c|	{
			if c == '\n'	{
				baseline += face.height as uint;
				position = 0u;
				prev_index = 0 as freetype::FT_UInt;
			}else	{
				let index = freetype::bindgen::FT_Get_Char_Index( self.face, c as freetype::FT_ULong );
				freetype::bindgen::FT_Load_Glyph( self.face, index,
					freetype::FT_LOAD_DEFAULT as freetype::FT_Int32 )
					.check( "Load_Glyph" );
				position += unsafe	{
					let zero = 0 as freetype::FT_Pos;
					let delta = { x:zero, y:zero };
					freetype::bindgen::FT_Get_Kerning( self.face, prev_index, index,
						freetype::FT_KERNING_DEFAULT, ptr::addr_of(&delta) )
						.check( "Get_Kerning" );
					//io::println(fmt!( "\tKerning %d-%d is %d",
					//	prev_index as int, index as int, delta.x as int ));
					delta.x as uint
				};
				prev_index = index;
				let glyph = unsafe { &*(face.glyph as freetype::FT_GlyphSlot) };
				assert self.face as uint == glyph.face as uint;
				let cx = position >> 6u;
				let cy = (baseline - glyph.metrics.horiBearingY as uint) >> 6u;
				let ex = position + (glyph.metrics.width + glyph.metrics.horiBearingX) as uint;
				let ey = baseline + (glyph.metrics.height- glyph.metrics.horiBearingY) as uint;
				/*if ex>limit_x	{
					let i = pos_array.len();
					baseline += face.height as uint;
					position = 0u;
					prev_index = 0 as freetype::FT_UInt;
					while i>0	{
						let &(c,_,y) = &pos_array[i];
						i = if c.is_whitespace() || y!=cy	{0u}
						else	{
							y = baseline>>6u;
							i-1u
						}
					}
				}*/
				max_x = uint::max( max_x, ((ex-1u)>>6u) + 1u );
				max_y = uint::max( max_y, ((ey-1u)>>6u) + 1u );
				/*io::println(fmt!( "\tSymbol '%c' (id=%u) at (%u,%u): size=(%d,%d) bearing=(%d,%d)",
					c, self.get_char_index(c), position, baseline,
					glyph.metrics.width as int, glyph.metrics.height as int,
					glyph.metrics.horiBearingX as int, glyph.metrics.horiBearingY as int ));*/
				pos_array.push( (c,cx,cy) );
				position += glyph.advance.x as uint;
			}
		}
		assert max_x<=limit_x && max_y<=limit_y;
		io::println(fmt!( "Sufficient dimensions (%u,%u)", max_x, max_y ));
		let mut image = vec::from_elem( max_x*max_y, 0u8 );
		for pos_array.each |slice|	{
			let &(c,x,y) = slice;
			freetype::bindgen::FT_Load_Char( self.face, c as freetype::FT_ULong,
				freetype::FT_LOAD_DEFAULT as freetype::FT_Int32 )
				.check( "Load_Char" );
			freetype::bindgen::FT_Render_Glyph(
				face.glyph as freetype::FT_GlyphSlot,
				freetype::FT_RENDER_MODE_NORMAL )
				.check( "Render_Glyph" );
			let glyph = unsafe { &*(face.glyph as freetype::FT_GlyphSlot) };
			let bmp = &(glyph.bitmap);
			assert x + (bmp.width	as uint) <= max_x;
			assert y + (bmp.rows	as uint) <= max_y;
			self.draw( bmp, &mut image, y*max_x + x, max_x );
		}
		let tex = gr.create_texture( ~"2D", max_x, max_y, 1u, 0u );
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
		Font{ face	:face,
			//cache	:send_map::linear::LinearMap::<char,Glyph>(),
		}
	}
}