#version 150 core

uniform mat4		u_ViewProjInverse;

in	vec2 a_Vertex;
out	vec4 v_TexCoords;


void main()	{
	vec2 pos = 2.0*a_Vertex-1.0;
	v_TexCoords = u_ViewProjInverse * vec4( pos, 1.0, 0.0 );
	gl_Position = vec4( pos, 1.0, 1.0 );
}