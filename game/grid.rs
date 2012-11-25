extern mod lmath;
extern mod engine;


pub struct Grid	{
	priv mesh			: @engine::mesh::Mesh,
	priv program		: @engine::shade::Program,
	priv mut data		: engine::shade::DataMap,
	priv rast			: engine::rast::State,
	priv nseg			: uint,
	priv mut selected	: (uint,uint),
	priv texture		: @engine::texture::Texture,
	priv mut cells		: ~[engine::rast::Color],
	priv v_scale		: lmath::vector::vec4,
}

const CELL_EMPTY 	: uint	= 0x20802000;
const CELL_ACTIVE	: uint	= 0x2040E040;


impl Grid	{
	pub pure fn get_cell_size()-> (f32,f32)	{
		(2f32*self.v_scale.x / (self.nseg as f32),
		 2f32*self.v_scale.y / (self.nseg as f32))
	}
	pub pure fn get_cell_center( x: uint, y : uint )-> lmath::vector::vec3	{
		let (x_unit,y_unit) = self.get_cell_size();
		let half = (self.nseg as f32) * 0.5f32;
		lmath::vector::Vec3::new(
			((x as f32)+0.5f32-half)*x_unit,
			((y as f32)+0.5f32-half)*y_unit,
			self.v_scale.z )
	}
	pub pure fn get_rectangle()-> engine::frame::Rect	{
		engine::frame::Rect{
			x:0u, y:0u, w:self.nseg, h:self.nseg
		}
	}

	pure fn call( fbo : @engine::frame::Buffer, pmap : engine::call::PlaneMap,
			vao : @engine::buf::VertexArray )-> engine::call::Call	{
		engine::call::CallDraw( fbo, pmap, vao, self.mesh, self.mesh.get_range(),
			self.program, copy self.data, copy self.rast )
	}

	priv fn upload_all_cells( tb : &engine::texture::Binding )	{
		tb.bind( self.texture );
		let fm_pix = engine::texture::map_pix_format( ~"rgba" );
		let component = (self.cells[0].r as @engine::context::GLType).to_gl_type();
		let r = self.get_rectangle();
		tb.load_sub_2D(	self.texture, 0u, &r, fm_pix, component, &const self.cells );
	}

	priv fn upload_single_cell( tb : &engine::texture::Binding, x : uint, y : uint )	{
		tb.bind( self.texture );
		let col = self.cells[x + y*self.nseg];
		let fm_pix = engine::texture::map_pix_format( ~"rgba" );
		let component = (col.r as @engine::context::GLType).to_gl_type();
		let r = engine::frame::Rect{ x:x, y:y, w:1u, h:1u };
		tb.load_sub_2D(	self.texture, 0u, &r, fm_pix, component, &const ~[col] );
	}

	priv fn get_cell_selected( cam : &main::Camera, nx : float, ny : float )-> (uint,uint)	{
		let ndc = lmath::vector::Vec3::new( (nx as f32)*2f32-1f32, 1f32-(ny as f32)*2f32, 0f32 );
		let origin = cam.node.world_space().position;
		let ray = cam.get_matrix().inverse().transform( &ndc ).sub_v( &origin );
		let (x_unit,y_unit) = self.get_cell_size();
		let k = (self.v_scale.z - origin.z) / ray.z;
		let x = (origin.x + ray.x*k + self.v_scale.x) / x_unit;
		let y = (origin.y + ray.y*k + self.v_scale.y) / y_unit;
		(x as uint, y as uint)
	}

	pub fn init( tb : &engine::texture::Binding )	{
		// init storage
		tb.bind( self.texture );
		let fm_int = engine::texture::map_int_format( ~"rgba8" );
		tb.init_2D(	self.texture, 1u, fm_int, true );
		// load data
		self.upload_all_cells(tb);
		// set up texture
		tb.wrap(	self.texture, 0 );
		tb.filter(	self.texture, 1u );
	}

	pub fn update( tb : &engine::texture::Binding, cam : &main::Camera, nx : float, ny : float )-> bool	{
		let view_proj = cam.get_matrix();
		self.data.insert( ~"u_ViewProj", engine::shade::UniMatrix(false,view_proj) );
		let (sx,sy) = self.get_cell_selected( cam, nx, ny );
		if sx<self.nseg && sy<self.nseg && self.selected != (sx,sy)	{
			let (ox,oy) = self.selected;
			self.cells[ox + oy*self.nseg] = engine::rast::make_color(CELL_EMPTY);
			self.upload_single_cell(tb,ox,oy);
			self.selected = (sx,sy);
			self.cells[sx + sy*self.nseg] = engine::rast::make_color(CELL_ACTIVE);
			self.upload_single_cell(tb,sx,sy);
		}
		true
	}
}


fn make_quad( ct : &engine::context::Context )-> engine::mesh::Mesh	{
	let vdata = ~[0i8,0i8,1i8,0i8,0i8,1i8,1i8,1i8];
	let count = 2u;
	let mut mesh = ct.create_mesh( ~"grid", ~"3s", vdata.len()/count, 0u );
	let vat = engine::mesh::make_attribute( ct, vdata, count, false );
	mesh.attribs.insert( ~"a_Vertex", vat );
	mesh
}

pub fn make_grid( ct : &engine::context::Context, segments : uint )-> Grid	{
	let mut data = engine::shade::create_data();
	let mut rast = engine::rast::create_rast(0,0);
	rast.prime.cull = true;
	rast.set_depth( ~"<=", false );
	rast.set_blend( ~"s+d", ~"Sa", ~"1" );
	let cells = do vec::from_fn::<engine::rast::Color>(segments*segments) |_i|	{
		engine::rast::make_color(CELL_EMPTY)
	};
	let tex = @ct.create_texture( ~"2D", segments, segments, 0u, 0u );
	data.insert( ~"t_Grid",		engine::shade::UniTexture(0,tex) );
	let par_scale = lmath::vector::Vec4::new( 10f32, 10f32, 0.1f32, 0f32 );
	data.insert( ~"u_ScaleZ",	engine::shade::UniFloatVec(par_scale) );
	let oo_seg = 1f32 / (segments as f32);
	let par_size = lmath::vector::Vec4::new( oo_seg, oo_seg, 0f32, 0f32 );
	data.insert( ~"u_Size",		engine::shade::UniFloatVec(par_size) );
	Grid{
		mesh	: @make_quad( ct ),
		program	: @engine::load::load_program( ct, ~"data/code-game/grid" ),
		data	: data,
		rast	: rast,
		nseg	: segments,
		selected: (0u,0u),
		texture	: tex,
		cells	: cells,
		v_scale	: par_scale,
	}
}
