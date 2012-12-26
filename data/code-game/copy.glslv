#version 150 core

in	vec2 a_Vertex;
out	vec2 v_TexCoords;


void main()	{
	vec2 pos = 2.0*a_Vertex-1.0;
	v_TexCoords = vec2( a_Vertex.x, 1.0-a_Vertex.y );
	gl_Position = vec4( pos, 0.0, 1.0 );
}