//%meta initSurface computeLight

uniform sampler2DArray	t_Dir;
uniform sampler2DArray	t_Col;
uniform vec4	u_TargetSize;

const float	c_TechAmbient	= 0.1;
const float c_TechReflect	= 1.0;

out	vec4 o_Color;


void main()	{
	ivec3 lbufSize = textureSize( t_Dir, 0 );
	vec2 tc = gl_FragCoord.xy * u_TargetSize.zw;
	o_Color = initSurface();
	for (int i=0; i<lbufSize.z; ++i)	{
		vec4 dir = texture( t_Dir, vec3(tc,i+0.5) );
		float reflected = length(dir);
		vec4 col = texture( t_Col, vec3(tc,i+0.5) );
		o_Color.rgb += (col.rgb/col.w) * computeLight(
			dir.w-reflected, reflected, dir/reflected );
	}
}
