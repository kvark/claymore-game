extern mod engine;

use cgmath::vector::{Vec2};

use battle::grid;


pub type Health		= uint;
#[deriving(Clone,Eq)]

pub enum LimbType	{
	LimbBody,
	LimbHead,
	LimbArm,
	LimbLeg,
	LimbTail,
}

impl ToStr for LimbType	{
	fn to_str( &self )-> ~str	{
		match self	{
			&LimbBody	=> ~"Body",
			&LimbHead	=> ~"Head",
			&LimbArm	=> ~"Arm",
			&LimbLeg	=> ~"Leg",
			&LimbTail	=> ~"Tail",			
		}
	}
}

pub type LimbId		= u8;
#[deriving(Clone,Eq)]
pub type LimbKey	= (LimbType,LimbId);
pub type LimbOffset = Vec2<int>;

#[deriving(Clone)]
pub struct Limb {
	health	: Health,
	node	: engine::space::NodePtr,
}


pub enum EffectResult	{
	ResultNone,
	ResultDamage,
	ResultLimbCut,
	ResultDeath,
}

pub enum Effect	{
	EfDamage( LimbKey, Health ),
}

impl ToStr for Effect	{
	fn to_str( &self )-> ~str	{
		match self	{
			&EfDamage(lkey,damage)	=>	{
				let (lt,id) = lkey;
				format!( "Damage({:u} -> {:s}[{:u}])", damage, lt.to_str(), id )
			}
		}
	}
}


pub trait Unit	{
	fn get_name<'a>( &'a self )-> &'a str;
	fn get_limb<'a>( &'a self, LimbKey )-> Option<&'a Limb>;
	fn each_limb( &self, fun: |LimbKey,&Limb,grid::Offset| );
	fn apply_effect( &mut self, Effect )-> EffectResult;
}

pub struct Standard {
	// info
	name		: ~str,
	body		: Limb,
	// stats
	move_speed	: f32,
	turn_speed	: f32,
	// view
	entity		: engine::object::Entity,
	skeleton	: engine::space::ArmaturePtr,
	record		: @engine::space::ArmatureRecord,
	elevation	: f32,
}

impl Unit for Standard {
	fn get_name<'a>( &'a self )-> &'a str	{ self.name.as_slice() }

	fn get_limb<'a>( &'a self, lkey: LimbKey )-> Option<&'a Limb> {
		match lkey   {
			(LimbBody,0)	=> Some(&self.body),
			_				=> None
		}
	}

	fn each_limb( &self, fun: |LimbKey,&Limb,LimbOffset| )	{
		fun( (LimbBody,0), &self.body, Vec2::new(0,0) );
	}

	fn apply_effect( &mut self, effect: Effect )-> EffectResult	{
		match effect	{
			EfDamage(_,damage)	=> {
				if self.body.health > damage	{
					self.body.health -= damage;
					ResultDamage
				}else	{
					self.body.health = 0;
					ResultDeath
				}
			},
			//_	=> fail!("Unknown effect: {:s}", effect.to_str())
		}
	}
}

