#version 150 core

uniform mat4 u_World, u_ViewProj;
uniform	vec3 u_PosCam;
const	vec3 c_LightPos = vec3(3.0,3.0,3.0);

in	vec3 a_Position;
in	vec2 a_Tex0;
in	vec4 a_Normal;

out	vec2 texCoords;
out vec3 normal, vecLight, vecHalf;
out	mat4 mx;


void main()	{
	texCoords = vec2( a_Tex0.x, 1.0-a_Tex0.y );
	normal = mat3(u_World) * a_Normal.xyz;
	vec4 posWorld = u_World * vec4(a_Position,1.0);
	vecLight = c_LightPos - posWorld.xyz;
	vec3 vecCam = u_PosCam - posWorld.xyz;
	vecHalf = normalize(vecLight) + normalize(vecCam);
	gl_Position = u_ViewProj * posWorld;
}