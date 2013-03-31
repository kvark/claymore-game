//%meta initSurface computeLight

/*
uniform sampler2DArray	t_LbufDir;
uniform sampler2DArray	t_LbufCol;

uniform vec4	u_TargetSize;

const float	c_TechAmbient	= 0.1;
const float c_TechReflect	= 1.0;

out	vec4 o_Color;


void main()	{
	ivec3 lbufSize = textureSize( t_LbufDir, 0 );
	vec2 tc = gl_FragCoord.xy * u_TargetSize.zw;
	o_Color = initSurface();
	for (int i=0; i<lbufSize.z; ++i)	{
		vec4 dir = texture( t_LbufDir, vec3(tc,i+0.5) );
		float reflected = length(dir);
		vec4 col = texture( t_LbufCol, vec3(tc,i+0.5) );
		o_Color.rgb += (col.rgb/col.w) * computeLight(
			dir.w-reflected, reflected, dir.xyz/reflected );
	}
}
*/

uniform sampler2D	t_LbufDir;
uniform sampler2D	t_LbufCol;

uniform vec4	u_TargetSize;

const float	c_TechAmbient	= 0.1;
const float c_TechReflect	= 1.0;

out	vec4 o_Color;


void main()	{
	vec2 tc = gl_FragCoord.xy * u_TargetSize.zw;
	o_Color = initSurface();
	vec4 dir = texture( t_LbufDir, tc );
	float reflected = length(dir);
	vec4 col = texture( t_LbufCol, tc );
	o_Color.rgb += (col.rgb/col.w) * computeLight(
		dir.w-reflected, reflected, dir.xyz/reflected );
}
