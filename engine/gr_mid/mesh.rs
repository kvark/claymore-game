extern mod glcore;

use core::hashmap::linear::LinearMap;
use core::managed;

use gr_low;
use gr_mid;


pub struct Range	{
	start	: uint,
	num		: uint,
}


pub struct Mesh	{
	name		: ~str,
	attribs		: LinearMap<~str,gr_low::buf::Attribute>,
	index		: Option<gr_low::buf::Attribute>,
	poly_type	: glcore::GLuint,
	num_vert	: uint,
	num_ind		: uint,
	black_list	: @mut ~[@gr_low::shade::Program]
}

pub impl Mesh	{
	fn get_poly_size( &self )-> uint	{
		match self.poly_type	{
			glcore::GL_POINT	=>1u,
			glcore::GL_LINES 		|
			glcore::GL_LINE_STRIP	=> 2u,
			glcore::GL_TRIANGLES		|
			glcore::GL_TRIANGLE_STRIP	|
			glcore::GL_TRIANGLE_FAN		=> 3u,
			_	=> fail!(fmt!( "Unknown poly type: %d",self.poly_type as int ))
		}
	}

	fn get_range( &self )-> Range	{
		match self.index	{
			Some(_)	=> Range{ start:0u, num:self.num_ind },
			None	=> Range{ start:0u, num:self.num_vert }
		}
	}
}


pub fn create_quad( ct : &mut gr_low::context::Context )-> Mesh	{
	let vdata = [0i8,0i8,1i8,0i8,0i8,1i8,1i8,1i8];
	let count = 2u;
	let mut mesh = ct.create_mesh( ~"grid", "3s", vdata.len()/count, 0u );
	let vat = ct.create_attribute( vdata, count, false );
	mesh.attribs.insert( ~"a_Vertex", vat );
	mesh
}


pub impl gr_low::context::Context	{
	fn create_mesh( &self, name : ~str, poly : &str, nv : uint, ni : uint )-> Mesh	{
		let ptype = match poly	{
			"1"	=> glcore::GL_POINTS,
			"2"	=> glcore::GL_LINES,
			"2s"=> glcore::GL_LINE_STRIP,
			"3"	=> glcore::GL_TRIANGLES,
			"3s"=> glcore::GL_TRIANGLE_STRIP,
			"3f"=> glcore::GL_TRIANGLE_FAN,
			_	=> fail!(fmt!( "Unknown poly type: %s", poly ))
		};
		let ats = LinearMap::new();
		Mesh{ name:name, attribs:ats, index:None, poly_type:ptype, num_vert:nv, num_ind:ni, black_list:@mut ~[] }
	}

	fn disable_mesh_attribs( &self, va : @mut gr_low::buf::VertexArray, clean_mask : uint )	{
		assert!( self.vertex_array.is_active(va) );
		let varray = &mut va.data;
		for uint::range(0,varray.len()) |i|	{
			if clean_mask&(1<<i)!=0u && varray[i].enabled	{
				glcore::glDisableVertexAttribArray( i as glcore::GLuint );
				varray[i].enabled = false;
			}
		}
	}

	fn bind_mesh_attrib( &mut self, va : @mut gr_low::buf::VertexArray, loc : uint, at : &gr_low::buf::Attribute, is_int : bool )	{
		assert!( self.vertex_array.is_active(va) );
		self.bind_buffer( at.buffer );
		let varray = &mut va.data;
		let vdata = &mut varray[loc];
		// update vertex info
		let need_bind = match vdata.attrib	{
			Some(ref attrib)	=> *attrib != *at,
			None				=> true
		};
		if need_bind	{
			vdata.attrib = Some(*at);
			let ptr = at.offset as *glcore::GLvoid;
			if is_int	{
				glcore::glVertexAttribIPointer(
					loc as glcore::GLuint, at.count as glcore::GLint, at.kind,
					at.stride as glcore::GLsizei, ptr );
			}else	{
				glcore::glVertexAttribPointer(
					loc as glcore::GLuint, at.count as glcore::GLint, at.kind,
					if at.normalized {glcore::GL_TRUE} else {glcore::GL_FALSE},
					at.stride as glcore::GLsizei, ptr );
			}
		}
		// enable attribute
		if !vdata.enabled	{
			glcore::glEnableVertexAttribArray( loc as glcore::GLuint );
			vdata.enabled = true;
		}
	}

	fn draw_mesh( &mut self, input : gr_mid::call::DrawInput, prog : @gr_low::shade::Program, data : &gr_low::shade::DataMap )-> bool	{
		let &(va,m,range) = &input;
		assert!( *va.handle as int != 0 );
		// check black list
		if m.black_list.find( |&p| managed::ptr_eq(p,prog) ).is_some()	{
			return false;
		}
		// bind program
		if !self.bind_program( prog, data )	{
			m.black_list.push( prog );
			io::println(fmt!( "Unable to activate program #%d", *prog.handle as int ));
			return false;
		}
		// bind attributes
		self.bind_vertex_array( va );
		let mut va_clean_mask = va.get_mask();
		for prog.attribs.each |&(name,pat)|	{
			match m.attribs.find(name)	{
				Some(sat) => {
					if !sat.compatible(pat)	{
						m.black_list.push( prog );
						io::println(fmt!( "Mesh attibute '%s' is incompatible with program #%d",
							*name, *prog.handle as int ));
						return false;
					}
					va_clean_mask &= !(1<<pat.loc);
					self.bind_mesh_attrib( va, pat.loc, sat, pat.is_integer() );
				},
				None => {
					m.black_list.push( prog );
					io::println(fmt!( "Mesh '%s' doesn't contain required attribute '%s', needed for program #%d",
						m.name, *name, *prog.handle as int ));
					return false;
				}
			}
		}
		self.disable_mesh_attribs( va, va_clean_mask );
		// call draw
		match m.index	{
			Some(el) =>	{
				self.bind_element_buffer( va, el.buffer );
				assert!( range.start + range.num <= m.num_ind );
				glcore::glDrawElements( m.poly_type, range.num as glcore::GLsizei, el.kind, range.start as *glcore::GLvoid );
			},
			None =>	{
				assert!( range.start + range.num <= m.num_vert );
				glcore::glDrawArrays( m.poly_type, range.start as glcore::GLint, range.num as glcore::GLsizei );
			}
		}
		true
	}
}
