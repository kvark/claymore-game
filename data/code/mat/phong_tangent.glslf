//%meta initSurface computeLight getWorldNormal getColor

uniform sampler2D	t_Main, t_Normal;

in vec3 v_Eye, v_NormalWorld;
in vec2 v_Tex;

const float	c_Shininess		= 10.0;
const vec3	c_ColorDiffuse	= vec3(1.0);
const vec3	c_ColorSpecular	= vec3(0.3);


vec3 getWorldNormal()	{
	return normalize( v_NormalWorld );
}

vec4 getColor()	{
	return texture(t_Main,v_Tex);
}

struct Context	{
	vec3 normal,eye;
	vec4 albedo;
}ct;


bool initAlpha()	{
	return getColor().a > 0.1;
}

vec4 initSurface()	{
	vec3 rawNormal = texture(t_Normal,v_Tex).xyz * 2.0 - 1.0;
	ct.normal = normalize(rawNormal);
	ct.eye = normalize(v_Eye);
	ct.albedo = getColor();
	return vec4( vec3(0.0), ct.albedo.a );
}

vec3 computeLight(float ambient, float reflected, vec3 light)	{
	// Blinn-Phong model BRDF, normal mapped
    float diff = max( 0.0, dot(ct.normal,light) );
    vec3 halfVector = normalize(ct.eye+light);
    float spec = max( 0.01, dot(ct.normal,halfVector) );
    vec3 diffColor	= c_ColorDiffuse * (ambient + reflected*diff) * ct.albedo.rgb;
    vec3 specColor	= c_ColorSpecular * reflected * pow(spec,c_Shininess);
    return diffColor + specColor;
}
