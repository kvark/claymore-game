//%meta initSurface computeLight

uniform	sampler2DShadow	t_Shadow;


const float	c_TechAmbient	= 0.1;
const float c_TechReflect	= 1.0;

in	vec3 v_LightMaterial;
in	vec4 v_LightShadow;

out	vec4 o_Color;


void main()	{
	float shadow = textureProj(t_Shadow,v_LightShadow);
	o_Color = initSurface();
	vec3 vl = normalize( v_LightMaterial );
	float reflect = c_TechReflect * shadow;
	o_Color.rgb += computeLight( c_TechAmbient, reflect, vl );
}
