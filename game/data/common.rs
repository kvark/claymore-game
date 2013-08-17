type Name = ~str;
type Path = ~str;
type UintRange = [uint, ..2];
type Scalar = f32;
type Vector2 = [Scalar, ..2];
type Vector3 = [Scalar, ..3];
type Vector4 = [Scalar, ..4];
type Quaternion = [Scalar, ..4];
type Color	= Vector4;	//uint

pub struct Global	{
	gravity		: Vector3,
}

pub enum MaterialKind	{
	KindFlat,
	KindPhong,
	KindAnisotropic,
}

pub enum MaterialData	{
	DataInt(~str,int),
	DataScalar(~str,Scalar),
	DataVector(~str,Vector4),
	DataColor(~str,Vector3),
}

pub struct Texture	{
	name	: Name,
	path	: Path,
	filter	: uint,
	wrap	: int,
	scale	: Vector2,
	offset	: Vector2,
}

pub struct Material	{
	name		: Name,
	kind		: MaterialKind,
	data		: ~[MaterialData],
	textures	: ~[Texture],
}

pub struct Entity	{
	material	: Name,
	mesh		: Path,
	range		: UintRange,
	armature	: Path,
}

pub struct Camera	{
	name		: Name,
	fov_y		: Scalar,
	range		: Vector2,
}

pub struct Omni;

pub struct Spot	{
	size	: Scalar,
	blend	: Scalar,
}

pub enum LightKind	{
	KindOmni(Omni),
	KindSpot(Spot),
}

pub struct Light	{
	kind		: LightKind,
	color		: Color,
	distance	: Scalar,
	energy		: Scalar,
	attenuation	: Vector2,
	spherical	: bool,
}

pub struct QuatSpace	{
	pos		: Vector3,
	rot		: Quaternion,
	scale	: Scalar,
}

pub enum NodeChild	{
	ChildNode(Node),
	ChildEntity(Entity),
	ChildCamera(Camera),
	ChildLight(Light),
}

pub struct Node	{
	name		: Name,
	space		: QuatSpace,
	children	: ~[NodeChild],
}

pub struct Scene	{
	global		: Global,
	materials	: ~[Material],
	nodes		: ~[NodeChild],
}
