//%meta initSurface computeLight getWorldNormal getColor

uniform sampler2D	t_DiffuseDirt, t_SpecBumpReflect, t_Reflection;
uniform vec4		u_SpecularParams;

in vec3 v_Eye, v_NormalWorld;
in vec2 v_Tex;
in mat3 TBN;

const float	c_BumpFactor	= 2.0;
const float c_ReflectFactor	= 0.1;
const vec3	c_ColorDiffuse	= vec3(1.0);
const vec3	c_ColorSpecular	= vec3(1.0);

vec3 getWorldNormal()	{
	return normalize( v_NormalWorld );
}

vec4 getColor()	{
	return texture(t_DiffuseDirt,v_Tex);
}

bool initAlpha()	{
	return true;
}


struct Context	{
	vec3 normal,eye;
	vec4 albedo;
	float specular;
}ct;

vec3 computeNormal(float height)	{
	const ivec3 off = ivec3(-1,0,1);
	float hx0 = textureOffset( t_SpecBumpReflect, v_Tex, off.xy ).y;
	float hx1 = textureOffset( t_SpecBumpReflect, v_Tex, off.zy ).y;
	float hy0 = textureOffset( t_SpecBumpReflect, v_Tex, off.yx ).y;
	float hy1 = textureOffset( t_SpecBumpReflect, v_Tex, off.yz ).y;
	return vec3( hx0-hx1, hy0-hy1, 2.0/c_BumpFactor );
}

vec2 envirCoords(vec3 vOrig)	{
	const float PI = 3.1415926;
	vec3 R = normalize(vOrig);
	float u = (atan(R.x/R.z) + PI)/(2.0*PI);
	float v = (asin(R.y)+0.5*PI)/PI;
	return vec2(u,1.0-v);
}

vec4 initSurface()	{
	vec4 param = texture(t_SpecBumpReflect,v_Tex);
	vec3 rawNormal = computeNormal( param.y );
	ct.normal = normalize(rawNormal);
	ct.eye = normalize(v_Eye);
	vec3 refl_tbn = reflect( -ct.eye, ct.normal );
	vec2 refl_tc = envirCoords( TBN*refl_tbn );
	vec4 refl_color = texture(t_Reflection,refl_tc);
	vec3 reflected = c_ReflectFactor*param.z*refl_color.xyz;
	ct.albedo = texture(t_DiffuseDirt,v_Tex);
	ct.specular = param.x;
	return vec4( reflected, 1.0 );
}

vec3 computeLight(float ambient, float reflected, vec3 light)	{
	//return vec3(0.0);
	// Blinn-Phong model BRDF, normal mapped
    float diff = max( 0.0, dot(ct.normal,light) );
    vec3 halfVector = normalize(ct.eye+light);
    float spec = max( 0.01, dot(ct.normal,halfVector) );
    vec3 diffColor	= c_ColorDiffuse * (ambient + reflected*diff) * ct.albedo.rgb;
    vec3 specColor	= c_ColorSpecular * reflected *
    	u_SpecularParams.x * pow(spec,u_SpecularParams.y);
    return diffColor + specColor;
}
