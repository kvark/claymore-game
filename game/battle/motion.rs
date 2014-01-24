use std::to_str::ToStr;
use cgmath::point::{Point};
use cgmath::vector::{EuclideanVector,Vector};
use engine::{anim,space};
use engine::anim::Player;

use battle::{grid,field,main,unit};
use battle::think;


pub struct Dummy;

impl ToStr for Dummy	{
	fn to_str( &self )-> ~str	{
		~"Dummy"
	}
}

impl<M: main::Member> think::Motion<M> for Dummy	{
	fn update( &mut self, _key: field::MemberKey, _m: &mut M, _delta: anim::float, _field: &mut field::Field, _grid: &grid::Grid )-> think::MotionStatus	{
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
	pub fn to_ptr<M: main::Member>( self )-> think::MotionPtr<M>	{
		~self as think::MotionPtr<M>
	}
}

impl ToStr for Idle	{
	fn to_str( &self )-> ~str	{
		~"Idle"
	}
}

impl<M: main::Member> think::Motion<M> for Idle	{
	fn update( &mut self, _key: field::MemberKey, _m: &mut M, delta: anim::float, _field: &mut field::Field, _grid: &grid::Grid )-> think::MotionStatus	{
		let moment = self.last_time + delta;
		self.last_time = if moment>self.record.borrow().duration	{0.0} else	{moment};
		self.armature.borrow().with_mut( |a| a.set_record(self.record.borrow(),moment) );
		think::StatusCanInterrupt
	}
	fn stop( &mut self )	{}
}


pub struct Move	{
	destinations: ~[grid::Location],
	orientation	: grid::Orientation,
	elevation	: f32,
	speed		: f32,
}

impl ToStr for Move	{
	fn to_str( &self )-> ~str	{
		format!( "Move{:s}", self.destinations.to_str() )
	}
}

impl<M: main::Member + 'static> think::Motion<M> for Move	{	
	fn update( &mut self, key: field::MemberKey, m: &mut M, full_delta: anim::float, field: &mut field::Field, grid: &grid::Grid )-> think::MotionStatus	{
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
		let self_loc = {let link = field.get_member(key); link.location};
		let dest_loc = (grid as &grid::GeometryGrid).point_cast( &pos );
		let done : bool = if dest_loc != self_loc	{
			//print(format!( "Location {:s} -> {:s}\n", self.location.to_str(), dest_loc.to_str() ));
			match field.get_by_location( dest_loc, grid as &grid::TopologyGrid )	{
				&field::CellEmpty	=>	{
					field.update_member( key, m, grid as &grid::TopologyGrid, |link|	{
						link.location = dest_loc;
						link.orientation = self.orientation;
					});
					false
				},
				&field::CellPart(_,_)	=>	{	//collide
					//println!( "Collided with {:s} while moving from {:s}", dest_loc.to_str(), self_loc.to_str() );
					pos = Point::from_vec( &(grid as &grid::GeometryGrid).compute_space(
						self_loc, self.orientation, self.elevation ).disp);
					true
				},
				_	=> fail!("Unexpected path cell: {:s}", dest_loc.to_str())
			}
		}else {false};
		
		root.borrow().with_mut( |n| {n.space.disp = pos.to_vec()} );
		
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

impl ToStr for Attack	{
	fn to_str( &self )-> ~str	{
		format!( "Attack{:s}", self.destination.to_str() )
	}
}

impl<M: main::Member> think::Motion<M> for Attack	{
	fn update( &mut self, _key: field::MemberKey, _m: &mut M, _delta: anim::float, field: &mut field::Field, grid: &grid::Grid )-> think::MotionStatus	{
		let id = (grid as &grid::TopologyGrid).get_index( self.destination ).expect("Invalid attack target");
		field.deal_damage( id, None, self.damage );
		think::StatusDone
	}
	fn stop( &mut self )	{
		//TODO
	}
}