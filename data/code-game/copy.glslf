#version 150 core

uniform sampler2D	t_Image;

in	vec2 v_TexCoords;
out vec4 o_Color;


void main()	{
	o_Color = texture( t_Image, v_TexCoords );
}