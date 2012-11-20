//%meta initSurface computeLight

in vec3 v_Light;

out	vec4 o_Color;

void main()	{
	vec3 vl = normalize( v_Light );
	o_Color = initSurface();
	o_Color.rgb += computeLight( 0.1, 1.0, vl );
}
