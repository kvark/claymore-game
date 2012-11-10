#version 150 core
uniform mat4 u_World, u_ViewProj;

in	vec3 a_Position;
in	vec2 a_Tex0;

out	vec2 texCoords;
out	mat4 mx;


void main()	{
	texCoords = vec2( a_Tex0.x, 1.0-a_Tex0.y );
	gl_Position = u_ViewProj * u_World * vec4(a_Position,1.0);
}