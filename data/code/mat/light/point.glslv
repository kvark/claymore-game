//%meta initLight 

uniform mat4 u_World;

in	vec3 a_Position;


vec3 initLight()	{
	vec3 pos = a_Position;
	return (u_World * vec4(pos,1.0)).xyz;
}
