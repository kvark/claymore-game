extern mod glcore;

pub struct Range	{
	start	: uint,
	num		: uint,
}


pub struct Attribute	{
	// semantics
	kind			: glcore::GLenum,
	count			: uint,
	normalized		: bool,
	interpolated	: bool,
	// location
	buffer			: @buf::Object,
	stride			: uint,
	offset			: uint,
}

pub fn make_attribute<T:context::GLType>( ct : &context::Context, vdata : ~[T], count : uint, norm : bool )-> Attribute	{
	Attribute{
		kind: vdata[0].to_gl_type(),
		count: count,
		normalized: norm,
		interpolated: true,
		buffer: @ct.create_buffer_loaded( vdata ),
		stride: count * sys::size_of::<T>(),
		offset: 0u
	}
}

//FIXME: remove once auto-generated
impl Attribute : cmp::Eq	{
	pure fn eq( other : &Attribute )-> bool	{
		self.kind==other.kind && self.count==other.count &&
		self.normalized==other.normalized && self.interpolated==other.interpolated &&
		*self.buffer.handle==*other.buffer.handle &&
		self.stride==other.stride && self.offset==other.offset
	}
	pure fn ne( other : &Attribute )-> bool	{
		!self.eq( other )
	}
}

impl Attribute	{
	pure fn compatible( at : &shade::Attribute )-> bool	{
		//io::println(fmt!( "Checking compatibility: kind=0x%x, count=%u, storage=0x%x",
		//	self.kind as uint, self.count, at.storage as uint ));
		let (count,unit) = at.decompose();
		count == self.count && if at.is_integer()	{
			if unit == glcore::GL_INT	{
				[glcore::GL_BYTE,glcore::GL_SHORT,glcore::GL_INT].contains( &self.kind )
			}else
			if unit == glcore::GL_UNSIGNED_INT	{
				[glcore::GL_UNSIGNED_BYTE,glcore::GL_UNSIGNED_SHORT,glcore::GL_UNSIGNED_INT].contains( &self.kind )
			}else	{false}
		}else {true}
	}
}


pub struct Mesh	{
	name			: ~str,
	attribs			: send_map::linear::LinearMap<~str,Attribute>,
	index			: Option<Attribute>,
	poly_type		: glcore::GLuint,
	num_vert		: uint,
	num_ind			: uint,
	mut black_list	: ~[shade::Handle]
}

impl Mesh	{
	pure fn create_attrib( format : ~str, buffer : @buf::Object, stride : uint, offset : uint )-> (Attribute,uint)	{
		assert (format.len()==3u && ['.','!'].contains(&format.char_at(2))) ||
			format.len()==2u || (format.len()==4u && str::substr(format,2,2)==~".!");
		let count = (format[0] - "0"[0]) as uint;
		let letter = format.char_at(1);
		let is_fixed_point	= format.len()>2u	&& format.char_at(2)=='.';
		let can_interpolate	= format.len()<=2u	|| format.char_at(format.len()-1u)!='!';
		let (el_size,el_type) =
			if letter=='b'	{(1u,glcore::GL_BYTE)}				else
			if letter=='B'	{(1u,glcore::GL_UNSIGNED_BYTE)}		else
			if letter=='h'	{(2u,glcore::GL_SHORT)}				else
			if letter=='H'	{(2u,glcore::GL_UNSIGNED_SHORT)}	else
			if letter=='i'	{(4u,glcore::GL_INT)}				else
			if letter=='I'	{(4u,glcore::GL_UNSIGNED_INT)}		else
			if letter=='f'	{(4u,glcore::GL_FLOAT)}				else
			{fail(fmt!("Unknown attribute format: %s", format))};
		(Attribute{
			kind			: el_type,
			count			: count,
			normalized		: is_fixed_point,
			interpolated	: can_interpolate,
			buffer			: buffer,
			stride			: stride,
			offset			: offset,
		}, count * el_size)
	}

	pure fn create_index( format : ~str, buffer : @buf::Object )-> (Attribute,uint)	{
		self.create_attrib( format, buffer, 0u, 0u )
	}

	pure fn get_poly_size()-> uint	{
		if [glcore::GL_POINT].contains(&self.poly_type)	{
			1u
		}else
		if [glcore::GL_LINES,glcore::GL_LINE_STRIP].contains(&self.poly_type)	{
			2u
		}else
		if [glcore::GL_TRIANGLES,glcore::GL_TRIANGLE_STRIP,glcore::GL_TRIANGLE_FAN].contains(&self.poly_type)	{
			3u
		}else	{
			fail(fmt!( "Unknown poly type: %d",self.poly_type as int ));
		}
	}

	pure fn get_range()-> Range	{
		match self.index	{
			Some(_)	=> Range{ start:0u, num:self.num_ind },
			None	=> Range{ start:0u, num:self.num_vert }
		}
	}
}


impl context::Context	{
	fn create_mesh( name : ~str, poly : ~str, nv : uint, ni : uint )-> Mesh	{
		let ptype = if poly == ~"1"		{glcore::GL_POINTS}
			else	if poly == ~"2"		{glcore::GL_LINES}
			else	if poly == ~"2s"	{glcore::GL_LINE_STRIP}
			else	if poly == ~"3"		{glcore::GL_TRIANGLES}
			else	if poly == ~"3s"	{glcore::GL_TRIANGLE_STRIP}
			else	if poly == ~"3f"	{glcore::GL_TRIANGLE_FAN}
			else	{fail fmt!("Unknown poly type: %s",poly)};
		let ats = send_map::linear::LinearMap::<~str,Attribute>();
		Mesh{ name:name, attribs:ats, index:None, poly_type:ptype, num_vert:nv, num_ind:ni, black_list:~[] }
	}

	fn disable_mesh_attribs( va : &buf::VertexArray, clean_mask : uint )	{
		assert self.vertex_array.is_active(va);
		for va.data.eachi |i,vd|	{
			if clean_mask&(1<<i)!=0u && vd.enabled	{
				glcore::glDisableVertexAttribArray( i as glcore::GLuint );
				vd.enabled = false;
			}
		}
	}

	fn bind_mesh_attrib( va : &buf::VertexArray, loc : uint, at : &Attribute, is_int : bool )	{
		assert self.vertex_array.is_active(va);
		self.bind_buffer( at.buffer );
		let mut vdata = &va.data[loc];
		// update vertex info
		if vdata.attrib != *at	{
			vdata.attrib = *at;
			if is_int	{
				glcore::glVertexAttribIPointer(
					loc as glcore::GLuint, at.count as glcore::GLint, at.kind,
					at.stride as glcore::GLsizei, at.offset as *glcore::GLvoid );
			}else	{
				glcore::glVertexAttribPointer(
					loc as glcore::GLuint, at.count as glcore::GLint, at.kind,
					if at.normalized {glcore::GL_TRUE} else {glcore::GL_FALSE},
					at.stride as glcore::GLsizei, at.offset as *glcore::GLvoid );
			}
		}
		// enable attribute
		if !vdata.enabled	{
			glcore::glEnableVertexAttribArray( loc as glcore::GLuint );
			vdata.enabled = true;
		}
	}

	fn draw_mesh( m : &Mesh, range : &Range, va : &buf::VertexArray, prog : &shade::Program, data : &shade::DataMap )-> bool	{
		// check black list
		if m.black_list.contains( &prog.handle )	{
			return false;
		}
		// bind program
		if !self.bind_program( prog, data )	{
			m.black_list.push( prog.handle );
			io::println(fmt!( "Unable to activate program #%d", *prog.handle as int ));
			return false;
		}
		// bind attributes
		self.bind_vertex_array( va );
		let mut va_clean_mask = va.get_mask();
		for prog.attribs.each |name,pat|	{
			match m.attribs.find(name)	{
				Some(sat) => {
					if !sat.compatible(pat)	{
						m.black_list.push( prog.handle );
						io::println(fmt!( "Mesh attibute '%s' is incompatible with program #%d",
							*name, *prog.handle as int ));
						return false;
					}
					va_clean_mask &= !(1<<pat.loc);
					self.bind_mesh_attrib( va, pat.loc, &sat, pat.is_integer() );
				},
				None => {
					m.black_list.push( prog.handle );
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
				assert range.start + range.num <= m.num_ind;
				glcore::glDrawElements( m.poly_type, range.num as glcore::GLsizei, el.kind, range.start as *glcore::GLvoid );
			},
			None =>	{
				assert range.start + range.num <= m.num_vert;
				glcore::glDrawArrays( m.poly_type, range.start as glcore::GLint, range.num as glcore::GLsizei );
			}
		}
		true
	}
}