//%meta initMaterial cookVector

uniform mat4 u_World;

in	vec3 a_Position;

vec3 cookVector(vec3 v)	{
	return v;
}

vec3 initMaterial()	{
	vec3 pos = a_Position;
	//%modify pos
	return (u_World * vec4(pos,1.0)).xyz;
}