extern mod freetype;
use freetype::FTErrorMethods;


pub struct Context	{
	priv lib	: freetype::FT_Library,

	drop	{
		assert self.lib.is_not_null();
		let err = freetype::bindgen::FT_Done_FreeType( self.lib );
		assert err.succeeded();
	}
}

pub fn create_context()-> Context	{
	let mut lib : freetype::FT_Library = ptr::null();
	unsafe	{
		let err = freetype::bindgen::FT_Init_FreeType( ptr::addr_of(&lib) );
		assert err.succeeded();
	}
	Context{ lib:lib }
}


pub struct Font	{
	priv face	: freetype::FT_Face,

	drop	{
		let err = freetype::bindgen::FT_Done_Face( self.face );
		assert err.succeeded();
	}
}

impl Font	{
	pub fn set_char_size( xp : uint, yp : uint, xdpi : uint, ydpi : uint )	{
		let err = freetype::bindgen::FT_Set_Char_Size( self.face,
			16u*xp as freetype::FT_F26Dot6,
			16u*yp as freetype::FT_F26Dot6,
			xdpi as freetype::FT_UInt,
			ydpi as freetype::FT_UInt
			);
		assert err.succeeded();
	}
	pub fn set_pixel_size( xpix : uint, ypix : uint )	{
		let err = freetype::bindgen::FT_Set_Pixel_Sizes( self.face,
			xpix as freetype::FT_UInt,
			ypix as freetype::FT_UInt
			);
		assert err.succeeded();
	}
}


impl Context	{
	pub fn load_font( path : ~str, index : uint )-> Font	{
		let mut face : freetype::FT_Face = ptr::null();
		do str::as_c_str(path) |text|	{
			unsafe	{
				let err = freetype::bindgen::FT_New_Face( self.lib, text, 
					index as freetype::FT_Long, ptr::addr_of(&face) );
				assert err.succeeded();
			}
		}
		Font{ face:face }
	}
}