pub trait Mod	{
	pure fn get_name()-> ~str;
	pure fn get_code()-> ~str;
}


pub struct Material	{
	name			: ~str,
	metas			: ~[~str],
	code_vertex		: ~str,
	code_fragment	: ~str,
}


pub struct Entity	{
	node	: @space::Node,
	//body	: @node::Body,
	vao		: @buf::VertexArray,
	mesh	: @mesh::Mesh,
	range	: mesh::Range,
	mods	: ~[@Mod],
	material: @Material,
}


struct CacheEntry	{
	mat		: @Material,
	mods	: ~[@Mod],
	tech	: ~[~str],
}

impl CacheEntry : cmp::Eq	{
	pure fn eq( other : &CacheEntry )-> bool	{
		self.mat.code_vertex == other.mat.code_vertex &&
		self.mat.code_fragment == other.mat.code_fragment &&
		do vec::all2(self.mods,other.mods)  |m1,m2| { m1.get_code()==m2.get_code() } &&
		self.tech == other.tech
	}
	pure fn ne( other : &CacheEntry )-> bool	{
		!self.eq( other )
	}
}

impl CacheEntry : to_bytes::IterBytes	{
	pure fn iter_bytes(lsb0 : bool, f : to_bytes::Cb)	{
		self.mat.name.iter_bytes( lsb0, f );
		for self.mods.each() |m|	{
			m.get_name().iter_bytes( lsb0, f );
		}
		self.tech.iter_bytes( lsb0, f );
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
	code_vertex		: ~str,
	code_fragment	: ~str,
	used_metas		: ~[~str],
	priv cache	: @mut Cache,
}


impl Technique	{
	fn make_vertex( mat : @Material, mods : &[@Mod] )-> ~str	{
		let S_MOD = ~"modify";
		let mut buf : ~[~str] = ~[];
		buf.push(fmt!( "//--- Material: %s ---//", mat.name ));
		buf.push( copy mat.code_vertex );
		// add modifier bases
		for mods.each() |m|	{
			let target = fmt!( "%s%s", S_MOD, m.get_name() );
			buf.push(fmt!( "//--- Modifier: %s} ---//", m.get_name() ));
			buf.push( str::replace( m.get_code(), S_MOD, target ) );
		}
		// add technique
		buf.push(fmt!( "//--- Technique: %s ---//", self.name ));
		let mod_start = match str::find_str( self.code_vertex, ~"//%"+S_MOD )	{
			Some(p)	=> p,
			None	=> fail(~"Unable to find modifier start marker")
		};
		buf.push( self.code_vertex.substr(0,mod_start) );
		let mod_end = match str::find_str_from( self.code_vertex, "\n", mod_start )	{
			Some(p)	=> p,
			None	=> fail(~"Unable to find modifier end marker")
		};
		// extract position and vector names
		let split = self.code_vertex.substr(mod_start,mod_end-mod_start).split_char(' ');
		// add modifier calls
		for mods.each() |m|	{
			for split.eachi() |i,s|	{
				let t = if i>1 {~"Vector"} else {~"Position"};
				if i>0	{
					buf.push(fmt!( "\t%s = %s%s%s(%s);", *s,S_MOD,m.get_name(),t,*s ));
				}
			}
		}
		// finish
		buf.push( self.code_vertex.substr( mod_end, self.code_vertex.len()-mod_end ) );
		str::connect( buf, "\n" )
	}
	
	fn make_fragment( mat : @Material )-> ~str	{
		str::connect([
			fmt!("//--- Material: %s ---//",mat.name),
			copy mat.code_fragment,
			fmt!("//--- Technique: %s ---//",self.name),
			copy self.code_fragment,
		], "\n")
	}
	
	fn link( e : &Entity, ct : &context::Context )-> Option<@shade::Program>	{
		if !do vec::all(self.used_metas)	|m|	{
			e.material.metas.contains(m)
		}{ return None; }
		let s_vert = self.make_vertex( e.material, e.mods );
		let s_frag = self.make_fragment( e.material );
		//io::println(s_vert); io::println(s_frag);
		let shaders = ~[ ct.create_shader('v',s_vert), ct.create_shader('f',s_frag) ];
		Some( @ct.create_program(shaders) )
	}

	fn get_program( e : &Entity, ct : &context::Context )-> Option<@shade::Program>	{
		let ce = CacheEntry{ mat:e.material, mods:e.mods,
			tech:~[copy self.code_vertex,copy self.code_fragment]
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

	fn process( e : &Entity, ct : &context::Context, data : shade::DataMap )-> call::Call	{
		//let mut data = shade::create_data();
		match self.get_program(e,ct)	{
			Some(p)	=> call::CallDraw( self.fbo, copy self.pmap,
				e.vao, e.mesh, e.range, p, data, self.rast ),
			None => call::CallEmpty
		}
	}
}


pub pure fn extract_metas( code : &str )->~[~str]	{
	let meta_start	= match str::find_str(code,"//%meta")	{
			Some(p)	=> p,
			None	=> fail(~"Unable to find meta start marker")
	};
	let meta_size		= match str::find_str_from(code,"\n",meta_start)	{
			Some(p)	=> p - meta_start,
			None	=> fail(~"Unable to find meta end marker")
	};
	let split = code.substr( meta_start, meta_size ).split_char(' ');
	//split.tail()	//FIXME
	vec::from_fn(split.len()-1, |i| copy split[i])
}

pub fn load_material( path : ~str )-> Material	{
	let s_vert = load::read_text(path+".glslv");
	let s_frag = load::read_text(path+".glslf");
	Material{ name:path, metas:extract_metas(s_frag),
		code_vertex:s_vert, code_fragment:s_frag,
	}
}

pub fn load_technique( path : ~str, fbo : @frame::Buffer, pmap : &call::PlaneMap,
		rast : &rast::State, cache : @mut Cache )-> Technique	{
	let s_vert = load::read_text(path+".glslv");
	let s_frag = load::read_text(path+".glslf");
	Technique{
		name:path, fbo:fbo, pmap:*pmap, rast:*rast,
		code_vertex:s_vert, code_fragment:s_frag,
		used_metas:extract_metas(s_frag),
		cache:cache,
	}
}
