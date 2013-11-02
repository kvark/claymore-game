extern mod cgmath;
extern mod engine;

use std;
use cgmath::{angle,vector};
use cgmath::angle::ToRad;
use cgmath::quaternion::Quat;
use cgmath::point::*;
use cgmath::vector::*;
use engine::{gr_low,gr_mid};
use engine::gr_low::context::GLType;
use engine::space::{QuatSpace,Space};

use scene = scene::common;


pub type Orientation	= int;
pub type Location		= Point2<int>;
pub type Offset			= Vec2<int>;
pub type Texel			= u32;


pub struct Grid	{
	priv nseg		: uint,
	priv cells		: ~[Texel],
	priv mesh		: @gr_mid::mesh::Mesh,
	priv program	: @gr_low::shade::Program,
	priv data		: gr_low::shade::DataMap,
	priv rast		: gr_low::rast::State,
	priv texture	: @gr_low::texture::Texture,
	priv v_scale	: vector::Vec4<f32>,
}

pub static CELL_EMPTY 	: Texel	= 0x20802000;
pub static CELL_OCCUPIED: Texel	= 0x80202020;
pub static CELL_ACTIVE	: Texel	= 0x2040E040;


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
			nseg	: segments,
			cells	: cells,
			mesh	: @gr_mid::mesh::create_quad( ct ),
			program	: engine::load::load_program( ct, "data/code-game/grid", lg ),
			data	: data,
			rast	: rast,
			texture	: tex,
			v_scale	: par_scale,
		}
	}

	pub fn get_index_size( &self )-> uint	{
		self.cells.len()
	}

	pub fn update( &mut self, cam : &scene::Camera, aspect : f32 )	{
		cam.fill_data( &mut self.data, aspect );
	}

	pub fn mut_cells<'a>( &'a mut self )-> &'a mut [Texel]	{
		self.cells.mut_slice_from(0)
	}

	pub fn clear( &mut self )	{
		for cell in self.cells.mut_iter()	{
			*cell = CELL_EMPTY;
		}
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
		let r = gr_low::frame::Rect::new( self.nseg, self.nseg );
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
	fn get_location( &self, index : uint )-> Location;
	fn get_index( &self, d : Location )-> Option<uint>;
	fn offset_position( &self, d : Location, o : Orientation, f : Offset )-> Location;
	fn get_neighbors( &self, index : uint )-> ~[uint];
}

impl TopologyGrid for Grid	{
	fn get_location( &self, index : uint )-> Location	{
		Point2::new( (index % self.nseg) as int, (index / self.nseg) as int )
	}
	fn get_index( &self, d : Location )-> Option<uint>	{
		let ns = self.nseg as int;
		if d.x>=0 && d.y<ns && d.y>=0 && d.y<ns	{
			Some((d.x + d.y*ns) as uint)
		}else	{None}
	}
	fn offset_position( &self, d : Location, o : Orientation, f : Offset )-> Location	{
		let offsets = [
			Vec2::new( 1i, 0i),	Vec2::new( 0i,-1i),
			Vec2::new(-1i, 0i),	Vec2::new( 0i, 1i),
			Vec2::new( 1i, 0i),	Vec2::new( 0i,-1i),
			];
		let off = offsets[o].mul_s(f.x).add_v( &offsets[o+2].mul_s(f.y) );
		d.add_v( &off )
	}
	fn get_neighbors( &self, index : uint )-> ~[uint]	{
		let d = self.get_location( index );
		range(0,4).filter_map( |o| {
			let dof = self.offset_position( d, o, Vec2::new(1i,0i) );
			self.get_index(dof)
		}).to_owned_vec()
	}
}


pub trait GeometryGrid : TopologyGrid	{
	fn get_cell_size( &self )-> Vec2<f32>;
	fn get_cell_center( &self, pos : Location )-> vector::Vec3<f32>;
	fn compute_space( &self, pos : Location, orient : Orientation, elevation : f32 )-> QuatSpace;
	fn point_cast( &self, point : &vector::Vec3<f32> )-> Location;
	fn ray_cast( &self, cam : &scene::Camera, aspect : f32, np : &[f32,..2] )-> Location;
}

impl GeometryGrid for Grid	{
	fn get_cell_size( &self )-> Vec2<f32>	{
		Vec2::new(
			2f32*self.v_scale.x / (self.nseg as f32),
		 	2f32*self.v_scale.y / (self.nseg as f32)
		 	)
	}
	fn get_cell_center( &self, pos : Location )-> vector::Vec3<f32>	{
		let unit = self.get_cell_size();
		let half = (self.nseg as f32) * 0.5f32;
		vector::Vec3::new(
			((pos.x as f32)+0.5f32-half)*unit.x,
			((pos.y as f32)+0.5f32-half)*unit.y,
			self.v_scale.z )
	}
	fn compute_space( &self, pos : Location, orient : Orientation, elevation : f32 )-> QuatSpace	{
		let mut center = self.get_cell_center( pos );
		center.z = elevation;
		let angle = angle::deg( (orient as f32) * 90f32 );
		let rot = Quat::from_axis_angle( &vector::Vec3::unit_z(), angle.to_rad() );
		QuatSpace	{
			position 	: center,
			orientation	: rot,
			scale		: 1.0,
		}
	}
	fn point_cast( &self, point : &vector::Vec3<f32> )-> Location	{
		let unit = self.get_cell_size();
		let x = (point.x + self.v_scale.x) / unit.x;
		let y = (point.y + self.v_scale.y) / unit.y;
		Point2::new( x as int, y as int )
	}
	fn ray_cast( &self, cam : &scene::Camera, aspect : f32, np : &[f32,..2] )-> Location	{
		let ndc = vector::Vec3::new( np[0]*2f32-1f32, 1f32-np[1]*2f32, 0f32 );
		let origin = cam.node.world_space().position;
		let ray = cam.get_matrix(aspect).inverted().transform( &ndc ).sub_v( &origin );
		let k = (self.v_scale.z - origin.z) / ray.z;
		//origin.add_v( &ray.mul_s(k) ).add_v( &self.v_scale ).div_v( &unit )
		self.point_cast( &origin.add_v( &ray.mul_s(k) ) )
	}
}