//%meta initSurface computeLight getWorldNormal getColor

uniform vec4		u_SpecularParams;

in	vec3 v_Normal, v_Eye;
in	vec2 v_Tex;

const vec3	c_ColorDiffuse	= vec3(1.0);
const vec3	c_ColorSpecular	= vec3(1.0);
const vec2	c_Anisotropic	= vec2(1.0,1.0);


vec3 getWorldNormal()	{
	return normalize(v_Normal);
}

vec4 getColor()	{
	return vec4(c_ColorDiffuse,1.0);
}

bool initAlpha()	{
	return true;
}


struct Context	{
	vec3 normal,eye;
	vec4 albedo;
}ct;

vec4 initSurface()	{
	ct.normal = getWorldNormal();
	ct.eye = normalize(v_Eye);
	ct.albedo = getColor();
	return vec4( vec3(0.0), ct.albedo.a );
}

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
			c_ColorSpecular*u_SpecularParams.x*reflected*spec;
	}else	{
		return c_ColorDiffuse * ambient;
	}
}

