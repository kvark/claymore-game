#version 150 core

uniform sampler2D	t_Image;
uniform vec4		u_Center;

in	vec2 v_Coord;
out vec4 o_Color;


void main()	{
	vec2 tmp = step(u_Center.xy,v_Coord.xy) - v_Coord.xy;
	vec2 bc = u_Center.xy + u_Center.zw * tmp;
	o_Color =  texture( t_Image, bc );
}