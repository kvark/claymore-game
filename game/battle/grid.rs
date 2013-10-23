extern mod cgmath;
extern mod engine;

use std;
use cgmath::{angle,rotation,vector};
use cgmath::quaternion::ToQuat;
use cgmath::vector::Vector;
use engine::{gr_low,gr_mid};
use engine::gr_low::context::GLType;
use engine::space::{QuatSpace,Space};

use scene = scene::common;


pub struct Location([int, ..2]);
pub type Orientation = int;

impl Location	{
	pub fn new( x : int, y : int )-> Location	{
		Location([x,y])
	}
}


pub struct Grid	{
	priv cells		: ~[u32],
	priv mesh		: @gr_mid::mesh::Mesh,
	priv program	: @gr_low::shade::Program,
	priv data		: gr_low::shade::DataMap,
	priv rast		: gr_low::rast::State,
	priv nseg		: uint,
	priv texture	: @gr_low::texture::Texture,
	priv v_scale	: vector::Vec4<f32>,
}

static CELL_EMPTY 	: u32	= 0x20802000;
static CELL_OCCUPIED: u32	= 0x80202020;
static CELL_ACTIVE	: u32	= 0x2040E040;


impl Grid	{
	pub fn create( ct : &mut gr_low::context::Context, segments : uint, lg : &engine::journal::Log )-> Grid	{
		let mut data = gr_low::shade::DataMap::new();
		let mut rast = ct.default_rast;
		rast.prime.cull = true;
		rast.set_depth( "<=", false );
		rast.set_blend( "s+d", "Sa", "1" );
		let cells = std::vec::from_elem( segments*segments, CELL_EMPTY );
		let tex = ct.create_texture( "2D", segments, segments, 0u, 0u );
		let s_opt = Some( gr_low::texture::Sampler::new(1u,0) );
		data.insert( ~"t_Grid",		gr_low::shade::UniTexture(0,tex,s_opt) );
		let par_scale = vector::Vec4::new( 10f32, 10f32, 0.1f32, 0f32 );
		data.insert( ~"u_ScaleZ",	gr_low::shade::UniFloatVec(par_scale) );
		let oo_seg = 1f32 / (segments as f32);
		let par_size = vector::Vec4::new( oo_seg, oo_seg, 0f32, 0f32 );
		data.insert( ~"u_Size",		gr_low::shade::UniFloatVec(par_size) );
		Grid{
			cells	: cells,
			mesh	: @gr_mid::mesh::create_quad( ct ),
			program	: engine::load::load_program( ct, "data/code-game/grid", lg ),
			data	: data,
			rast	: rast,
			nseg	: segments,
			texture	: tex,
			v_scale	: par_scale,
		}
	}

	pub fn get_rectangle( &self )-> gr_low::frame::Rect	{
		gr_low::frame::Rect::new( self.nseg, self.nseg )
	}

	pub fn update( &mut self, cam : &scene::Camera, aspect : f32 )	{
		cam.fill_data( &mut self.data, aspect );
	}

	pub fn compute_space( &self, pos : Location, orient : Orientation, elevation : f32 )-> QuatSpace	{
		let mut center = self.get_cell_center( pos );
		center.z = elevation;
		let angle = angle::deg( (orient as f32) * 90f32 );
		let rot = rotation::AxisAngle::new( vector::Vec3::unit_z(), angle );
		QuatSpace	{
			position 	: center,
			orientation	: rot.to_quat(),
			scale		: 1.0,
		}
	}
}


pub enum Cell	{
	CellEmpty,
	CellOccupied,
	CellFocus,
}

pub trait MutableGrid	{
	fn clear( &mut self );
	fn set_cell( &mut self, d : Location, cell : Cell )-> bool;
}

impl MutableGrid for Grid	{
	fn clear( &mut self )	{
		for c in self.cells.mut_iter()	{
			*c = CELL_EMPTY;
		}
	}
	fn set_cell( &mut self, d : Location, cell : Cell )-> bool	{
		let ns = self.nseg as int;
		if d[0]>=0 && d[0]<ns && d[1]>=0 && d[1]<ns	{
			self.cells[d[0] + d[1]*ns] = match cell	{
				CellEmpty		=> CELL_EMPTY,
				CellOccupied	=> CELL_OCCUPIED,
				CellFocus		=> CELL_ACTIVE,
			};
			true
		}else	{false}
	}
}


pub trait DrawableGrid	{
	fn init( &mut self, tb : &mut gr_low::texture::Binding );
	fn upload( &mut self, tb : &mut gr_low::texture::Binding );
	fn draw( &self, output : gr_mid::call::Output, vao : @mut gr_low::buf::VertexArray )-> gr_mid::call::Call;
}

impl DrawableGrid for Grid	{
	fn init( &mut self, tb : &mut gr_low::texture::Binding )	{
		// init storage
		tb.bind( self.texture );
		let fm_int = gr_low::texture::map_int_format( "rgba8" );
		tb.init( self.texture, 1u, fm_int, true );
		// load data
		self.upload(tb);
		// set up texture
	}
	fn upload( &mut self, tb : &mut gr_low::texture::Binding )	{
		tb.bind( self.texture );
		let fm_pix = gr_low::texture::map_pix_format( "rgba" );
		let component = 0u8.to_gl_type();
		let r = self.get_rectangle();
		tb.load_sub_2D(	self.texture, 0u, &r, fm_pix, component, self.cells );
	}
	fn draw( &self, output : gr_mid::call::Output, vao : @mut gr_low::buf::VertexArray )-> gr_mid::call::Call	{
		gr_mid::call::CallDraw(
			gr_mid::call::Input::new( vao, self.mesh ),
			output,
			self.rast, self.program, self.data.clone() )
	}
}


pub trait TopologyGrid	{
	fn get_cell_size( &self )-> (f32,f32);
	fn get_cell_center( &self, d : Location )-> vector::Vec3<f32>;
	fn ray_cast( &self, cam : &scene::Camera, aspect : f32, np : &[f32,..2] )-> Location;
}

impl TopologyGrid for Grid	{
	fn get_cell_size( &self )-> (f32,f32)	{
		(2f32*self.v_scale.x / (self.nseg as f32),
		 2f32*self.v_scale.y / (self.nseg as f32))
	}
	fn get_cell_center( &self, d : Location )-> vector::Vec3<f32>	{
		let (x_unit,y_unit) = self.get_cell_size();
		let half = (self.nseg as f32) * 0.5f32;
		vector::Vec3::new(
			((d[0] as f32)+0.5f32-half)*x_unit,
			((d[1] as f32)+0.5f32-half)*y_unit,
			self.v_scale.z )
	}
	fn ray_cast( &self, cam : &scene::Camera, aspect : f32, np : &[f32,..2] )-> Location	{
		let ndc = vector::Vec3::new( np[0]*2f32-1f32, 1f32-np[1]*2f32, 0f32 );
		let origin = cam.node.world_space().position;
		let ray = cam.get_matrix(aspect).inverted().transform( &ndc ).sub_v( &origin );
		let (x_unit,y_unit) = self.get_cell_size();
		let k = (self.v_scale.z - origin.z) / ray.z;
		let x = (origin.x + ray.x*k + self.v_scale.x) / x_unit;
		let y = (origin.y + ray.y*k + self.v_scale.y) / y_unit;
		Location::new( x as int, y as int )
	}
}