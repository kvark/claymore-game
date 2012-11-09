extern mod glcore;


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

impl Attribute	{
	fn compatible( _at : &shade::Attribute )-> bool	{
		//FIXME
		true
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

	fn bind_mesh_element( loc : uint, at : &Attribute )	{
		self.buffer_array.bind( at.buffer );
		let mut vdata = &self.vertex_array.data[loc];
		// update vertex info
		//FIXME: cache this
		glcore::glVertexAttribPointer(
			loc as glcore::GLuint, at.count as glcore::GLint, at.kind,
			if at.normalized {glcore::GL_TRUE} else {glcore::GL_FALSE},
			at.stride as glcore::GLsizei, at.offset as *glcore::GLvoid
			);
		// enable attribute
		if !vdata.enabled	{
			glcore::glEnableVertexAttribArray( loc as glcore::GLuint );
			vdata.enabled = true;
		}
	}

	fn draw_mesh( m : &Mesh, prog : &shade::Program, data : &shade::DataMap )-> bool	{
		assert *self.vertex_array.handle != 0;
		// check black list
		if m.black_list.contains( &prog.handle )	{
			return false;
		}
		// bind program
		if !self.bind_program( prog, data )	{
			m.black_list.push( prog.handle );
			io::println(fmt!( "Unable to activate program '%d'", *prog.handle as int ));
			return false;
		}
		// bind attributes
		self.disable_vertex_attribs();
		for prog.attribs.each |name,pat|	{
			match m.attribs.find(name)	{
				Some(sat) => {
					if !sat.compatible(pat)	{
						m.black_list.push( prog.handle );
						io::println(fmt!( "Mesh attibute '%s' is incompatible with program '%d'",
							*name, *prog.handle as int ));
						return false;
					}
					self.bind_mesh_element( pat.loc, &sat );
				},
				None => {
					m.black_list.push( prog.handle );
					io::println(fmt!( "Mesh '%s' doesn't contain required attribute '%s', needed for program '%d'",
						m.name, *name, *prog.handle as int ));
					return false;
				}
			}
		}
		// FIXME: bind elements
		// issue a draw call
		glcore::glDrawArrays( m.poly_type, 0, m.num_vert as glcore::GLsizei );
		true
	}
}