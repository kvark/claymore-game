//%meta initSurface computeLight getWorldNormal getColor

uniform sampler2D	t_Normal, t_SpecAlphaRefl;
uniform	vec4		u_SpecularParams;

in vec3 v_Eye, v_NormalWorld;
in vec2 v_Tex;

const vec3	c_ColorDiffuse	= vec3(0.1);
const vec3	c_ColorSpecular	= vec3(1.0);
const vec2	c_Anisotropic	= vec2(1.0,1.0);


vec3 getWorldNormal()	{
	return normalize( v_NormalWorld );
}

vec4 getColor()	{
	return vec4(0.0);
}

struct Context	{
	vec3 normal,eye;
	float specular;
}ct;


vec4 initSurface()	{
	vec3 rawNormal = texture(t_Normal,v_Tex).xyz * 2.0 - 1.0;
	ct.normal = normalize(rawNormal);
	ct.eye = normalize(v_Eye);
	vec4 params = texture(t_SpecAlphaRefl,v_Tex);
	ct.specular = params.x;
	return vec4( vec3(0.0), params.y );
}

//Brushed metal (anisotropic shader)
//http://en.wikibooks.org/wiki/GLSL_Programming/Unity/Brushed_Metal

vec3 computeLight(float ambient, float reflected, vec3 light)	{
	// Phong anisotropic BRDF, normal mapped
	vec3 tan = vec3(1.0,0.0,0.0);
	float dotLN = dot( light, ct.normal );
	if (dotLN>0.0)	{
		vec3 H = normalize(light + ct.eye);
		float dotHN = dot( H, ct.normal );
		float dotVN = dot( ct.eye, ct.normal );
		vec2 dotA = H.xy / c_Anisotropic;
		float spec = sqrt(max(0.0,dotLN/dotVN)) * 
			exp(-2.0*dot(dotA,dotA)/(1.0+dotHN));
		return c_ColorDiffuse*(ambient + reflected*dotLN) +
			c_ColorSpecular*u_SpecularParams.x*reflected*ct.specular*spec;
	}else	{
		return c_ColorDiffuse * ambient;
	}
}
