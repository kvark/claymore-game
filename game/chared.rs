extern mod engine;
extern mod lmath;
extern mod numeric;

use lmath::quat::*;
use lmath::vec::vec3::*;


struct Envir	{
	input	: engine::call::DrawInput,
	prog	: @engine::shade::Program,
	mut data: engine::shade::DataMap,
	rast	: engine::rast::State,
}

struct CamControl	{
	node		: @engine::space::Node,
	origin		: Vec3<f32>,
	speed_rot	: f32,
	speed_zoom	: f32,
	mut last_scroll	: Option<float>,
}

pub fn clamp<T:cmp::Ord>( x:T, a:T, b:T )-> T	{
	if x>a {
		if x<b	{x}
		else	{b}
	}else {a}
}

impl CamControl	{
	fn update( dt : float, nx : float, _ny : float, hit : bool, scroll : float )	{
		// calculate rotation
		if hit	{
			let axis = Vec3::new( 0f32, 0f32, if nx>0.5f {1f32} else {-1f32} );
			let angle = numeric::types::Radians( dt as f32 * self.speed_rot );
			let qr = Quat::from_axis_angle( &axis, angle );
			let sq = engine::space::QuatSpace{
				position : Vec3::new(0f32,0f32,0f32),
				orientation : qr, scale : 1f32 };
			*self.node.mut_space() = sq * self.node.space;
		}
		// calculate scroll
		if scroll != 0f	{
			let v_origin = self.origin.sub_v( &self.node.space.position );
			let dist_min = 20f32;
			let dist_max = 100f32;
			let dist = v_origin.length();
			let dist_raw = dist - (scroll as f32) * self.speed_zoom;
			let dist_diff = clamp( dist_raw, dist_min, dist_max ) - dist;
			let p = self.node.space.position.sub_v( &v_origin.mul_t(dist_diff/dist) );
			self.node.mut_space().position = p;
		}
	}
}


pub struct Scene	{
	gr_main	: scene::EntityGroup,
	gr_hair	: scene::EntityGroup,
	skel	: @engine::space::Armature,
	cam		: scene::Camera,
	control	: CamControl,
	envir	: Envir,
	tech_solid	: engine::draw::Technique,
	tech_alpha	: engine::draw::Technique,
	start	: float,
	hud_screen	: hud::Screen,
	hud_context	: hud::Context,
	hud_debug	: @engine::shade::Program,
}


impl Scene	{
	fn update( nx : float, ny : float, hit : bool, scroll : float, lg : &engine::context::Log )-> bool	{
		if hit	{
			let root = &self.hud_screen.root;
			let (mx,my) = root.min_size;
			let x = ((0f+nx) * (mx as float)) as int;
			let y = ((1f-ny) * (my as float)) as int;
			let name = root.trace( x, y, lg );
			io::println( ~"Click: " + name );
		}
		let dt = 1f/30f;
		self.control.update( dt, nx, ny, hit, scroll );
		let lit_pos	= lmath::gltypes::vec4::new( 3f32, 3f32, 3f32, 0f32 );
		for self.gr_main.each() |ent|	{
			ent.update_world();
			let gd = ent.mut_data();
			gd.insert( ~"u_LightPos",	engine::shade::UniFloatVec(lit_pos) );
			self.cam.fill_data( gd );
			//self.skel.fill_data( gd );
		}
		for self.gr_hair.each() |ent|	{
			ent.update_world();
			let gd = ent.mut_data();
			gd.insert( ~"u_LightPos",	engine::shade::UniFloatVec(lit_pos) );
			self.cam.fill_data( gd );
			//self.skel.fill_data( gd );
		}
		let vpi = self.cam.get_matrix().invert();
		//self.cam.fill_data( &mut self.envir.data );
		self.envir.data.insert( ~"u_ViewProjInverse",
			engine::shade::UniMatrix(false,vpi) );
		true
	}
	fn render( el : &main::Elements, ct : &engine::context::Context, lg : &engine::context::Log  )	{
		// clear screen
		let c0 = self.tech_solid.gen_clear(
			engine::call::ClearData{
				color	:Some( engine::rast::make_color(0xFFFFFFFF) ),
				depth	:Some( 1f ),
				stencil	:Some( 0u ),
			}
		);
		let mut queue = ~[c0];
		if el.environment	{
			queue.push({
				let e = &self.envir;
				let tech = &self.tech_solid;
				engine::call::CallDraw(
					copy e.input, copy tech.output,
					e.prog, copy e.data )
			});
		}
		if true	{	// update animation
			let t = engine::anim::get_time() - self.start;
			let r = self.skel.actions[0];
			let nloops = (t / r.duration) as uint;
			let t2 = t - r.duration * (nloops as float);
			self.skel.set_record( r, t2 );
			//self.skel.fill_data( self.girl.mut_data() );
		}
		if el.character	{
			for self.gr_main.each() |ent|	{
				queue.push( self.tech_solid.process( ent, ct, lg )
					);
			}
			for self.gr_hair.each() |ent|	{
				queue.push( self.tech_alpha.process( ent, ct, lg )
					);
			}
		}
		if el.hud	{
			queue.push_all_move(
				self.hud_screen.root.draw_all( &self.hud_context )
				);
		}
		if el.hud_debug	{
			queue.push_all_move({
				let mut rast  = engine::rast::make_rast(0,0);
				rast.prime.poly_mode = engine::rast::map_polygon_fill(2);
				let mut data = engine::shade::make_data();
				let vc = lmath::gltypes::vec4::new(1f32,0f32,0f32,1f32);
				data.insert( ~"u_Color", engine::shade::UniFloatVec(vc) );
				self.hud_screen.root.draw_debug( &self.hud_context,
					self.hud_debug, &mut data, &rast )
			});
		}
		ct.flush(queue);
	}
}

pub fn make_scene( ct : &engine::context::Context, aspect : float, lg : &engine::context::Log )-> Scene	{
	let vao = @ct.create_vertex_array();
	let scene = scene::load_scene( ~"data/claymore-2", ct, Some(vao), aspect, lg );
	let (t_solid,t_alpha) = {
		let pmap = engine::call::make_plane_map( ~"o_Color", engine::frame::TarEmpty );
		let mut rast = engine::rast::make_rast(0,0);
		rast.depth.test = true;
		//rast.prime.cull = true;	//the cloak is 2-sided
		let cache = @mut engine::draw::create_cache();
		let t1 = engine::draw::load_technique( ~"data/code/tech/forward/light",
			(ct.default_frame_buffer, copy pmap, copy rast), cache);
		rast.prime.cull = true;
		rast.set_blend( ~"s+d", ~"Sa", ~"1-Sa" );
		let t2 = engine::draw::load_technique( ~"data/code/tech/forward/light",
			(ct.default_frame_buffer, pmap, rast), cache);
		(t1,t2)
	};
	let arm = scene.armatures.get(&~"Armature.002");
	let cam = scene.cameras.get(&~"Camera");
	//cam.test();
	let mut group = scene::divide_group( &mut scene.entities, &~"noTrasnform" );
	let hair = scene::divide_group( &mut group, &~"Hair_Geo2" );
	lg.add(fmt!( "Group size: %u", group.len() ));
	lg.add(fmt!( "Camera fov:%f, aspect:%f, range:%f-%f",
		*cam.proj.vfov as float,
		cam.proj.aspect as float,
		cam.proj.near as float,
		cam.proj.far as float ));
	lg.add( ~"\tWorld :" + cam.node.world_space().to_string() );
	let envir = {
		let mesh = @engine::mesh::create_quad( ct );
		let prog = @engine::load::load_program( ct, ~"data/code-game/envir", lg );
		let tex = scene.textures.get( &~"data/texture/Topanga_Forest_B_3k.hdr" );
		let samp = engine::texture::make_sampler(3u,1);
		let mut data = engine::shade::make_data();
		data.insert( ~"t_Environment",		engine::shade::UniTexture(0,tex,Some(samp)) );
		let mut rast = engine::rast::make_rast(0,0);
		rast.set_depth( ~"<=", false );
		Envir{
			input:(vao,mesh,mesh.get_range()),
			prog:prog,
			data:data,
			rast:rast,
		}		
	};
	// load char HUD
	let fcon = @engine::font::create_context();
	let hud_screen = hud::load_screen( ~"data/hud/char.json", ct, fcon, lg );
	hud_screen.root.update( lg );
	let hc = {
		let mut hud_rast = engine::rast::make_rast(0,0);
		hud_rast.set_blend( ~"s+d", ~"Sa", ~"1-Sa" );
		let quad = @engine::mesh::create_quad(ct);
		let &(_,pmap,_) = &t_solid.output;
		hud::Context{
			input	: (vao,quad,quad.get_range()),
			output	: (ct.default_frame_buffer, copy pmap, hud_rast),
			size	: ct.screen_size,
		}
	};
	let hdebug = @engine::load::load_program( ct, ~"data/code/hud/debug", lg );
	let control = CamControl{
		node	:cam.node,
		origin	:Vec3::new(0f32,0f32,78f32),
		speed_rot	:1.5f32,
		speed_zoom	:15f32,
		last_scroll	: None,
	};
	//arm.set_record( arm.actions[0], 0f );
	Scene	{
		gr_main	: group,
		gr_hair	: hair,
		skel	: arm,
		cam		: cam,
		control	: control,
		envir	: envir,
		tech_solid	: t_solid,
		tech_alpha	: t_alpha,
		start	: engine::anim::get_time(),
		hud_screen	: hud_screen,
		hud_context : hc,
		hud_debug	: hdebug,
	}
}
