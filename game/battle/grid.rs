extern mod cgmath;
extern mod engine;

use std;
use cgmath::{angle,point,transform,vector};
use cgmath::angle::ToRad;
use cgmath::matrix::Matrix;
use cgmath::point::Point;
use cgmath::quaternion::Quat;
use cgmath::transform::Transform;
use cgmath::vector::Vector;
use engine::{gr_low,gr_mid};
use engine::gr_low::context::GLType;
use engine::space;

use scene = scene::common;


pub type Orientation	= int;
pub type Location		= point::Point2<int>;
pub type Offset			= vector::Vec2<int>;
pub type Texel			= u32;


pub struct Grid	{
	priv nseg		: uint,
	priv cells		: ~[Texel],
	priv mesh		: gr_mid::mesh::MeshPtr,
	priv program	: gr_low::shade::ProgramPtr,
	priv data		: gr_low::shade::DataMap,
	priv rast		: gr_low::rast::Rast,
	priv texture	: gr_low::texture::TexturePtr,
	priv v_scale	: vector::Vec4<f32>,
}

pub static CELL_EMPTY 	: Texel	= 0x20802000;
pub static CELL_OCCUPIED: Texel	= 0x80202020;
pub static CELL_ACTIVE	: Texel	= 0x2040E040;


impl Grid	{
	pub fn create( ct: &mut gr_low::context::Context, segments: uint, lg: &engine::journal::Log )-> Grid	{
		let mut data = gr_low::shade::DataMap::new();
		let mut rast = ct.default_rast;
		rast.prime.cull = true;
		rast.set_depth( "<=", false );
		rast.set_blend( "s+d", "Sa", "1" );
		let cells = std::vec::from_elem( segments*segments, CELL_EMPTY );
		lg.add(format!( "Grid created of size {}x{}", segments, segments ));
		let tex = ct.create_texture( "2D", segments, segments, 0u, 0u );
		let s_opt = Some( gr_low::texture::Sampler::new(1u,0) );
		data.set( ~"t_Grid",		gr_low::shade::UniTexture(0,tex.clone(),s_opt) );
		let par_scale = vector::Vec4::new( 10f32, 10f32, 0.1f32, 0f32 );
		data.set( ~"u_ScaleZ",	gr_low::shade::UniFloatVec(par_scale) );
		let oo_seg = 1f32 / (segments as f32);
		let par_size = vector::Vec4::new( oo_seg, oo_seg, 0f32, 0f32 );
		data.set( ~"u_Size",	gr_low::shade::UniFloatVec(par_size) );
		Grid{
			nseg	: segments,
			cells	: cells,
			mesh	: gr_mid::mesh::create_quad( ct ),
			program	: engine::load::load_program( ct, "data/code-game/grid", lg ),
			data	: data,
			rast	: rast,
			texture	: tex,
			v_scale	: par_scale,
		}
	}

	pub fn update( &mut self, cam: &scene::Camera, aspect: f32 )	{
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
	fn init( &mut self, &mut gr_low::texture::Binding );
	fn upload( &mut self, &mut gr_low::texture::Binding );
	fn draw( &self, gr_mid::call::Output, gr_low::buf::VertexArrayPtr )-> gr_mid::call::Call;
}

impl DrawableGrid for Grid	{
	fn init( &mut self, tb: &mut gr_low::texture::Binding )	{
		// init storage
		tb.bind( &self.texture );
		let fm_int = gr_low::texture::map_int_format( "rgba8" );
		tb.init( &self.texture, 1u, fm_int, true );
		// load data
		self.upload(tb);
		// set up texture
	}
	fn upload( &mut self, tb: &mut gr_low::texture::Binding )	{
		tb.bind( &self.texture );
		let fm_pix = gr_low::texture::map_pix_format( "rgba" );
		let component = 0u8.to_gl_type();
		let r = gr_low::frame::Rect::new( self.nseg, self.nseg );
		tb.load_sub_2D(	&self.texture, 0u, &r, fm_pix, component, self.cells );
	}
	fn draw( &self, output: gr_mid::call::Output, vao: gr_low::buf::VertexArrayPtr )-> gr_mid::call::Call	{
		gr_mid::call::CallDraw(
			gr_mid::call::Input::new( &vao, &self.mesh ),
			output,
			self.rast, self.program.clone(), self.data.clone() )
	}
}


pub trait TopologyGrid	{
	fn get_index_size( &self )-> uint;
	fn get_location( &self, uint )-> Location;
	fn get_index( &self, Location )-> Option<uint>;
	fn offset_position( &self, Location, Orientation, Offset )-> Location;
	fn approximate_orientation( &self, Location, Location )-> Orientation;
	fn get_neighbors( &self, Location )-> ~[Location];
	
	fn get_neighbors_indices( &self, index: uint )-> ~[uint]	{
		self.get_neighbors( self.get_location(index) ).
			move_iter().filter_map( |loc| self.get_index(loc) ).
			to_owned_vec()
	}
}

impl TopologyGrid for Grid	{
	fn get_index_size( &self )-> uint	{
		self.cells.len()
	}
	fn get_location( &self, index: uint )-> Location	{
		point::Point2::new( (index % self.nseg) as int, (index / self.nseg) as int )
	}
	fn get_index( &self, d: Location )-> Option<uint>	{
		let ns = self.nseg as int;
		if d.x>=0 && d.x<ns && d.y>=0 && d.y<ns	{
			let id = (d.x + d.y*ns) as uint;
			assert!( id < self.get_index_size() );
			Some(id)
		}else	{None}
	}
	fn offset_position( &self, d: Location, o: Orientation, f: Offset )-> Location	{
		let offsets = [
			vector::Vec2::new( 1i, 0i),	vector::Vec2::new( 0i,-1i),
			vector::Vec2::new(-1i, 0i),	vector::Vec2::new( 0i, 1i),
			vector::Vec2::new( 1i, 0i),	vector::Vec2::new( 0i,-1i),
			];
		let off = offsets[o].mul_s(f.x).add_v( &offsets[o+2].mul_s(f.y) );
		d.add_v( &off )
	}
	fn approximate_orientation( &self, _src: Location, _dst: Location )-> Orientation	{
		0	//TODO
	}
	fn get_neighbors( &self, d: Location )-> ~[Location]	{
		range(0,4).map(|o| self.offset_position( d, o, vector::Vec2::new(1i,0i) )).
			to_owned_vec()
	}
}


pub trait GeometryGrid : TopologyGrid	{
	fn get_cell_size( &self )-> vector::Vec2<f32>;
	fn get_cell_center( &self, Location )-> point::Point3<f32>;
	fn compute_space( &self, Location, Orientation, f32 )-> space::Space;
	fn point_cast( &self, &point::Point3<f32> )-> Location;
	fn ray_cast( &self, &scene::Camera, f32, &[f32,..2] )-> Location;
}

impl GeometryGrid for Grid	{
	fn get_cell_size( &self )-> vector::Vec2<f32>	{
		vector::Vec2::new(
			2f32*self.v_scale.x / (self.nseg as f32),
		 	2f32*self.v_scale.y / (self.nseg as f32)
		 	)
	}
	fn get_cell_center( &self, pos: Location )-> point::Point3<f32>	{
		let unit = self.get_cell_size();
		let half = (self.nseg as f32) * 0.5f32;
		point::Point3::new(
			((pos.x as f32)+0.5f32-half)*unit.x,
			((pos.y as f32)+0.5f32-half)*unit.y,
			self.v_scale.z )
	}
	fn compute_space( &self, pos: Location, orient: Orientation, elevation: f32 )-> space::Space	{
		let mut center = self.get_cell_center( pos );
		center.z = elevation;
		let angle = angle::deg( (orient as f32) * 90f32 );
		let rot = Quat::from_axis_angle( &vector::Vec3::unit_z(), angle.to_rad() );
		space::make( 1.0, rot, center.to_vec() )
	}
	fn point_cast( &self, point: &point::Point3<f32> )-> Location	{
		let unit = self.get_cell_size();
		let x = (point.x + self.v_scale.x) / unit.x;
		let y = (point.y + self.v_scale.y) / unit.y;
		point::Point2::new( x as int, y as int )
	}
	fn ray_cast( &self, cam: &scene::Camera, aspect: f32, np: &[f32,..2] )-> Location	{
		let ndc = point::Point3::new( np[0]*2f32-1f32, 1f32-np[1]*2f32, 0f32 );
		let origin = cam.node.borrow().with( |n|	{
			point::Point::from_vec(&n.world_space().disp)
			});
		let cit = transform::AffineMatrix3{ mat:cam.get_inverse_matrix(aspect) };
		let ray = cit.transform_point( &ndc ).sub_p( &origin );
		let k = (self.v_scale.z - origin.z) / ray.z;
		//origin.add_v( &ray.mul_s(k) ).add_v( &self.v_scale ).div_v( &unit )
		self.point_cast( &origin.add_v( &ray.mul_s(k) ) )
	}
}
