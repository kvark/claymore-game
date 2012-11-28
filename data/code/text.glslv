#version 150 core

uniform vec4 u_Transform;

in	vec2 a_Vertex;
out	vec2 v_TexCoord;

void main()	{
	v_TexCoord = a_Vertex * vec2(200.0,50.0);
	gl_Position = vec4( a_Vertex*u_Transform.xy + u_Transform.zw, 0.0, 1.0 );	
}
