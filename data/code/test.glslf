#version 150 core

uniform	float u_Color = 0.5;
uniform	sampler2D t_Image;

in	vec2 texCoords;
out	vec4 result;


void main()	{
	result = texture( t_Image, texCoords );
}