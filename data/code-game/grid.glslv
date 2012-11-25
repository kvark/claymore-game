#version 150 core

uniform mat4	u_ViewProj;
uniform vec4	u_ScaleZ;
//const vec2	c_Scale = vec2(10.0,10.0);
//const float	c_ZLevel = 1.0;

in	vec2 a_Vertex;
out	vec2 v_TexCoords;


void main()	{
	v_TexCoords = a_Vertex;
	vec4 pos = vec4( u_ScaleZ.xy * (2.0*a_Vertex-1.0), u_ScaleZ.z, 1.0 );
	gl_Position = u_ViewProj * pos;
}