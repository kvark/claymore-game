//%meta initSurface computeLight

const float	c_TechAmbient	= 0.1;
const float c_TechReflect	= 1.0;

in vec3 v_Light;

out	vec4 o_Color;

void main()	{
	vec3 vl = normalize( v_Light );
	o_Color = initSurface();
	o_Color.rgb += computeLight( c_TechAmbient, c_TechReflect, vl );
}
