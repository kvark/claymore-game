//%meta getIntensity getColor

uniform	sampler2D	t_Depth;
uniform	mat4		u_ViewInverse;
uniform vec4		u_TargetSize;
uniform	vec4		u_LightPos;

const	float	c_TechAmbient	= 0.1;
const	float	c_TechReflect	= 1.0;

out	vec4	o_LbufDir;
out	vec4	o_LbufCol;


void main()	{
	vec2 tc = gl_FragCoord.xy * u_TargetSize.zw;
	float depth = texture( t_Depth, tc ).x;
	vec4 ndc = vec4(tc,depth,1.0)*2.0 - 1.0;
	vec3 wp = (u_ViewInverse * ndc).xyz;
	vec3 dir = wp - u_LightPos.xyz;
	float intensity = getIntensity(dir);
	o_LbufDir = intensity * vec4( normalize(dir), 1.0 );
	o_LbufCol = intensity * getColor();
}
