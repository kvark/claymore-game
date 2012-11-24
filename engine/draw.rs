pub trait Mod	{
	pure fn get_name()-> ~str;
	pure fn get_code()-> ~str;
	fn fill_data( data : &mut shade::DataMap );
}

impl () : Mod	{
	pure fn get_name()->~str	{~"Dummy"}
	pure fn get_code()->~str	{~"
		vec3 modifyInit  (vec3 p) {return p}
		vec3 modifyVector(vec3 v) { return v}"}
	fn fill_data( _data : &mut shade::DataMap )	{}
}

pub struct Material	{
	name			: ~str,
	meta_vertex		: ~[~str],
	meta_fragment	: ~[~str],
	code_vertex		: ~str,
	code_fragment	: ~str,
}


pub struct Entity	{
	node	: @space::Node,
	//body	: @node::Body,
	mut data: shade::DataMap,
	vao		: @buf::VertexArray,
	mesh	: @mesh::Mesh,
	range	: mesh::Range,
	modifier: @Mod,
	material: @Material,
}

//FIXME: remove this
impl Entity	{
	fn set_data( name : ~str, val : shade::Uniform )	{
		self.data.insert( name, val );
	}
}

struct CacheEntry	{
	material	: @Material,
	modifier	: @Mod,
	technique	: ~[~str],
}

impl CacheEntry : cmp::Eq	{
	pure fn eq( other : &CacheEntry )-> bool	{
		self.material.code_vertex == other.material.code_vertex &&
		self.material.code_fragment == other.material.code_fragment &&
		self.modifier.get_code() == other.modifier.get_code();
		self.technique == other.technique
	}
	pure fn ne( other : &CacheEntry )-> bool	{
		!self.eq( other )
	}
}

impl CacheEntry : to_bytes::IterBytes	{
	pure fn iter_bytes(lsb0 : bool, f : to_bytes::Cb)	{
		self.material.name.iter_bytes( lsb0, f );
		self.modifier.get_name().iter_bytes( lsb0, f );
		self.technique.iter_bytes( lsb0, f );
	}
}

pub type Cache = send_map::linear::LinearMap< CacheEntry, Option<@shade::Program> >;
pub fn create_cache()-> Cache	{
	send_map::linear::LinearMap::< CacheEntry, Option<@shade::Program> >()
}


pub struct Technique	{
	name	: ~str,
	fbo		: @frame::Buffer,
	pmap	: call::PlaneMap,
	rast	: rast::State,
	meta_vertex		: ~[~str],
	meta_fragment	: ~[~str],
	code_vertex		: ~str,
	code_fragment	: ~str,
	priv cache	: @mut Cache,
}


impl Technique	{
	pure fn get_header()-> ~str	{~"#version 150 core"}
	
	fn make_vertex( material : @Material, modifier : @Mod )-> ~str	{
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
	
	fn make_fragment( mat : @Material )-> ~str	{
		str::connect([ self.get_header(),
			fmt!("//--- Material: %s ---//",mat.name),
			copy mat.code_fragment,
			fmt!("//--- Technique: %s ---//",self.name),
			copy self.code_fragment,
		], "\n")
	}
	
	fn link( e : &Entity, ct : &context::Context )-> Option<@shade::Program>	{
		if !do vec::all(self.meta_vertex)	|m|	{
			e.material.meta_vertex.contains(m)
		}||!do vec::all(self.meta_fragment)	|m|	{
			e.material.meta_fragment.contains(m)
		}{
			io::println(fmt!( "Material '%s' rejected by '%s'", e.material.name, self.name ));
			return None;
		}
		let s_vert = self.make_vertex( e.material, e.modifier );
		let s_frag = self.make_fragment( e.material );
		let shaders = if false	{
			io::println("Compiling vert");
			io::println(s_vert);
			let sv = ct.create_shader('v',s_vert);
			io::println("Compiling frag");
			io::println(s_frag);
			let sf = ct.create_shader('f',s_frag);
			io::println("Linking");
			~[sv,sf]
		}else	{
			~[ ct.create_shader('v',s_vert), ct.create_shader('f',s_frag) ]
		};
		Some( @ct.create_program(shaders) )
	}

	fn get_program( e : &Entity, ct : &context::Context )-> Option<@shade::Program>	{
		let ce = CacheEntry{ material:e.material, modifier:e.modifier,
			technique:~[copy self.code_vertex,copy self.code_fragment]
		};
		match self.cache.find(&ce)	{
			Some(p)	=> p,
			None =>	{
				let p = self.link( e, ct );
				self.cache.insert( ce, p );
				p
			}
		}
	}

	fn process( e : &Entity, ct : &context::Context )-> call::Call	{
		//let mut data = shade::create_data();
		match self.get_program(e,ct)	{
			Some(p)	=> call::CallDraw( self.fbo, copy self.pmap,
				e.vao, e.mesh, e.range, p, copy e.data, self.rast ),
			None => call::CallEmpty
		}
	}
}


pub pure fn extract_metas( code : &str )->~[~str]	{
	let meta_start	= str::find_str(code,"//%meta")				.expect(~"Unable to find meta start marker");
	let meta_size	= str::find_str_from(code,"\n",meta_start)	.expect(~"Unable to find meta end marker");
	let split = code.slice( meta_start, meta_size ).split_char(' ');
	//split.tail()	//FIXME
	vec::from_fn(split.len()-1, |i| copy split[i])
}

pub fn load_material( path : ~str )-> Material	{
	let s_vert = load::read_text(path+".glslv");
	let s_frag = load::read_text(path+".glslf");
	Material{ name:path,
		meta_vertex		:extract_metas(s_vert),
		meta_fragment	:extract_metas(s_frag),
		code_vertex		:s_vert,
		code_fragment	:s_frag,
	}
}

pub fn load_technique( path : ~str, fbo : @frame::Buffer, pmap : &call::PlaneMap,
		rast : &rast::State, cache : @mut Cache )-> Technique	{
	let s_vert = load::read_text(path+".glslv");
	let s_frag = load::read_text(path+".glslf");
	Technique{ name:path,
		fbo:fbo, pmap:*pmap, rast:*rast,
		meta_vertex		:extract_metas(s_vert),
		meta_fragment	:extract_metas(s_frag),
		code_vertex		:s_vert,
		code_fragment	:s_frag,
		cache:cache,
	}
}
