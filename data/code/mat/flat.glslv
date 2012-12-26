//%meta initMatPure initMatRich cookVector 

uniform mat4 u_World;

in	vec3 a_Position;


vec3 initMatPure()	{
	vec3 pos = modifyInit( a_Position );
	return (u_World * vec4(pos,1.0)).xyz;
}

vec3 cookVector(vec3 v)	{
	return v;
}

vec3 initMatRich()	{
	return initMatPure();
}