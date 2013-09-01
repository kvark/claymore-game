type Name = ~str;
type Path = ~str;
type UintRange = [uint, ..2];
type Scalar = f32;
type Vector2 = [Scalar, ..2];
type Vector3 = [Scalar, ..3];
type Vector4 = [Scalar, ..4];
type Quaternion = [Scalar, ..4];
type Color	= Vector3;

pub struct Global	{
	gravity		: Vector3,
}

pub enum MaterialData	{
	DataInt(int),
	DataScalar(Scalar),
	DataVector(Vector4),
	DataColor(Color),
}

pub struct Texture	{
	name	: Name,
	path	: Path,
	filter	: uint,
	wrap	: int,
	scale	: Vector3,
	offset	: Vector3,
}

pub struct Material	{
	name		: Name,
	shader		: Path,
	data		: ~[(~str,MaterialData)],
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
	name		: Name,
	kind		: LightKind,
	color		: Color,
	energy		: Scalar,
	attenuation	: Vector2,
	distance	: Scalar,
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
