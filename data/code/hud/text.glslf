#version 150 core

uniform sampler2D	t_Text;
uniform vec4		u_Color;

in	vec2 v_TexCoord;
out vec4 o_Color;

void main()	{
	o_Color = texture( t_Text, v_TexCoord ).x * u_Color;
}