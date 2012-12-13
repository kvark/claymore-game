#version 150 core

uniform sampler2D	t_Environment;

in	vec4	v_TexCoords;
out	vec4	o_Color;

void main()	{
	vec2 tc = normalize(v_TexCoords.xyz).zy*0.5+0.5;
	o_Color = texture(t_Environment,tc);
}