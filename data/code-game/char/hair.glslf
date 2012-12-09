//%meta initSurface computeLight getWorldNormal getColor

uniform sampler2D	t_Normal, t_SpecAlphaRefl;
uniform	vec4		u_SpecularParams;

in vec3 v_Eye, v_NormalWorld;
in vec2 v_Tex;

const vec3	c_ColorDiffuse	= vec3(0.1);
const vec3	c_ColorSpecular	= vec3(1.0);


vec3 getWorldNormal()	{
	return normalize( v_NormalWorld );
}

vec4 getColor()	{
	return vec4(0.0);
}

struct Context	{
	vec3 normal,eye;
	vec3 specAlphaRefl;
}ct;


vec4 initSurface()	{
	vec3 rawNormal = texture(t_Normal,v_Tex).xyz * 2.0 - 1.0;
	ct.normal = normalize(rawNormal);
	ct.eye = normalize(v_Eye);
	ct.specAlphaRefl = texture(t_SpecAlphaRefl,v_Tex).xyz;
	return vec4( vec3(0.0), ct.specAlphaRefl.z );
}

vec3 computeLight(float ambient, float reflected, vec3 light)	{
	// Blinn-Phong model BRDF, normal mapped
    float diff = max( 0.0, dot(ct.normal,light) );
    vec3 halfVector = normalize(ct.eye+light);
    float spec = max( 0.01, dot(ct.normal,halfVector) );
    vec3 diffColor	= c_ColorDiffuse * (ambient + reflected*diff);
    vec3 specColor	= c_ColorSpecular * u_SpecularParams.x *
    	reflected * pow(spec,u_SpecularParams.y);
    return diffColor + specColor;
}
