//%meta initSurface computeLight

const float	c_Ambient	= 0.1;
const float c_Reflect	= 1.0;

in vec3 v_Light;

out	vec4 o_Color;

void main()	{
	vec3 vl = normalize( v_Light );
	o_Color = initSurface();
	o_Color.rgb += computeLight( c_Ambient, c_Reflect, vl );
}
