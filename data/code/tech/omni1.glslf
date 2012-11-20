//%meta initSurface computeLight

in vec3 v_Light;

out	vec4 o_Color;

void main()	{
	o_Color = initSurface();
	vec3 vl = normalize( v_Light );
	o_Color.rgb += computeLight( 0.1, 1.0, vl );
}
