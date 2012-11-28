#version 150 core

uniform sampler2DRect	t_Text;

in	vec2 v_TexCoord;
out vec4 o_Color;

void main()	{
	o_Color = texture( t_Text, v_TexCoord );
}