#version 150 core

uniform sampler2D	t_Environment;

in	vec4	v_TexCoords;
out	vec4	o_Color;

const float PI = 3.1415926;
//Blinn/Newell Latitude Mapping
//http://www.reindelsoftware.com/Documents/Mapping/Mapping.html

vec2 envir_coords(vec3 vOrig)	{
	vec3 R = normalize(vOrig);
	float u = (atan(R.x/R.z) + PI)/(2.0*PI);
	float v = (asin(R.y)+0.5*PI)/PI;
	return vec2(u,1.0-v);
}

void main()	{
	vec2 tc = envir_coords(v_TexCoords.xyz);
	o_Color = texture(t_Environment,tc);
}