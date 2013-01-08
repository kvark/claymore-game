//%meta initSurface computeLight

uniform	sampler2DShadow	t_Shadow;


const float	c_Ambient	= 0.1;
const float c_Reflect	= 1.0;

in	vec3 v_LightMaterial;
in	vec4 v_LightShadow;

out	vec4 o_Color;


void main()	{
	float shadow = textureProj(t_Shadow,v_LightShadow);
	o_Color = initSurface();
	vec3 vl = normalize( v_LightMaterial );
	float reflect = c_Reflect * shadow;
	o_Color.rgb += computeLight( c_Ambient, reflect, vl );
}
