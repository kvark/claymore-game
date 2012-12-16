#version 150 core

uniform vec4 u_Transform;

in	vec2 a_Vertex;


void main()	{
	gl_Position = vec4( a_Vertex*u_Transform.xy + u_Transform.zw, 0.0, 1.0 );	
}
