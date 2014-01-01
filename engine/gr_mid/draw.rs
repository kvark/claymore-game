use std::hash::Hash;
use std::hashmap::HashMap;
use std::to_bytes;

use gr_low::{context,shade};
use journal;
use load;


pub trait Mod	{
	fn get_name<'a>( &'a self )-> &'a str;
	fn get_code<'a>( &'a self )-> &'a str;
	fn fill_data( &self, data : &mut shade::DataMap );
}

static empty_name : &'static str = &"Dummy";
static empty_code : &'static str = &"
vec3 modifyInit  (vec3 p) {return p;}
vec3 modifyVector(vec3 v) {return v;}";

impl Mod for ()	{
	fn get_name<'a>( &'a self )-> &'a str	{empty_name}
	fn get_code<'a>( &'a self )-> &'a str	{empty_code}
	fn fill_data( &self, _data : &mut shade::DataMap )	{}
}


pub struct Material	{
	name			: ~str,
	meta_vertex		: ~[~str],
	meta_fragment	: ~[~str],
	code_vertex		: ~str,
	code_fragment	: ~str,
}


struct CacheEntry	{
	material	: @Material,
	modifier	: @Mod,
	technique	: ~[~str],	//TODO: borrow
}

impl to_bytes::IterBytes for CacheEntry	{
	fn iter_bytes( &self, lsb0 : bool, f : to_bytes::Cb )-> bool	{
		self.material.name.iter_bytes( lsb0, |x| f(x) ) &&
		self.modifier.get_name().iter_bytes( lsb0, |x| f(x) ) &&
		self.technique.iter_bytes( lsb0, f )
	}
}


pub type Cache = HashMap< u64, Option<@shade::Program> >;
pub fn make_cache()-> Cache	{
	HashMap::new()
}


pub struct Technique	{
	name	: ~str,
	meta_vertex		: ~[~str],
	meta_fragment	: ~[~str],
	code_vertex		: ~str,
	code_fragment	: ~str,
}

static glsl_header_150 : &'static str = &"#version 150 core";

impl Technique	{
	pub fn get_header<'a>( &'a self )-> &'a str	{glsl_header_150}
	
	pub fn make_vertex( &self, material : &Material, modifier : @Mod )-> ~str	{
		let smod = format!( "//--- Modifier: {:s} ---//", modifier.get_name() );
		let smat = format!( "//--- Material: {:s} ---//", material.name );
		let stek = format!( "//--- Technique: {:s} ---//", self.name );
		[	self.get_header(),
			smod				.as_slice(),
			modifier.get_code(),
			smat				.as_slice(),
			material.code_vertex.as_slice(),
			stek				.as_slice(),
			self.code_vertex	.as_slice()
		].connect("\n")
	}
	
	pub fn make_fragment( &self, mat : &Material )-> ~str	{
		let smat = format!( "//--- Material: {:s} ---//", mat.name );
		let stek = format!( "//--- Technique: {:s} ---//", self.name );
		[	self.get_header(),
			smat				.as_slice(),
			mat.code_fragment	.as_slice(),
			stek				.as_slice(),
			self.code_fragment	.as_slice(),
		].connect("\n")
	}
	
	pub fn link( &self, mat : &Material, modifier : @Mod, ct : &context::Context, lg : &journal::Log )-> Option<@shade::Program>	{
		if !self.meta_vertex.iter().all(|m|	{ mat.meta_vertex.contains(m) 	})
		|| !self.meta_fragment.iter().all(|m|	{ mat.meta_fragment.contains(m)	})	{
			lg.add(format!( "Material '{:s}' rejected by '{:s}'", mat.name, self.name ));
			return None;
		}
		lg.add(format!( "Linking material '{:s}' with technique '{:s}'", mat.name, self.name ));
		let s_vert = self.make_vertex( mat, modifier );
		let s_frag = self.make_fragment( mat );
		let shaders = if false	{
			lg.add("Compiling vert");
			lg.add( s_vert.clone() );
			let sv = ct.create_shader('v',s_vert);
			lg.add("Compiling frag");
			lg.add( s_frag.clone() );
			let sf = ct.create_shader('f',s_frag);
			lg.add("Linking");
			~[sv,sf]
		}else	{
			~[ ct.create_shader('v',s_vert), ct.create_shader('f',s_frag) ]
		};
		Some( ct.create_program(shaders,lg) )
	}

	pub fn get_program( &self, mat : @Material, modifier : @Mod, cache : &mut Cache, ct : &context::Context, lg : &journal::Log )-> Option<@shade::Program>	{
		let hash = CacheEntry{ material:mat, modifier:modifier,
			technique:~[self.code_vertex.clone(), self.code_fragment.clone()]	//FIXME
		}.hash();
		match cache.find(&hash)	{
			Some(p)	=> return *p,
			_	=> ()
		}
		let p = self.link( mat, modifier, ct, lg );
		cache.insert( hash, p );
		p
	}
}


pub fn extract_metas( code : &str )-> ~[~str]	{
	let meta_start	= code.find_str("//%meta")
		.expect("Unable to find meta start marker");
	let meta_size	= code.slice_from(meta_start).find_str("\n")
		.expect("Unable to find meta end marker");	
	let slice = code.slice(meta_start,meta_start+meta_size);
	slice.words().skip(1).map(|w| w.to_owned()).collect()
}

pub fn load_material( path : &str )-> Material	{
	let s_vert = load::load_text( path + ".glslv" );
	let s_frag = load::load_text( path + ".glslf" );
	Material{ name		: path.to_owned(),
		meta_vertex		: extract_metas(s_vert),
		meta_fragment	: extract_metas(s_frag),
		code_vertex		: s_vert,
		code_fragment	: s_frag,
	}
}

pub fn load_technique( path : &str )-> Technique	{
	let s_vert = load::load_text( path + ".glslv" );
	let s_frag = load::load_text( path + ".glslf" );
	Technique{ name		: path.to_owned(),
		meta_vertex		: extract_metas(s_vert),
		meta_fragment	: extract_metas(s_frag),
		code_vertex		: s_vert,
		code_fragment	: s_frag,
	}
}
