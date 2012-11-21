extern mod glcore;
extern mod lmath;
use io::ReaderUtil;

//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Chunk reader									//

const NAME_SIZE	:uint = 8;

struct Reader	{
	path	: ~str,
	bin		: @io::Reader,
	priv mut chunks	: ~[uint],
}


impl Reader	{
	fn get_uint( size : uint )-> uint	{
		let bytes = self.bin.read_bytes(size);
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

	fn enter()-> ~str	{
		let name_bin = self.bin.read_bytes( NAME_SIZE );
		let size = self.get_uint(4u);
		self.chunks.push( self.bin.tell() + size );
		let name_clean = do name_bin.filter()	|b| {*b != 0u8};
		str::from_bytes(name_clean)
	}

	fn leave()	{
		let end = self.chunks.pop();
		assert self.bin.tell() == end;
	}

	fn has_more()-> bool	{
		self.bin.tell() < self.chunks.last()
	}
}


pub fn create_reader( path : ~str )->Reader	{
	let p = path::Path( path );
	match io::file_reader(&p)	{
		Ok(bin)		=> Reader{ path:path, bin:bin, chunks:~[] },
		Err(msg)	=> fail(fmt!( "Unable to read %s: %s", path, msg ))
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Misc utilities											//

pub fn read_text( path:~str )-> ~str	{
	match io::read_whole_file_str(&path::Path(path))	{
		Ok(text) => copy text,
		Err(msg) => fail(msg)
	}
}

pub fn read_space( br : &Reader )-> space::QuatSpace	{
	let d = br.get_floats(8u);
	space::QuatSpace{
		position	: lmath::vector::Vec3::new(d[0],d[1],d[2]),
		orientation	: lmath::quaternion::Quat::new(d[7],d[4],d[5],d[6]),
		scale		: d[3],
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Mesh											//

pub fn read_mesh( br : &Reader, context : &context::Context )-> mesh::Mesh	{
	let signature = br.enter();
	if signature != ~"k3mesh"	{
		fail(fmt!( "Invalid mesh signature '%s': %s", signature, br.path ));
	}
	let n_vert	= br.get_uint(4u);
	let n_ind	= br.get_uint(4u);
	io::println(fmt!( "Loading mesh of %u vertices and %u indices: %s", n_vert, n_ind, br.path ));
	let mut mesh = context.create_mesh( br.get_string(), ~"3", n_vert, n_ind );
	let mut num_buffers = br.get_uint(1u);
	while num_buffers>0u	{
		let buffer = @context.create_buffer();
		let stride = br.get_uint(1u);
		let mut offset = 0u;
		let format = br.get_string();
		io::println(fmt!( "\tbuf: stride:%u, format:%s", stride, format ));
		let mut i = 0;
		while i < format.len()	{
			let name = ~"a_" + br.get_string();
			let mut fm = str::substr( format, i, 2 );
			if br.get_bool()	{ fm += ~"."; }
			if !br.get_bool()	{ fm += ~"!"; }
			io::println(fmt!( "\t\tname:%s, type:%s", name, fm ));
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

//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Armature										//

pub fn read_key_position( br : &Reader )-> lmath::vector::vec3	{
	let v = br.get_floats(3u);
	lmath::vector::Vec3::new(v[0],v[1],v[2])
}
pub fn read_key_orientation_quat( br : &Reader )-> lmath::quaternion::quat4	{
	let v = br.get_floats(4u);
	lmath::quaternion::Quat::new(v[0],v[1],v[2],v[3])
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


pub fn read_armature( br : &Reader, node_opt : Option<@space::Node>, _dual_quat : bool )-> space::Armature	{
	let signature = br.enter();
	if signature != ~"k3arm"	{
		fail(fmt!( "Invalid armature signature '%s': %s", signature, br.path ));
	}
	// read bones
	let num_bones = br.get_uint(1u);
	io::println(fmt!( "Loading armature of %u bones: %s", num_bones, br.path ));
	let mut bones : ~[space::Bone] = vec::with_capacity(num_bones);
	while bones.len()<num_bones	{
		let name = br.get_string();
		let pid = br.get_uint(1u);
		let parent = if pid==0u {node_opt}	else {Some(bones[pid-1u].node)};
		let space = read_space(br);
		bones.push(space::Bone{
			node		: @space::Node{ name:name, space:space, parent:parent, actions:~[] },
			bind_pose	: space,
			transform	: space::identity(),
			parent_id	: if pid==0u {None} else {Some(pid-1u)},
		});
	}
	// read actions
	let mut actions : ~[@space::ArmatureRecord] = ~[];
	while br.has_more()	{
		assert br.enter() == ~"action";
		let act_name = br.get_string();
		//final ani.Record rec = a.records[actName] = new ani.Record();
		let length = br.get_float() as float;
		io::println(fmt!( "\tAnim '%s' of length %f", act_name, length as float ));
		let mut curves : ~[space::ArmatureCurve] = ~[];
		while br.has_more()	{
			assert br.enter() == ~"curve";
			let curve_name = br.get_string();
			let dimension = br.get_uint(1u);
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
				if split[2] == ~"].scale"	{
					assert dimension == 3u;
					let c = read_curve( br, read_key_scale3 );
					curves.push( space::ACuScale(bid,c) );
				}else {
					fail(fmt!( "Unable to find curve '%s'", split[2] ));
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
	// finish
	//FIXME: extract max_bones from the shader
	space::Armature{
		node	: node_opt,
		bones	: bones,
		code	: read_text(~"data/code/mod/arm.glslv"),
		actions	: actions,
		max_bones		: 100u,
		last_updated	: 0u,
	}
}
