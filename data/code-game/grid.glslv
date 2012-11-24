#version 150 core

uniform mat4	u_ViewProj;
const vec2	c_Scale = vec2(10.0,10.0);
const float	c_ZLevel = 1.0;

in	vec2 a_Vertex;


void main()	{
	vec4 pos = vec4(c_Scale * a_Vertex,c_ZLevel,1.0);
	gl_Position = u_ViewProj * pos;
}