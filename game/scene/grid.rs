extern mod lmath;
extern mod engine;

use lmath::vec::*;
use engine::context::GLType;
use engine::space::Space;

use scene = scene::common;


pub struct Grid	{
	priv mesh		: @engine::mesh::Mesh,
	priv program	: @engine::shade::Program,
	priv data		: engine::shade::DataMap,
	priv rast		: engine::rast::State,
	priv nseg		: uint,
	priv selected	: (uint,uint),
	priv texture	: @engine::texture::Texture,
	priv cells		: ~[engine::rast::Color],
	priv v_scale	: Vec4<f32>,
}

static CELL_EMPTY 	: uint	= 0x20802000;
static CELL_ACTIVE	: uint	= 0x2040E040;


pub impl Grid	{
	fn create( ct : &mut engine::context::Context, segments : uint, lg : &engine::context::Log )-> Grid	{
		let mut data = engine::shade::make_data();
		let mut rast = copy ct.default_rast;
		rast.prime.cull = true;
		rast.set_depth( ~"<=", false );
		rast.set_blend( ~"s+d", ~"Sa", ~"1" );
		let cells = do vec::from_fn::<engine::rast::Color>(segments*segments) |_i|	{
			engine::rast::Color::new(CELL_EMPTY)
		};
		let tex = ct.create_texture( ~"2D", segments, segments, 0u, 0u );
		let s_opt = Some( engine::texture::Sampler::new(1u,0) );
		data.insert( ~"t_Grid",		engine::shade::UniTexture(0,tex,s_opt) );
		let par_scale = vec4::new( 10f32, 10f32, 0.1f32, 0f32 );
		data.insert( ~"u_ScaleZ",	engine::shade::UniFloatVec(par_scale) );
		let oo_seg = 1f32 / (segments as f32);
		let par_size = vec4::new( oo_seg, oo_seg, 0f32, 0f32 );
		data.insert( ~"u_Size",		engine::shade::UniFloatVec(par_size) );
		Grid{
			mesh	: @engine::mesh::create_quad( ct ),
			program	: @engine::load::load_program( ct, ~"data/code-game/grid", lg ),
			data	: data,
			rast	: rast,
			nseg	: segments,
			selected: (0u,0u),
			texture	: tex,
			cells	: cells,
			v_scale	: par_scale,
		}
	}

	fn get_cell_size( &self )-> (f32,f32)	{
		(2f32*self.v_scale.x / (self.nseg as f32),
		 2f32*self.v_scale.y / (self.nseg as f32))
	}
	fn get_cell_center( &self, x: uint, y : uint )-> Vec3<f32>	{
		let (x_unit,y_unit) = self.get_cell_size();
		let half = (self.nseg as f32) * 0.5f32;
		vec3::new(
			((x as f32)+0.5f32-half)*x_unit,
			((y as f32)+0.5f32-half)*y_unit,
			self.v_scale.z )
	}
	fn get_rectangle( &self )-> engine::frame::Rect	{
		engine::frame::Rect{
			x:0u, y:0u, w:self.nseg, h:self.nseg
		}
	}

	fn call( &self, fbo : @mut engine::frame::Buffer, pmap : engine::call::PlaneMap,
			vao : @mut engine::buf::VertexArray )-> engine::call::Call	{
		engine::call::CallDraw( (vao, self.mesh, self.mesh.get_range()),
			(fbo, pmap, copy self.rast), self.program, copy self.data )
	}

	priv fn upload_all_cells( &self, tb : &mut engine::texture::Binding )	{
		tb.bind( self.texture );
		let fm_pix = engine::texture::map_pix_format( ~"rgba" );
		let component = self.cells[0].r.to_gl_type();
		let r = self.get_rectangle();
		tb.load_sub_2D(	self.texture, 0u, &r, fm_pix, component, self.cells );
	}

	priv fn upload_single_cell( &self, tb : &mut engine::texture::Binding, x : uint, y : uint )	{
		tb.bind( self.texture );
		let col = self.cells[x + y*self.nseg];
		let fm_pix = engine::texture::map_pix_format( ~"rgba" );
		let component = col.r.to_gl_type();
		let r = engine::frame::Rect{ x:x, y:y, w:1u, h:1u };
		tb.load_sub_2D(	self.texture, 0u, &r, fm_pix, component, &[col] );
	}

	priv fn get_cell_selected( &self, cam : &scene::Camera, nx : float, ny : float )-> (uint,uint)	{
		let ndc = vec3::new( (nx as f32)*2f32-1f32, 1f32-(ny as f32)*2f32, 0f32 );
		let origin = cam.node.world_space().position;
		let ray = cam.get_matrix().invert().transform( &ndc ).sub_v( &origin );
		let (x_unit,y_unit) = self.get_cell_size();
		let k = (self.v_scale.z - origin.z) / ray.z;
		let x = (origin.x + ray.x*k + self.v_scale.x) / x_unit;
		let y = (origin.y + ray.y*k + self.v_scale.y) / y_unit;
		(x as uint, y as uint)
	}

	fn init( &self, tb : &mut engine::texture::Binding )	{
		// init storage
		tb.bind( self.texture );
		let fm_int = engine::texture::map_int_format( ~"rgba8" );
		tb.init( self.texture, 1u, fm_int, true );
		// load data
		self.upload_all_cells(tb);
		// set up texture
	}

	fn update( &mut self, tb : &mut engine::texture::Binding, cam : &scene::Camera, nx : float, ny : float )-> (uint,uint,bool)	{
		let view_proj = cam.get_matrix();
		self.data.insert( ~"u_ViewProj", engine::shade::UniMatrix(false,view_proj) );
		let (sx,sy) = self.get_cell_selected( cam, nx, ny );
		if sx<self.nseg && sy<self.nseg && self.selected != (sx,sy)	{
			let (ox,oy) = self.selected;
			self.cells[ox + oy*self.nseg] = engine::rast::Color::new(CELL_EMPTY);
			self.upload_single_cell(tb,ox,oy);
			self.selected = (sx,sy);
			self.cells[sx + sy*self.nseg] = engine::rast::Color::new(CELL_ACTIVE);
			self.upload_single_cell(tb,sx,sy);
		}
		(sx,sy,true)
	}
}
