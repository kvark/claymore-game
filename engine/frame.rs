extern mod glcore;
enum Handle	= glcore::GLuint;


pub struct Surface	{
	handle	: Handle,
	width	: uint,
	height	: uint,
	samples	: uint,
}


pub enum Target	{
	TarEmpty,
	TarSurface(@Surface),
	TarTexture(@texture::Texture,uint),
}

impl Target : cmp::Eq	{
	pure fn eq( other : &Target )-> bool	{
		match (&self,other)	{
			(&TarEmpty,&TarEmpty)					=> true,
			(&TarSurface(s1),&TarSurface(s2))		=> *s1.handle == *s2.handle,
			(&TarTexture(t1,l1),&TarTexture(t2,l2))	=> *t1.handle == *t2.handle && l1==l2,
			(_,_) => false
		}
	}
	pure fn ne( other : &Target)-> bool	{
		!self.eq( other )
	}
}


priv struct TargetBlock	{
	depth	: Target,
	stencil	: Target,
	color0	: Target,
	color1	: Target,
	color2	: Target,
	color3	: Target,
}


impl TargetBlock	{
	pure fn check()	{
		let mut wid = 0u, het = 0u, sam = 0u;
		for [self.depth,self.stencil,self.color0,self.color1,self.color2,self.color3].each |tar|	{
			match tar	{
				&TarEmpty => {},
				&TarSurface(sf) => 	{
					if wid==0u	{ wid=sf.width; het=sf.height; sam=sf.samples; }
					else	{ assert wid==sf.width && het==sf.height && sam==sf.samples };
				},
				&TarTexture(tex,lev) =>	{
					let (w,h) = tex.get_level_size(lev);
					if wid==0u	{ wid=w; het=h; sam=tex.samples; }
					else	{ assert wid==w && het==h && sam==tex.samples; }
				}
			}
		}
	}
}


struct Buffer	{
	handle	: Handle,
	//viewport,
	mut 		at	: TargetBlock,
	priv mut	bt	: TargetBlock,
}