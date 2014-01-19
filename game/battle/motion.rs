use cgmath::point::{Point};
use cgmath::vector::{EuclideanVector,Vector};
use engine::{anim,space};
use engine::anim::Player;

use battle::{grid,field,main,unit};
use battle::think;


pub struct Dummy;

impl think::Motion for Dummy	{
	fn get_name<'a>( &'a self )-> &'a str	{ "None" }
	fn update( &mut self, _m: &mut main::Member, _delta: anim::float, _field: &mut field::Field, _grid: &grid::Grid )-> think::MotionStatus	{
		think::StatusCanInterrupt
	}
	fn stop( &mut self )	{}
}


pub struct Idle	{
	armature		: space::ArmaturePtr,
	priv record		: space::ArmatureRecordPtr,
	priv last_time	: anim::float,
}

impl Idle	{
	pub fn new( arm: &space::ArmaturePtr, name: ~str )-> Idle	{
		let record = arm.borrow().with( |a| a.find_record(name).expect(~"character Idle not found") );
		Idle	{
			armature	: arm.clone(),
			record		: record,
			last_time	: 0.0,
		}
	}
	pub fn to_ptr( self )-> think::MotionPtr	{
		~self as think::MotionPtr
	}
}

impl think::Motion for Idle	{
	fn get_name<'a>( &'a self )-> &'a str	{ "Idle" }
	fn update( &mut self, _m: &mut main::Member, delta: anim::float, _field: &mut field::Field, _grid: &grid::Grid )-> think::MotionStatus	{
		let moment = self.last_time + delta;
		self.last_time = if moment>self.record.borrow().duration	{0.0} else	{moment};
		self.armature.borrow().with_mut( |a| a.set_record(self.record.borrow(),moment) );
		think::StatusCanInterrupt
	}
	fn stop( &mut self )	{}
}


pub struct Move	{
	destinations: ~[grid::Location],
	location	: grid::Location,
	orientation	: grid::Orientation,
	elevation	: f32,
	speed		: f32,
}

impl think::Motion for Move	{
	fn get_name<'a>( &'a self )-> &'a str	{ "Move" }
	
	fn update( &mut self, m: &mut main::Member, full_delta: anim::float, field: &mut field::Field, grid: &grid::Grid )-> think::MotionStatus	{
		let mut delta = full_delta as f32;
		let root = m.get_root();
		let mut pos = root.borrow().with(|n| Point::from_vec( &n.space.disp ));
		while delta > 0.0 && !self.destinations.is_empty()	{
			let mut dest_pos = (grid as &grid::GeometryGrid).get_cell_center( self.destinations[0] );
			dest_pos.z = self.elevation;
			let dest_vector = dest_pos.sub_p( &pos );
			let dest_len = dest_vector.length();
			let time_left = dest_len / self.speed;
			pos = if delta >= time_left	{
				delta -= time_left;
				self.destinations.shift();
				dest_pos
			}else	{
				let move_vector = dest_vector.mul_s( delta * self.speed / dest_len );
				delta = 0.0;
				pos.add_v( &move_vector )
			};
		}
		let dest_loc = (grid as &grid::GeometryGrid).point_cast( &pos );
		let done : bool = if dest_loc != self.location	{
			//print(format!( "Location {:s} -> {:s}\n", self.location.to_str(), dest_loc.to_str() ));
			match field.get_by_location( dest_loc, grid as &grid::TopologyGrid )	{
				&field::CellEmpty	=>	{
					//m.update_link( dest_loc, self.orientation );	//FIXME!!!
					//field.update_member( key, field.get_member(key), m, grid as &grid::TopologyGrid ); 
					false
				},
				&field::CellPart(_,_)	=>	{	//collide
					pos = Point::from_vec( &(grid as &grid::GeometryGrid).compute_space(
						dest_loc, self.orientation, self.elevation ).disp);
					true
				},
				_	=> fail!("Unexpected path cell: {:s}", dest_loc.to_str())
			}
		}else {false};
		
		root.borrow().with_mut( |n| {n.space.disp = pos.to_vec();} );
		
		if done || self.destinations.is_empty()	{
			think::StatusDone
		}else	{
			think::StatusCanInterrupt
		}
	}
	
	fn stop( &mut self )	{
		//TODO
	}
}


pub struct Attack	{
	destination	: grid::Location,
	damage		: unit::Health,
}
impl think::Motion for Attack	{
	fn get_name<'a>( &'a self )-> &'a str	{ "Attack" }
	fn update( &mut self, _m: &mut main::Member, _delta: anim::float, field: &mut field::Field, grid: &grid::Grid )-> think::MotionStatus	{
		let id = (grid as &grid::TopologyGrid).get_index( self.destination ).expect("Invalid attack target");
		field.deal_damage( id, None, self.damage );
		think::StatusDone
	}
	fn stop( &mut self )	{
		//TODO
	}
}