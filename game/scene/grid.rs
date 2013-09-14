extern mod lmath;
extern mod engine;

use lmath::vec::*;
use engine::{gr_low,gr_mid};
use engine::gr_low::context::GLType;
use engine::gr_low::rast::Color;
use engine::space::Space;

use scene = scene::common;


pub struct Grid	{
	selected	: Option<[uint,..2]>,
	priv mesh		: @gr_mid::mesh::Mesh,
	priv program	: @gr_low::shade::Program,
	priv data		: gr_low::shade::DataMap,
	priv rast		: gr_low::rast::State,
	priv nseg		: uint,
	priv texture	: @gr_low::texture::Texture,
	priv cells		: ~[Color],
	priv v_scale	: Vec4<f32>,
}

static CELL_EMPTY 	: uint	= 0x20802000;
static CELL_ACTIVE	: uint	= 0x2040E040;


pub impl Grid	{
	fn create( ct : &mut gr_low::context::Context, segments : uint, lg : &engine::journal::Log )-> Grid	{
		let mut data = gr_low::shade::DataMap::new();
		let mut rast = copy ct.default_rast;
		rast.prime.cull = true;
		rast.set_depth( ~"<=", false );
		rast.set_blend( ~"s+d", ~"Sa", ~"1" );
		let cells = do vec::from_fn::<Color>(segments*segments) |_i|	{
			Color::new(CELL_EMPTY)
		};
		let tex = ct.create_texture( ~"2D", segments, segments, 0u, 0u );
		let s_opt = Some( gr_low::texture::Sampler::new(1u,0) );
		data.insert( ~"t_Grid",		gr_low::shade::UniTexture(0,tex,s_opt) );
		let par_scale = vec4::new( 10f32, 10f32, 0.1f32, 0f32 );
		data.insert( ~"u_ScaleZ",	gr_low::shade::UniFloatVec(par_scale) );
		let oo_seg = 1f32 / (segments as f32);
		let par_size = vec4::new( oo_seg, oo_seg, 0f32, 0f32 );
		data.insert( ~"u_Size",		gr_low::shade::UniFloatVec(par_size) );
		Grid{
			selected: None,
			mesh	: @gr_mid::mesh::create_quad( ct ),
			program	: engine::load::load_program( ct, ~"data/code-game/grid", lg ),
			data	: data,
			rast	: rast,
			nseg	: segments,
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
	fn get_rectangle( &self )-> gr_low::frame::Rect	{
		gr_low::frame::Rect{
			x:0u, y:0u, w:self.nseg, h:self.nseg
		}
	}

	fn call( &self, fbo : @mut gr_low::frame::Buffer, pmap : gr_mid::call::PlaneMap,
			vao : @mut gr_low::buf::VertexArray )-> gr_mid::call::Call	{
		gr_mid::call::CallDraw(
			gr_mid::call::Input::new( vao, self.mesh ),
			gr_mid::call::Output::new( fbo, pmap ),
			copy self.rast, self.program, copy self.data )
	}

	priv fn upload_all_cells( &self, tb : &mut gr_low::texture::Binding )	{
		tb.bind( self.texture );
		let fm_pix = gr_low::texture::map_pix_format( ~"rgba" );
		let component = self.cells[0].r.to_gl_type();
		let r = self.get_rectangle();
		tb.load_sub_2D(	self.texture, 0u, &r, fm_pix, component, self.cells );
	}

	priv fn upload_single_cell( &self, tb : &mut gr_low::texture::Binding, x : uint, y : uint )	{
		tb.bind( self.texture );
		let col = self.cells[x + y*self.nseg];
		let fm_pix = gr_low::texture::map_pix_format( ~"rgba" );
		let component = col.r.to_gl_type();
		let r = gr_low::frame::Rect{ x:x, y:y, w:1u, h:1u };
		tb.load_sub_2D(	self.texture, 0u, &r, fm_pix, component, &[col] );
	}

	priv fn get_cell_selected( &self, cam : &scene::Camera, aspect : f32, nx : float, ny : float )-> (uint,uint)	{
		let ndc = vec3::new( (nx as f32)*2f32-1f32, 1f32-(ny as f32)*2f32, 0f32 );
		let origin = cam.node.world_space().position;
		let ray = cam.get_matrix(aspect).invert().transform( &ndc ).sub_v( &origin );
		let (x_unit,y_unit) = self.get_cell_size();
		let k = (self.v_scale.z - origin.z) / ray.z;
		let x = (origin.x + ray.x*k + self.v_scale.x) / x_unit;
		let y = (origin.y + ray.y*k + self.v_scale.y) / y_unit;
		(x as uint, y as uint)
	}

	fn init( &self, tb : &mut gr_low::texture::Binding )	{
		// init storage
		tb.bind( self.texture );
		let fm_int = gr_low::texture::map_int_format( ~"rgba8" );
		tb.init( self.texture, 1u, fm_int, true );
		// load data
		self.upload_all_cells(tb);
		// set up texture
	}

	fn update( &mut self, tb : &mut gr_low::texture::Binding, cam : &scene::Camera, aspect : f32, nx : float, ny : float )-> bool	{
		let view_proj = cam.get_matrix( aspect );
		self.data.insert( ~"u_ViewProj", gr_low::shade::UniMatrix(false,view_proj) );
		let (sx,sy) = self.get_cell_selected( cam, aspect, nx, ny );
		let (ox,oy) = match self.selected	{
			Some(sel) if sel[0]==sx && sel[1]==sy	=> return true,
			Some(sel)	=> (sel[0],sel[1]),
			None		=> (0u,0u)
		};
		self.selected = if sx<self.nseg && sy<self.nseg	{
			self.cells[ox + oy*self.nseg] = Color::new(CELL_EMPTY);
			self.upload_single_cell(tb,ox,oy);
			self.cells[sx + sy*self.nseg] = Color::new(CELL_ACTIVE);
			self.upload_single_cell(tb,sx,sy);
			Some([sx,sy])
		}else	{
			None
		};
		true
	}
}
