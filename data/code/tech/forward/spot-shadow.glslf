//%meta initSurface computeLight

uniform	sampler2DShadow	t_Shadow;
uniform	vec4			u_LightColor;
uniform vec4			u_LightAttenuation;

const float	c_TechAmbient	= 0.1;
const float c_TechReflect	= 1.0;
const float c_SpotBlend		= 0.15;

in	vec3 v_LightMaterial, v_LightWorld;
in	vec4 v_LightShadow;

out	vec4 o_Color;


float getAttenuation(float d)	{
	vec3 k = 1.0 + vec3(d,d*d,-d)*u_LightAttenuation.yzw;
	return u_LightAttenuation.x * max(0.0,k.z) / (k.x*k.y);
}


void main()	{
	float radLength = dot(v_LightShadow.xy,v_LightShadow.xy) / (v_LightShadow.w*v_LightShadow.w);
	float sideShadow = 1.0-smoothstep( (1.0-c_SpotBlend)*(1.0-c_SpotBlend), 1.0, radLength );
	float shadow = textureProj( t_Shadow, 0.5*(v_LightShadow + v_LightShadow.wwww) );
	o_Color = initSurface();
	float dist = length( v_LightWorld );
	float attenu = getAttenuation(dist);
	vec3 vl = normalize(v_LightMaterial);
	float reflect = c_TechReflect * sideShadow * shadow * attenu;
	o_Color.rgb += u_LightColor.xyz * computeLight( c_TechAmbient, reflect, vl );
}
