extern mod gl;

use std::managed;
use std::cell::RefCell;
use std::hashmap::HashMap;

use gr_low;
use gr_mid;


#[deriving(Clone)]
pub struct Range	{
	start	: uint,
	num		: uint,
}


pub struct Mesh	{
	name		: ~str,
	attribs		: HashMap<~str,gr_low::buf::Attribute>,
	index		: Option<gr_low::buf::Attribute>,
	poly_type	: gl::types::GLuint,
	num_vert	: uint,
	num_ind		: uint,
	black_list	: RefCell<~[@gr_low::shade::Program]>,
}

impl Mesh	{
	pub fn get_poly_size( &self )-> uint	{
		match self.poly_type	{
			gl::POINT	=>1u,
			gl::LINES 		|
			gl::LINE_STRIP	=> 2u,
			gl::TRIANGLES		|
			gl::TRIANGLE_STRIP	|
			gl::TRIANGLE_FAN		=> 3u,
			_	=> fail!("Unknown poly type: {:i}", self.poly_type as int)
		}
	}

	pub fn get_range( &self )-> Range	{
		match self.index	{
			Some(_)	=> Range{ start:0u, num:self.num_ind },
			None	=> Range{ start:0u, num:self.num_vert }
		}
	}
}


pub fn create_quad( ct: &mut gr_low::context::Context )-> Mesh	{
	let vdata = [0i8,0i8,1i8,0i8,0i8,1i8,1i8,1i8];
	let count = 2u;
	let mut mesh = ct.create_mesh( ~"quad", "3s", vdata.len()/count, 0u );
	let vat = ct.create_attribute( vdata, count, false );
	mesh.attribs.insert( ~"a_Vertex", vat );
	mesh
}


impl gr_low::context::Context	{
	pub fn create_mesh( &self, name: ~str, poly: &str, nv: uint, ni: uint )-> Mesh	{
		let ptype = match poly	{
			"1"	=> gl::POINTS,
			"2"	=> gl::LINES,
			"2s"=> gl::LINE_STRIP,
			"3"	=> gl::TRIANGLES,
			"3s"=> gl::TRIANGLE_STRIP,
			"3f"=> gl::TRIANGLE_FAN,
			_	=> fail!("Unknown poly type: {:s}", poly)
		};
		let ats = HashMap::new();
		Mesh{ name:name, attribs:ats, index:None, poly_type:ptype, num_vert:nv, num_ind:ni,
			black_list: RefCell::new(~[]) }
	}

	pub fn disable_mesh_attribs( &self, vap: &gr_low::buf::VertexArrayPtr, clean_mask: uint )	{
		assert!( self.vertex_array.is_active(vap) );
		let mut va = vap.borrow().borrow_mut();
		let varray = &mut va.get().data;
		for i in range(0,varray.len())	{
			if clean_mask&(1<<i)!=0u && varray[i].enabled	{
				gl::DisableVertexAttribArray( i as gl::types::GLuint );
				varray[i].enabled = false;
			}
		}
	}

	pub fn bind_mesh_attrib( &mut self, vap: &gr_low::buf::VertexArrayPtr, loc: uint, at: &gr_low::buf::Attribute, is_int: bool )	{
		assert!( self.vertex_array.is_active(vap) );
		self.bind_buffer( at.buffer );
		let mut va = vap.borrow().borrow_mut();
		let varray = &mut va.get().data;
		let vdata = &mut varray[loc];
		// update vertex info
		let need_bind = match vdata.attrib	{
			Some(ref attrib)	=> *attrib != *at,
			None				=> true
		};
		if need_bind	{
			vdata.attrib = Some(*at);
			let ptr = at.offset as *gl::types::GLvoid;
			unsafe{
				if is_int	{
					gl::VertexAttribIPointer(
						loc as gl::types::GLuint, at.count as gl::types::GLint, at.kind,
						at.stride as gl::types::GLsizei, ptr );
				}else	{
					gl::VertexAttribPointer(
						loc as gl::types::GLuint, at.count as gl::types::GLint, at.kind,
						if at.normalized {gl::TRUE} else {gl::FALSE},
						at.stride as gl::types::GLsizei, ptr );
				}
			}
		}
		// enable attribute
		if !vdata.enabled	{
			gl::EnableVertexAttribArray( loc as gl::types::GLuint );
			vdata.enabled = true;
		}
	}

	pub fn draw_mesh( &mut self, inp: &gr_mid::call::Input, prog: @gr_low::shade::Program, data: &gr_low::shade::DataMap )-> bool	{
		assert!({ let gr_low::buf::ArrayHandle(h) = inp.va.borrow().borrow().get().handle; h != 0 });
		// check black list
		let mut black = inp.mesh.black_list.borrow_mut();
		if black.get().iter().find( |&p| managed::ptr_eq(*p,prog) ).is_some()	{
			return false;
		}
		// bind program
		let gr_low::shade::ProgramHandle(phan) = prog.handle;
		if !self.bind_program( prog, data )	{
			black.get().push( prog );
			print!( "Unable to activate program {}{}\n", '#', phan );
			return false;
		}
		// bind attributes
		self.bind_vertex_array( inp.va.clone() );
		let mut va_clean_mask = {
			let va = inp.va.borrow().borrow();
			va.get().get_mask()
		};
		for (name,pat) in prog.attribs.iter()	{
			match inp.mesh.attribs.find(name)	{
				Some(sat) => {
					if !sat.compatible(pat)	{
						black.get().push( prog );
						print!( "Mesh attibute '{}' is incompatible with program {}{}\n",
							*name, '#', phan );
						return false;
					}
					va_clean_mask &= !(1<<pat.loc);
					self.bind_mesh_attrib( &inp.va, pat.loc, sat, pat.is_integer() );
				},
				None => {
					black.get().push( prog );
					print!( "Mesh '{}' doesn't contain required attribute '{}', needed for program {}{}\n",
						inp.mesh.name, *name, '#', phan );
					return false;
				}
			}
		}
		self.disable_mesh_attribs( &inp.va, va_clean_mask );
		// call draw
		match inp.mesh.index	{
			Some(el) =>	{
				self.bind_element_buffer( &inp.va, el.buffer );
				assert!( inp.range.start + inp.range.num <= inp.mesh.num_ind );
				unsafe{
					gl::DrawElements( inp.mesh.poly_type, inp.range.num as gl::types::GLsizei,
						el.kind, inp.range.start as *gl::types::GLvoid );
				}
			},
			None =>	{
				assert!( inp.range.start + inp.range.num <= inp.mesh.num_vert );
				gl::DrawArrays( inp.mesh.poly_type, inp.range.start as gl::types::GLint,
					inp.range.num as gl::types::GLsizei );
			}
		}
		true
	}
}
