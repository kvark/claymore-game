//%meta getFinalColor

in	vec3 v_Normal;

out	vec4 o_Color;

void main()	{
	vec3 n = normalize(v_Normal);
	o_Color = getFinalColor(n);
}
