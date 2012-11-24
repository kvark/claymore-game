#version 150 core

uniform mat4	u_ViewProj;
const vec2	c_Scale = vec2(10.0,10.0);
const float	c_ZLevel = 1.0;

in	vec2 a_Vertex;
out	vec2 v_TexCoords;


void main()	{
	v_TexCoords = a_Vertex;
	vec4 pos = vec4( c_Scale * (2.0*a_Vertex-1.0), c_ZLevel, 1.0 );
	gl_Position = u_ViewProj * pos;
}