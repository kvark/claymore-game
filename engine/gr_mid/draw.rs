use core::hashmap::linear::LinearMap;
use core::to_bytes;

use gr_low::{context,shade};
use gr_mid::call;
use journal;
use load;
use space;
use space::Space;


pub trait Mod	{
	fn get_name( &self )-> ~str;
	fn get_code( &self )-> ~str;
	fn fill_data( &self, data : &mut shade::DataMap );
}

impl Mod for ()	{
	fn get_name( &self )->~str	{~"Dummy"}
	fn get_code( &self )->~str	{~"
vec3 modifyInit  (vec3 p) {return p;}
vec3 modifyVector(vec3 v) {return v;}"}
	fn fill_data( &self, _data : &mut shade::DataMap )	{}
}


pub struct Material	{
	name			: ~str,
	meta_vertex		: ~[~str],
	meta_fragment	: ~[~str],
	code_vertex		: ~str,
	code_fragment	: ~str,
}


pub struct Entity	{
	node	: @mut space::Node,
	//body	: @node::Body,
	input	: call::DrawInput,
	data	: shade::DataMap,
	modifier: @Mod,
	material: @Material,
}

pub impl Entity	{
	fn update_world( &mut self )	{
		let world = self.node.world_space().to_matrix();
		self.data.insert( ~"u_World", shade::UniMatrix(false,world) );
	}
}


struct CacheEntry	{
	material	: @Material,
	modifier	: @Mod,
	technique	: ~[~str],
}

impl cmp::Eq for CacheEntry	{
	fn eq( &self, other : &CacheEntry )-> bool	{
		self.material.code_vertex == other.material.code_vertex &&
		self.material.code_fragment == other.material.code_fragment &&
		self.modifier.get_code() == other.modifier.get_code();
		self.technique == other.technique
	}
	fn ne( &self, other : &CacheEntry )-> bool	{
		!self.ne(other)
	}
}

impl to_bytes::IterBytes for CacheEntry	{
	fn iter_bytes( &self, lsb0 : bool, f : to_bytes::Cb )	{
		self.material.name.iter_bytes( lsb0, f );
		self.modifier.get_name().iter_bytes( lsb0, f );
		self.technique.iter_bytes( lsb0, f );
	}
}


pub type Cache = LinearMap< CacheEntry, Option<@shade::Program> >;
pub fn make_cache()-> Cache	{
	LinearMap::new()
}


pub struct Technique	{
	name	: ~str,
	meta_vertex		: ~[~str],
	meta_fragment	: ~[~str],
	code_vertex		: ~str,
	code_fragment	: ~str,
	priv cache		: @mut Cache,
}


pub impl Technique	{
	fn get_header( &self )-> ~str	{~"#version 150 core"}
	
	fn make_vertex( &self, material : @Material, modifier : @Mod )-> ~str	{
		str::connect([
			self.get_header(),
			fmt!( "//--- Modifier: %s ---//", modifier.get_name() ),
			modifier.get_code(),
			fmt!( "//--- Material: %s ---//", material.name ),
			copy material.code_vertex,
			fmt!( "//--- Technique: %s ---//", self.name ),
			copy self.code_vertex
		], "\n")
	}
	
	fn make_fragment( &self, mat : @Material )-> ~str	{
		str::connect([ self.get_header(),
			fmt!("//--- Material: %s ---//",mat.name),
			copy mat.code_fragment,
			fmt!("//--- Technique: %s ---//",self.name),
			copy self.code_fragment,
		], "\n")
	}
	
	fn link( &self, e : &Entity, ct : &context::Context, lg : &journal::Log )-> Option<@shade::Program>	{
		if !vec::all(self.meta_vertex,	|m|	{ e.material.meta_vertex.contains(m) 	})
		|| !vec::all(self.meta_fragment,|m|	{ e.material.meta_fragment.contains(m)	})	{
			lg.add(fmt!( "Material '%s' rejected by '%s'", e.material.name, self.name ));
			return None;
		}
		lg.add(fmt!( "Linking material '%s' with technique '%s'", e.material.name, self.name ));
		let s_vert = self.make_vertex( e.material, e.modifier );
		let s_frag = self.make_fragment( e.material );
		let shaders = if false	{
			lg.add(~"Compiling vert");
			lg.add(copy s_vert);
			let sv = ct.create_shader('v',s_vert);
			lg.add(~"Compiling frag");
			lg.add(copy s_frag);
			let sf = ct.create_shader('f',s_frag);
			lg.add(~"Linking");
			~[sv,sf]
		}else	{
			~[ ct.create_shader('v',s_vert), ct.create_shader('f',s_frag) ]
		};
		Some( ct.create_program(shaders,lg) )
	}

	fn get_program( &self, e : &Entity, ct : &context::Context, lg : &journal::Log )-> Option<@shade::Program>	{
		let ce = CacheEntry{ material:e.material, modifier:e.modifier,
			technique:~[copy self.code_vertex,copy self.code_fragment]
		};
		match self.cache.find(&ce)	{
			Some(p)	=> *p,
			None =>	{
				let p = self.link( e, ct, lg );
				self.cache.insert( ce, p );
				p
			}
		}
	}

	fn process( &self, e : &Entity, output	: call::DrawOutput, ct : &context::Context, lg : &journal::Log )-> call::Call	{
		match self.get_program(e,ct,lg)	{
			Some(p)	=> call::CallDraw( copy e.input, output, p, copy e.data ),
			None => call::CallEmpty
		}
	}
}


pub fn extract_metas( code : &str )-> ~[~str]	{
	let meta_start	= str::find_str(code,"//%meta")
		.expect(~"Unable to find meta start marker");
	let meta_size	= str::find_str_from(code,"\n",meta_start)
		.expect(~"Unable to find meta end marker");	
	vec::build(|push|	{
		let mut start = true;
		do str::each_word( code.slice( meta_start, meta_size )) |word|	{
			if start	{start=false;}
			else	{ push( word.to_owned() ); }
			true
		}
	})
}

pub fn load_material( path : ~str )-> Material	{
	let s_vert = load::load_text( path + ".glslv" );
	let s_frag = load::load_text( path + ".glslf" );
	Material{ name:path,
		meta_vertex		:extract_metas(s_vert),
		meta_fragment	:extract_metas(s_frag),
		code_vertex		:s_vert,
		code_fragment	:s_frag,
	}
}

pub fn load_technique( path : ~str )-> Technique	{
	let s_vert = load::load_text( path + ".glslv" );
	let s_frag = load::load_text( path + ".glslf" );
	Technique{ name:path,
		meta_vertex		:extract_metas(s_vert),
		meta_fragment	:extract_metas(s_frag),
		code_vertex		:s_vert,
		code_fragment	:s_frag,
		cache : @mut make_cache(),
	}
}
