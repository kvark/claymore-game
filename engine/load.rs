extern mod glcore;
extern mod lmath;
extern mod stb_image;

use io::ReaderUtil;
use lmath::vec::vec3::*;
use lmath::quat::*;

//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Chunk reader									//

pub struct Chunk	{
	name	: ~str,
	size	: uint,
	finish	: uint,
}


pub struct Reader	{
	path	: ~str,
	priv bin		: @io::Reader,
	priv walk_fun	: @static/ fn (&Reader)->Chunk,
	priv mut chunks	: ~[Chunk],
}

pub fn enter_dummy( _rd : &Reader )-> Chunk	{
	fail ~"unexpected chunk read"
}

pub fn enter_chunk( rd : &Reader )-> Chunk	{
	let name_bin = rd.get_bytes(8u);
	let size = rd.get_uint(4u);
	let name_clean = do name_bin.filter()	|b| {*b != 0u8};
	Chunk	{
		name	: str::from_bytes(name_clean),
		size	: size,
		finish	: rd.position() + size,
	}
}


impl Reader	{
	fn get_bytes( num : uint )-> ~[u8]	{
		self.bin.read_bytes(num)
	}
	fn get_uint( size : uint )-> uint	{
		let bytes = self.get_bytes(size);
		do vec::foldr(bytes,0u) |t,u|	{(u<<8u) + (*t as uint)}
	}

	fn get_bool()-> bool	{
		self.get_uint(1u) != 0u
	}

	fn get_string()-> ~str	{
		let size = self.bin.read_byte() as uint;
		str::from_bytes( self.bin.read_bytes(size) )
	}

	fn get_floats( num : uint )-> ~[f32]	{
		let data = self.bin.read_bytes( num * 4u );
		let mut vals : ~[f32];
		unsafe	{
			vals = vec::raw::from_buf_raw( vec::raw::to_ptr(data) as *f32, num );
		}
		vals
	}
	fn get_float()-> f32	{
		self.get_floats(1u)[0]
	}

	fn position()-> uint	{
		self.bin.tell()
	}

	fn enter()-> ~str	{
		let c = (self.walk_fun)(&self);
		self.chunks.push(copy c);
		copy c.name
	}

	fn skip()	{
		let len = self.chunks.len();
		let end = self.chunks[len-1u].finish;
		self.bin.seek( end as int, io::SeekSet );
	}

	fn leave()	{
		let c = self.chunks.pop();
		assert self.bin.tell() == c.finish;
	}

	fn has_more()-> uint	{
		let len = self.chunks.len();
		let end = self.chunks[len-1u].finish;
		assert self.bin.tell() <= end;
		end - self.bin.tell()
	}
}

pub fn create_reader_ext( path : ~str, fun : @static/ fn(&Reader)->Chunk )-> Reader	{
	let p = path::Path( path );
	match io::file_reader(&p)	{
		Ok(bin)		=> Reader{ path:path, bin:bin, walk_fun:fun, chunks:~[] },
		Err(msg)	=> fail fmt!( "Unable to read %s: %s", path, msg )
	}
}

pub fn create_reader_std( path : ~str )-> Reader	{
	create_reader_ext( path, enter_chunk )
}

pub fn create_reader( path : ~str )-> Reader	{
	create_reader_ext( path, enter_dummy )
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Misc utilities											//

pub fn load_text( path : ~str )-> ~str	{
	match io::read_whole_file_str(&path::Path(path))	{
		Ok(text) => copy text,
		Err(msg) => fail msg
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

pub fn load_program( ct : &context::Context, path : ~str, lg : &context::Log )-> shade::Program	{
	lg.add(fmt!( "Loading program: %s", path ));
	let sv = ct.create_shader( 'v', load_text( path + ~".glslv" ));
	let sf = ct.create_shader( 'f', load_text( path + ~".glslf" ));
	ct.create_program( ~[sv,sf], lg )
}

fn create_texture_2D<T>( ct : &context::Context, image : &stb_image::image::Image<T>, mipmap : bool,
	int_format : glcore::GLenum, pix_type : glcore::GLenum )-> texture::Texture	{
	assert (image.width | image.height) & 3u == 0u;
	let format = match image.depth	{
		4u	=> glcore::GL_RGBA,
		3u	=> glcore::GL_RGB,
		_	=> fail fmt!("Unknown image depth: %u", image.depth)
	};
	let t = ct.create_texture( ~"2D", image.width, image.height, 1, 0 );
	ct.texture.bind( &t );
	ct.texture.load_2D( &t, 0, int_format as glcore::GLint,	format, pix_type, &image.data );
	if mipmap	{
		ct.texture.generate_levels( &t );
	}
	t
}

pub fn load_texture_2D( ct : &context::Context, path : &~str, mipmap : bool )-> texture::Texture	{
	match stb_image::image::load(copy *path)	{
		stb_image::image::ImageU8(img)	=>
			create_texture_2D( ct, &img, mipmap, glcore::GL_RGBA, glcore::GL_UNSIGNED_BYTE ),
		stb_image::image::ImageF32(img)	=>
			create_texture_2D( ct, &img, mipmap, glcore::GL_RGB16F, glcore::GL_FLOAT ),
		stb_image::image::Error			=>
			fail fmt!( "Unable to load image: %s", *path )
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Mesh											//

pub fn read_mesh( br : &Reader, context : &context::Context, lg : &context::Log )-> mesh::Mesh	{
	let signature = br.enter();
	if signature != ~"k3mesh"	{
		fail fmt!( "Invalid mesh signature '%s': %s", signature, br.path )
	}
	let n_vert	= br.get_uint(4u);
	let n_ind	= br.get_uint(4u);
	lg.add(fmt!( "Loading mesh of %u vertices and %u indices: %s", n_vert, n_ind, br.path ));
	let mut mesh = context.create_mesh( br.get_string(), ~"3", n_vert, n_ind );
	let mut num_buffers = br.get_uint(1u);
	while num_buffers>0u	{
		let buffer = @context.create_buffer();
		let stride = br.get_uint(1u);
		let mut offset = 0u;
		let format = br.get_string();
		lg.add(fmt!( "\tbuf: stride:%u,\tformat:%s", stride, format ));
		let mut i = 0;
		while i < format.len()	{
			let name = ~"a_" + br.get_string();
			let mut fm = str::substr( format, i, 2 );
			if br.get_bool()	{ fm += ~"."; }
			if !br.get_bool()	{ fm += ~"!"; }
			lg.add(fmt!( "\t\tname:%s,\ttype:%s", name, fm ));
			let (at,size) = mesh.create_attrib( fm, buffer, stride, offset );
			if stride == 0u	{
				assert at.count == 1u;
				mesh.index = Some( at );
			}else	{
				mesh.attribs.insert( name, at );
			}
			offset += size;
			i += 2;
		}
		assert stride==0u || offset == stride;
		let size = if stride==0u {offset * n_ind} else {stride * n_vert};
		let data = br.bin.read_bytes( size );
		context.load_buffer( buffer, data, false );
		num_buffers -= 1u;
	}
	br.leave();
	mesh
}

pub fn load_mesh( path : ~str, ct : &context::Context, lg : &context::Log )-> mesh::Mesh	{
	let rd = create_reader_std( path );
	read_mesh( &rd, ct, lg )
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
pub fn read_key_scale3( br : &Reader )-> f32	{
	let v = br.get_floats(3u);
	(v[0]+v[1]+v[2]) * (1f32/3f32)
}

pub fn read_curve<T : space::Interpolate>( br : &Reader, fkey : &fn(&Reader)->T )-> anim::Curve<T>	{
	let num = br.get_uint(2u);
	let _extrapolate = br.get_uint(1u)!=0u;
	let bezier = br.get_uint(1u)!=0u;
	if bezier	{
		@do vec::from_fn::< anim::KeyBezier<T> >(num) |_i|	{
			let time = br.get_float() as float;
			let co = fkey(br), hl = fkey(br), hr = fkey(br);
			anim::KeyBezier{ t:time, co:co, hl:hl, hr:hr }
		} as @anim::Curve<T>
	}else	{
		@do vec::from_fn::< anim::KeySimple<T> >(num) |_i|	{
			let time = br.get_float() as float;
			let co = fkey(br);
			anim::KeySimple{ t:time, co:co }
		} as @anim::Curve<T>
	}
}


pub fn read_armature( br : &Reader, root : @space::Node, dual_quat : bool, lg : &context::Log )-> space::Armature	{
	let signature = br.enter();
	if signature != ~"k3arm"	{
		fail fmt!( "Invalid armature signature '%s': %s", signature, br.path )
	}
	// read bones
	let num_bones = br.get_uint(1u);
	lg.add(fmt!( "Loading armature of %u bones: %s", num_bones, br.path ));
	let mut bones : ~[space::Bone] = vec::with_capacity(num_bones);
	while bones.len()<num_bones	{
		let name = br.get_string();
		let pid = br.get_uint(1u);
		let parent = Some(if pid==0u {root}	else {bones[pid-1u].node});
		let space = read_space(br);
		let bind_inv = space.invert();
		bones.push(space::Bone{
			node			: @space::Node{ name:name, space:space, parent:parent, actions:~[] },
			bind_space		: space,
			bind_pose_inv	: if pid==0u {bind_inv} else {bind_inv * bones[pid-1u].bind_pose_inv},
			transform		: space::identity(),
			parent_id		: if pid==0u {None} else {Some(pid-1u)},
		});
	}
	// read actions
	let mut actions : ~[@space::ArmatureRecord] = ~[];
	while br.has_more()!=0u	{
		assert br.enter() == ~"action";
		let act_name = br.get_string();
		//final ani.Record rec = a.records[actName] = new ani.Record();
		let length = br.get_float() as float;
		lg.add(fmt!( "\tAnim '%s' of length %f", act_name, length as float ));
		let mut curves : ~[space::ArmatureCurve] = ~[];
		while br.has_more()!=0u	{
			assert br.enter() == ~"curve";
			let curve_name = br.get_string();
			let dimension = br.get_uint(1u);
			lg.add( ~"\t\tCurve" + curve_name );
			let split = curve_name.split_char('"');
			if split.len() == 3u	{
				assert split[0] == ~"pose.bones[";
				let mut bid = 0u;	//FIXME: vec::position, when Rust allows
				while bones[bid].node.name != split[1]	{
					bid += 1;
					assert bid < bones.len();
				}
				if split[2] == ~"].location"	{
					assert dimension == 3u;
					let c = read_curve( br, read_key_position );
					curves.push( space::ACuPos(bid,c) );
				}else
				if split[2] == ~"].rotation_quaternion"	{
					assert dimension == 4u;
					let c = read_curve( br, read_key_orientation_quat );
					curves.push( space::ACuRotQuat(bid,c) );
				}else
				if split[2] == ~"].rotation_euler"	{
					assert dimension == 3u;
					read_curve( br, read_key_position );
					//curves.push( space)	//FIXME!
				}else
				if split[2] == ~"].scale"	{
					assert dimension == 3u;
					let c = read_curve( br, read_key_scale3 );
					curves.push( space::ACuScale(bid,c) );
				}else {
					fail fmt!( "Unable to find curve '%s'", split[2] )
				};
			}
			br.leave();
		}
		br.leave();
		actions.push(@anim::Record{
			name:act_name, duration:length, curves:curves
		});
	}
	br.leave();
	// load shader
	let shader = load_text( if dual_quat
		{~"data/code/mod/arm_dualquat.glslv"} else
		{~"data/code/mod/arm.glslv"} );
	let max = {
		let start	= str::find_str(shader,~"MAX_BONES")	.expect(~"Has to have MAX_BONES");
		let end		= str::find_char_from(shader,';',start)	.expect(~"Line has to end");
		let split	= shader.slice(start,end).split_char(' ');
		uint::from_str( split[split.len()-1u] )				.expect(~"Unable to parse int")
	};
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

pub fn load_armature( path : ~str, root : @space::Node, lg : &context::Log )-> space::Armature	{
	let rd = create_reader_std( path );
	read_armature( &rd, root, false, lg )
}