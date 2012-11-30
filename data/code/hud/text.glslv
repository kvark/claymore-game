#version 150 core

uniform vec4 u_Transform;

in	vec2 a_Vertex;
out	vec2 v_TexCoord;

void main()	{
	v_TexCoord = vec2( a_Vertex.x, 1.0-a_Vertex.y );
	gl_Position = vec4( a_Vertex*u_Transform.xy + u_Transform.zw, 0.0, 1.0 );	
}
