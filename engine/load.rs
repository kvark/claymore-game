extern mod gl;	//TODO: remove this
extern mod cgmath;
extern mod stb_image;

use std;
use std::{io,str};
use extra::future::Future;

use cgmath::angle;
use cgmath::rotation::Euler;
use cgmath::quaternion::{Quat,ToQuat};
use cgmath::vector::Vec3;

use gr_low::{buf,context,shade,texture};
use gr_mid::mesh;
use anim;
use journal;
use space;
use space::Space;

//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Chunk reader									//

#[deriving(Clone)]
pub struct Chunk	{
	name	: ~str,
	size	: uint,
	finish	: uint,
}


pub struct Reader	{
	path			: ~str,
	priv bin		: @io::Reader,
	priv walk_fun	: ~'static fn (&Reader)->Chunk,
	priv chunks		: ~[Chunk],
}

pub fn enter_dummy( _rd : &Reader )-> Chunk	{
	fail!( ~"unexpected chunk read" )
}

pub fn enter_chunk( rd : &Reader )-> Chunk	{
	let mut name_bin = rd.get_bytes(8u);
	name_bin.retain( |&b| {b != 0u8} );
	let size = rd.get_uint(4u);
	Chunk	{
		name	: str::from_utf8( name_bin ),
		size	: size,
		finish	: rd.position() + size,
	}
}


impl Reader	{
	pub fn create_ext( path : &str, fun : ~'static fn(&Reader)->Chunk )-> Reader	{
		let p = std::path::Path( path );
		match io::file_reader(&p)	{
			Ok(bin)		=> Reader{
				path	: path.to_owned(),
				bin		: bin,
				walk_fun: fun,
				chunks	: ~[]
			},
			Err(msg)	=> fail!("Unable to read %s: %s", path, msg)
		}
	}

	pub fn create_std( path : &str )-> Reader	{
		Reader::create_ext( path, enter_chunk )
	}

	pub fn create( path : &str )-> Reader	{
		Reader::create_ext( path, enter_dummy )
	}

	pub fn get_bytes( &self, num : uint )-> ~[u8]	{
		self.bin.read_bytes(num)
	}
	pub fn get_uint( &self, size : uint )-> uint	{
		let bytes = self.get_bytes(size);
		bytes.rev_iter().fold( 0u, |u,&t|	{(u<<8u) + t as uint} )
	}

	pub fn get_bool( &self )-> bool	{
		self.get_uint(1u) != 0u
	}

	pub fn get_string( &self )-> ~str	{
		let size = self.bin.read_byte() as uint;
		str::from_utf8( self.bin.read_bytes(size) )
	}

	pub fn get_floats( &self, num : uint )-> ~[f32]	{
		let data = self.bin.read_bytes( num * 4u );
		let mut vals : ~[f32];
		unsafe	{
			vals = std::vec::raw::from_buf_raw( std::vec::raw::to_ptr(data) as *f32, num );
		}
		vals
	}
	pub fn get_float( &self )-> f32	{
		self.get_floats(1u)[0]
	}

	pub fn position( &self )-> uint	{
		self.bin.tell()
	}

	pub fn enter( &mut self )-> ~str	{
		let c = (self.walk_fun)( self );
		self.chunks.push( c.clone() );
		c.name
	}

	pub fn skip( &self )	{
		let len = self.chunks.len();
		let end = self.chunks[len-1u].finish;
		self.bin.seek( end as int, io::SeekSet );
	}

	pub fn leave( &mut self )	{
		let c = self.chunks.pop();
		assert!( self.bin.tell() == c.finish );
	}

	pub fn has_more( &self )-> uint	{
		let len = self.chunks.len();
		let end = self.chunks[len-1u].finish;
		assert!( self.bin.tell() <= end );
		end - self.bin.tell()
	}
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Misc utilities											//

pub fn load_text( path : ~str )-> ~str	{
	match io::read_whole_file_str(&std::path::Path(path))	{
		Ok(text) => text,
		Err(msg) => fail!(msg)
	}
}

pub fn read_space( br : &Reader )-> space::QuatSpace	{
	let d = br.get_floats(8u);
	space::QuatSpace{
		position	: Vec3::new(d[0],d[1],d[2]),
		orientation	: Quat::new(d[7],d[4],d[5],d[6]),
		scale		: d[3],
	}
}

pub fn load_program( ct : &context::Context, path : &str, lg : &journal::Log )-> @shade::Program	{
	lg.add(fmt!( "Loading program: %s", path ));
	let sv = ct.create_shader( 'v', load_text( path + ".glslv" ));
	let sf = ct.create_shader( 'f', load_text( path + ".glslf" ));
	ct.create_program( [sv,sf], lg )
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Texture											//

fn create_texture_2D<T>( ct : &mut context::Context, image : &stb_image::image::Image<T>, mipmap : bool,
	int_format : gl::types::GLenum, pix_type : gl::types::GLenum )-> @texture::Texture	{
	//assert (image.width | image.height) & 3u == 0u;
	let format = match image.depth	{
		4u	=> gl::RGBA,
		3u	=> gl::RGB,
		_	=> fail!("Unknown image depth: %u", image.depth)
	};
	let t = ct.create_texture( "2D", image.width, image.height, 1, 0 );
	ct.texture.bind( t );
	ct.texture.load_2D( t, 0, int_format as gl::types::GLint,	format, pix_type, image.data );
	if mipmap	{
		ct.texture.generate_levels( t );
	}
	t
}

pub fn load_texture_2D_image( ct : &mut context::Context, result : &stb_image::image::LoadResult, mipmap : bool, name : &str )-> @texture::Texture	{
	match result	{
		&stb_image::image::ImageU8(ref img)		=>
			create_texture_2D( ct, img, mipmap, gl::RGBA, gl::UNSIGNED_BYTE ),
		&stb_image::image::ImageF32(ref img)	=>
			create_texture_2D( ct, img, mipmap, gl::RGB16F, gl::FLOAT ),
		&stb_image::image::Error			=>
			fail!("Unable to load image: %s", name)
	}
}

pub fn load_texture_2D( ct : &mut context::Context, path : &str, mipmap : bool )-> @texture::Texture	{
	let result = stb_image::image::load( path.to_owned() );
	load_texture_2D_image( ct, &result, mipmap, path )
}

pub struct TextureFuture	{
	name	: ~str,
	image	: Future<stb_image::image::LoadResult>,
	mipmap	: bool,
}

impl TextureFuture	{
	pub fn new_2D( path : ~str, mipmap : bool )-> TextureFuture	{
		TextureFuture	{
			name	: path.clone(),
			image	: Future::from_fn(|| {stb_image::image::load(path.clone())} ),
			mipmap	: mipmap,
		}
	}
	pub fn get( &mut self, ct : &mut context::Context )-> @texture::Texture	{
		let result = self.image.get_ref();
		load_texture_2D_image( ct, result, self.mipmap, self.name )
	}
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Mesh											//

pub fn read_mesh( br : &mut Reader, context : &mut context::Context, lg : &journal::Log )-> mesh::Mesh	{
	lg.add(fmt!( "Loading mesh from %s", br.path ));
	let signature = br.enter();
	if signature != ~"k3mesh"	{
		fail!("Invalid mesh signature '%s': %s", signature, br.path)
	}
	let name	= br.get_string();
	let n_vert	= br.get_uint(4u);
	let n_ind	= br.get_uint(4u);
	let topology= br.get_string();
	lg.add(fmt!( "\tName: %s, Vertices: %u, Indices: %u", name, n_vert, n_ind ));
	let mut mesh = context.create_mesh( name, topology, n_vert, n_ind );
	let mut num_buffers = br.get_uint(1u);
	while num_buffers>0u	{
		let buffer = context.create_buffer();
		let stride = br.get_uint(1u);
		let mut offset = 0u;
		let format = br.get_string();
		lg.add(fmt!( "\tbuf: stride:%u,\tformat:%s", stride, format ));
		let mut i = 0;
		while i < format.len()	{
			let name = ~"a_" + br.get_string();
			let is_fixed_point	= br.get_bool();
			let is_interpolated	= br.get_bool();
			let mut fm = format.slice(i,i+2).to_owned();
			if is_fixed_point	{ fm.push_char('.'); }
			if !is_interpolated	{ fm.push_char('!'); }
			lg.add(fmt!( "\t\tname:%s,\ttype:%s", name, fm ));
			let (at,size) = buf::Attribute::new( fm, buffer, stride, offset );
			if stride == 0u	{
				assert!( at.count == 1u );
				mesh.index = Some( at );
			}else	{
				mesh.attribs.insert( name, at );
			}
			offset += size;
			i += 2;
		}
		assert!( stride==0u || offset == stride );
		let size = if stride==0u {offset * n_ind} else {stride * n_vert};
		let data = br.bin.read_bytes( size );
		context.load_buffer( buffer, data, false );
		num_buffers -= 1u;
	}
	br.leave();
	mesh
}

pub fn load_mesh( path : ~str, ct : &mut context::Context, lg : &journal::Log )-> mesh::Mesh	{
	let mut rd = Reader::create_std( path );
	read_mesh( &mut rd, ct, lg )
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Armature										//

pub fn read_key_position( br : &Reader )-> Vec3<f32>	{
	let v = br.get_floats(3u);
	Vec3::new(v[0],v[1],v[2])
}
pub fn read_key_orientation_quat( br : &Reader )-> Quat<f32>	{
	let v = br.get_floats(4u);
	Quat::new(v[0],v[1],v[2],v[3])
}
pub fn read_key_orientation_euler( br : &Reader )-> Quat<f32>	{
	let v = br.get_floats(3u);
	Euler::new( angle::rad(v[0]), angle::rad(v[1]), angle::rad(v[2]) ).to_quat()
}
pub fn read_key_scale3( br : &Reader )-> f32	{
	let v = br.get_floats(3u);
	(v[0]+v[1]+v[2]) * (1f32/3f32)
}

pub fn read_curve<T : Clone + Send + space::Interpolate>( br : &Reader, fkey : &fn(&Reader)->T )-> ~anim::Curve<T>	{
	let num = br.get_uint(2u);
	let _extrapolate = br.get_uint(1u)!=0u;
	let bezier = br.get_uint(1u)!=0u;
	if bezier	{
		~std::vec::from_fn::< anim::KeyBezier<T> >(num, |_i|	{
			let time = br.get_float() as float;
			let co = fkey(br);
			let hl = fkey(br);
			let hr = fkey(br);
			anim::KeyBezier{ t:time, co:co, hl:hl, hr:hr }
		}) as ~anim::Curve<T>
	}else	{
		~std::vec::from_fn::< anim::KeySimple<T> >(num, |_i|	{
			let time = br.get_float() as float;
			let co = fkey(br);
			anim::KeySimple{ t:time, co:co }
		}) as ~anim::Curve<T>
	}
}


pub fn read_action( br : &mut Reader, bones : &[space::Bone], lg : &journal::Log )-> space::ArmatureRecord	{
	lg.add(fmt!( "Loading anim from %s", br.path ));
	assert!( br.enter() == ~"action" );
	let act_name = br.get_string();
	//final ani.Record rec = a.records[actName] = new ani.Record();
	let length = br.get_float() as float;
	lg.add(fmt!( "\tName: '%s', Length: %f sec", act_name, length as float ));
	let mut curves : ~[space::ArmatureCurve] = ~[];
	while br.has_more()!=0u	{
		assert!( br.enter() == ~"curve" );
		let curve_name = br.get_string();
		let dimension = br.get_uint(1u);
		lg.add(fmt!( "\t\tCurve: %s", curve_name ));
		let split = curve_name.split_iter('"').map(|w| {w.to_owned()}).to_owned_vec();
		if split.len() == 3u	{
			assert!( split[0] == ~"pose.bones[" );
			let mut bid = 0u;	//FIXME: vec::position, when Rust allows
			while bones[bid].node.name != split[1]	{
				bid += 1;
				assert!( bid < bones.len(), fmt!("Bone '%s' not found", split[1]) );
			}
			let arm_curve = match split[2]	{
				~"].location"	=>	{
					assert!( dimension == 3u );
					let c = read_curve( br, read_key_position );
					space::ACuPos(bid,c)
				},
				~"].rotation_quaternion"	=>	{
					assert!( dimension == 4u );
					let c = read_curve( br, read_key_orientation_quat );
					space::ACuRotQuat(bid,c)
				},
				~"].rotation_euler"	=>	{
					assert!( dimension == 3u );
					let c = read_curve( br, read_key_orientation_euler );
					space::ACuRotQuat(bid,c)
				},
				~"].scale"	=>	{
					assert!( dimension == 3u );
					let c = read_curve( br, read_key_scale3 );
					space::ACuScale(bid,c)
				},
				_	=> fail!("Unknown pose curve: %s", split[2])
			};
			curves.push( arm_curve );
		}else	{
			br.skip();
			fail!("Unknown curve: %s", curve_name)
		}
		br.leave();
	}
	br.leave();
	anim::Record{
		name:act_name, duration:length, curves:curves
	}
}

pub fn get_armature_shader( dual_quat : bool )-> (~str,uint)	{
	let shader = load_text( if dual_quat
		{~"data/code/mod/arm_dualquat.glslv"} else
		{~"data/code/mod/arm.glslv"} );
	let max : uint = {
		let start	= shader.find_str("MAX_BONES")			.expect("Has to have MAX_BONES");
		let end		= shader.slice_from(start).find(';')	.expect("Line has to end")		+ start;
		let npos	= shader.slice(start,end).rfind(' ')	.expect("Space is expected")	+ start;
		std::from_str::from_str( shader.slice(npos+1,end) )	.expect("Unable to parse int")
	};
	(shader,max)
}

pub fn read_armature( br : &mut Reader, root : @mut space::Node, dual_quat : bool, lg : &journal::Log )-> space::Armature	{
	let signature = br.enter();
	if signature != ~"k3arm"	{
		fail!("Invalid armature signature '%s': %s", signature, br.path)
	}
	// read bones
	let num_bones = br.get_uint(1u);
	lg.add(fmt!( "Loading armature of %u bones: %s", num_bones, br.path ));
	let mut bones : ~[space::Bone] = std::vec::with_capacity(num_bones);
	while bones.len()<num_bones	{
		let name = br.get_string();
		let pid = br.get_uint(1u);
		let parent = Some(if pid==0u {root}	else {bones[pid-1u].node});
		let space = read_space(br);
		let bind_inv = space.inverted();
		let bind_pose_inv = if pid==0u {bind_inv} else {bind_inv.concat( &bones[pid-1u].bind_pose_inv )};
		bones.push(space::Bone{
			node			: @mut space::Node{ name:name, space:space, parent:parent, actions:~[] },
			bind_space		: space,
			bind_pose_inv	: bind_pose_inv,
			transform		: space::QuatSpace::identity(),
			parent_id		: if pid==0u {None} else {Some(pid-1u)},
		});
	}
	// read actions
	let mut actions : ~[@space::ArmatureRecord] = ~[];
	while br.has_more()!=0u	{
		let rec = @read_action( br, bones, lg );
		actions.push(rec);
	}
	br.leave();
	// load shader
	let (shader,max) = get_armature_shader( dual_quat );
	lg.add(fmt!( "\tDetected %u bones", max ));
	// finish
	space::Armature{
		root	: root,
		bones	: bones,
		code	: shader,
		actions	: actions,
		max_bones	: max,
	}
}

pub fn load_armature( path : ~str, root : @mut space::Node, lg : &journal::Log )-> space::Armature	{
	let mut rd = Reader::create_std( path );
	read_armature( &mut rd, root, false, lg )
}