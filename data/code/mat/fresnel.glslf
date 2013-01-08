//%meta initSurface computeLight getWorldNormal getColor getFresnel

uniform vec4 		u_Color;
uniform sampler2D	t_Main, t_Reflect;

in vec2 v_Tex;
in vec3 v_Normal, v_Eye;

const vec3	c_ColorReflect	= vec3(1.0,0.0,0.0);
const vec3	c_ColorFront	= vec3(0.0,1.0,0.0);
const vec3	c_ColorBack		= vec3(0.0,0.0,1.0);
const vec4	c_FresnelParams	= vec4(0.2,2.0,0.0,0.0);	//bias,power


float getFresnel(float NdV)	{
	// http://http.developer.nvidia.com/GPUGems2/gpugems2_chapter19.html
	return max( 0.0, (1.0-c_FresnelParams.x) * pow(1.0-NdV,c_FresnelParams.y) );
}

vec4 getColor()	{
	return texture(t_Main,v_Tex);
}

vec3 getWorldNormal()	{
	return normalize(v_Normal);
}

struct Context	{
	vec3 normal,eye;
	vec4 albedo;
}ct;


bool initAlpha()	{
	return getColor().a > 0.1;
}

vec4 initSurface()	{
	ct.normal = getWorldNormal();
	ct.eye = normalize(v_Eye);
	ct.albedo = getColor();
	float NdV = max( 0.0, dot( ct.normal, normalize(ct.eye) ));
	float fresnel = getFresnel(NdV);
	vec3 diffuse = mix( c_ColorFront, c_ColorBack*NdV, NdV );
	vec3 color = diffuse*ct.albedo.xyz + fresnel*c_ColorReflect;
	return vec4( color , ct.albedo.a );
}

vec3 computeLight(float, float, vec3)	{
	return vec3(0.0);
}