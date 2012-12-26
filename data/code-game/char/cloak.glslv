//%meta initMaterial cookVector 

uniform mat4 u_World;
uniform vec4 u_CameraPos;

in	vec3 a_Position;
in	vec4 a_Normal;

out vec3 v_Normal, v_Eye;

vec3 cookVector(vec3 v)	{
	return v;
}

vec3 initMaterial()	{
	vec3 pos = modifyInit( a_Position );
	vec3 nor = modifyVector( a_Normal.xyz );
	vec3 wp = (u_World * vec4(pos,1.0)).xyz;
	v_Normal = mat3(u_World) * nor;
	v_Eye = cookVector( u_CameraPos.xyz - wp );
	return wp;
}
