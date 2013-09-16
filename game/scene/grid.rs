extern mod lmath;
extern mod engine;

use lmath::vec::*;
use engine::{gr_low,gr_mid};
use engine::gr_low::context::GLType;
use engine::space::Space;

use scene = scene::common;

pub type Coordinate = [int, ..2];


pub struct Grid	{
	selected		: Option<Coordinate>,
	priv mesh		: @gr_mid::mesh::Mesh,
	priv program	: @gr_low::shade::Program,
	priv data		: gr_low::shade::DataMap,
	priv rast		: gr_low::rast::State,
	priv nseg		: uint,
	priv texture	: @gr_low::texture::Texture,
	priv cells		: ~[u32],
	priv v_scale	: Vec4<f32>,
}

static CELL_EMPTY 	: u32	= 0x20802000;
static CELL_ACTIVE	: u32	= 0x2040E040;


impl Grid	{
	pub fn create( ct : &mut gr_low::context::Context, segments : uint, lg : &engine::journal::Log )-> Grid	{
		let mut data = gr_low::shade::DataMap::new();
		let mut rast = copy ct.default_rast;
		rast.prime.cull = true;
		rast.set_depth( ~"<=", false );
		rast.set_blend( ~"s+d", ~"Sa", ~"1" );
		let cells = vec::from_elem( segments*segments, CELL_EMPTY );
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

	pub fn reset( &mut self )	{
		for self.cells.each_mut |c|	{
			*c = CELL_EMPTY;
		}
	}

	pub fn get_cell_size( &self )-> (f32,f32)	{
		(2f32*self.v_scale.x / (self.nseg as f32),
		 2f32*self.v_scale.y / (self.nseg as f32))
	}
	pub fn get_cell_center( &self, d : Coordinate )-> Vec3<f32>	{
		let (x_unit,y_unit) = self.get_cell_size();
		let half = (self.nseg as f32) * 0.5f32;
		vec3::new(
			((d[0] as f32)+0.5f32-half)*x_unit,
			((d[1] as f32)+0.5f32-half)*y_unit,
			self.v_scale.z )
	}
	pub fn get_rectangle( &self )-> gr_low::frame::Rect	{
		gr_low::frame::Rect{
			x:0u, y:0u, w:self.nseg, h:self.nseg
		}
	}

	pub fn get_cell( &self, d : Coordinate )-> Option<u32>	{
		let ns = self.nseg as int;
		if d[0]>=0 && d[0]<ns && d[1]>=0 && d[1]<ns	{
			Some(self.cells[ d[0] + d[1]*ns ])
		}else	{None}
	}

	pub fn mut_cell( &mut self, d : Coordinate, fun : &fn(&mut u32) )-> bool	{
		let ns = self.nseg as int;
		if d[0]>=0 && d[0]<ns && d[1]>=0 && d[1]<ns	{
			fun( &mut self.cells[ d[0] + d[1]*ns ] );
			true
		}else	{false}
	}

	pub fn call( &self, fbo : @mut gr_low::frame::Buffer, pmap : gr_mid::call::PlaneMap,
			vao : @mut gr_low::buf::VertexArray )-> gr_mid::call::Call	{
		gr_mid::call::CallDraw(
			gr_mid::call::Input::new( vao, self.mesh ),
			gr_mid::call::Output::new( fbo, pmap ),
			copy self.rast, self.program, copy self.data )
	}

	priv fn upload_all_cells( &self, tb : &mut gr_low::texture::Binding )	{
		tb.bind( self.texture );
		let fm_pix = gr_low::texture::map_pix_format( ~"rgba" );
		let component = 0u8.to_gl_type();
		let r = self.get_rectangle();
		tb.load_sub_2D(	self.texture, 0u, &r, fm_pix, component, self.cells );
	}

	priv fn upload_single_cell( &self, tb : &mut gr_low::texture::Binding, d : Coordinate )-> bool	{
		tb.bind( self.texture );
		match self.get_cell(d)	{
			Some(col)	=>	{
				let fm_pix = gr_low::texture::map_pix_format( ~"rgba" );
				let component = 0u8.to_gl_type();
				let r = gr_low::frame::Rect{ x:d[0] as uint, y:d[1] as uint, w:1u, h:1u };
				tb.load_sub_2D(	self.texture, 0u, &r, fm_pix, component, &[col] );
				true
			},
			None	=> false
		}	
	}

	priv fn get_cell_selected( &self, cam : &scene::Camera, aspect : f32, nx : float, ny : float )-> Coordinate	{
		let ndc = vec3::new( (nx as f32)*2f32-1f32, 1f32-(ny as f32)*2f32, 0f32 );
		let origin = cam.node.world_space().position;
		let ray = cam.get_matrix(aspect).invert().transform( &ndc ).sub_v( &origin );
		let (x_unit,y_unit) = self.get_cell_size();
		let k = (self.v_scale.z - origin.z) / ray.z;
		let x = (origin.x + ray.x*k + self.v_scale.x) / x_unit;
		let y = (origin.y + ray.y*k + self.v_scale.y) / y_unit;
		[x as int, y as int]
	}

	pub fn init( &self, tb : &mut gr_low::texture::Binding )	{
		// init storage
		tb.bind( self.texture );
		let fm_int = gr_low::texture::map_int_format( ~"rgba8" );
		tb.init( self.texture, 1u, fm_int, true );
		// load data
		self.upload_all_cells(tb);
		// set up texture
	}

	pub fn update( &mut self, tb : &mut gr_low::texture::Binding, cam : &scene::Camera, aspect : f32, nx : float, ny : float )-> bool	{
		let view_proj = cam.get_matrix( aspect );
		self.data.insert( ~"u_ViewProj", gr_low::shade::UniMatrix(false,view_proj) );
		let sp = self.get_cell_selected( cam, aspect, nx, ny );
		let op = match self.selected	{
			Some(sel) if sel==sp	=> return true,
			Some(sel)	=> sel,
			None		=> [0,0]
		};
		self.selected = match self.get_cell(sp)	{
			Some(_col)	=>	{
				let ns = self.nseg as int;
				self.cells[op[0] + op[1]*ns] = CELL_EMPTY;
				self.upload_single_cell(tb,op);
				self.cells[sp[0] + sp[1]*ns] = CELL_ACTIVE;
				self.upload_single_cell(tb,sp);
				Some(sp)
			},
			None	=> None,
		};
		true
	}
}
