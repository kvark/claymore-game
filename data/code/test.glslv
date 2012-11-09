#version 150 core
uniform mat4 u_World, u_ViewProj;

in	vec3 a_Position;

out	vec2 texCoords;
out	mat4 mx;


void main()	{
	texCoords = vec2(0.5,-0.5)*a_Position.xy + 0.5;
	gl_Position = u_ViewProj * u_World * vec4(a_Position,1.0);
}