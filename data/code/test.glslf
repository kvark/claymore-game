#version 150 core

uniform	float u_Color = 0.5;
uniform	sampler2D t_Image;

const	vec4	c_Diffuse = vec4(1.0);
const 	vec4	c_Specular = vec4(1.0);
const	float	c_Shininess = 10.0;

in	vec2 texCoords;
in	vec3 normal, vecLight, vecHalf;

out	vec4 o_Color;


void main()	{
	vec3 N = normalize(normal);
	float kDiff = max(0.00, dot(N,normalize(vecLight)) );
	float kSpec = max(0.01, dot(N,normalize(vecHalf)) );
	vec4 albedo = texture( t_Image, texCoords );
	o_Color = albedo * c_Diffuse * kDiff +
		c_Specular * pow(kSpec,c_Shininess);
}