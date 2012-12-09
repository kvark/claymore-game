//%meta initSurface computeLight getWorldNormal getColor

uniform sampler2D	t_DiffuseDirt, t_SpecBumpReflect;
uniform vec4		u_SpecularParams;

in vec3 v_Eye, v_NormalWorld;
in vec2 v_Tex;

const float	c_BumpFactor	= 10.0;
const vec3	c_ColorDiffuse	= vec3(1.0);
const vec3	c_ColorSpecular	= vec3(0.3);


vec3 getWorldNormal()	{
	return normalize( v_NormalWorld );
}

vec4 getColor()	{
	return vec4( texture(t_DiffuseDirt,v_Tex).xyz, 1.0 );
}

struct Context	{
	vec3 normal,eye;
	vec4 albedo;
	vec3 specBumpReflect;
}ct;


vec3 computeNormal(float height)	{
	const ivec3 off = ivec3(-1,0,1);
	float s01 = textureOffset(t_SpecBumpReflect, v_Tex, off.xy).y;
	float s21 = textureOffset(t_SpecBumpReflect, v_Tex, off.zy).y;
	float s10 = textureOffset(t_SpecBumpReflect, v_Tex, off.yx).y;
	float s12 = textureOffset(t_SpecBumpReflect, v_Tex, off.yz).y;
	return vec3( s01-s21, s12-s12, 2.0/c_BumpFactor );
}

vec4 initSurface()	{
	ct.specBumpReflect = texture(t_SpecBumpReflect,v_Tex).xyz;
	vec3 rawNormal = computeNormal( ct.specBumpReflect.y );
	//vec3 rawNormal = vec3(0.0,0.0,1.0);
	ct.normal = normalize(rawNormal);
	ct.eye = normalize(v_Eye);
	ct.albedo = texture(t_DiffuseDirt,v_Tex);
	return vec4(0.0,0.0,0.0,1.0);
}

vec3 computeLight(float ambient, float reflected, vec3 light)	{
	//return vec3(0.0);
	// Blinn-Phong model BRDF, normal mapped
	float diff = max( 0.0, dot(ct.normal,light) );
	vec3 halfVector = normalize(ct.eye+light);
	float spec = max( 0.01, dot(ct.normal,halfVector) );
	vec3 diffColor	= c_ColorDiffuse * (ambient + reflected*diff) * ct.albedo.rgb;
	vec3 specColor	= c_ColorSpecular * u_SpecularParams.x *
		reflected * ct.specBumpReflect.x * pow( spec, u_SpecularParams.y );
	return diffColor + specColor;
}
