//%meta initMaterial cookVector 

uniform mat4 u_World;
uniform vec4 u_CameraPos;

in	vec3 a_Position;
in	vec4 a_Normal;
in	vec2 a_Tex0;

out vec3 v_Normal, v_Eye;
out vec2 v_Tex;

vec3 cookVector(vec3 v)	{
	return v;
}

vec3 initMaterial()	{
	vec3 pos = a_Position;
	vec3 nor = a_Normal.xyz;
	//%modify pos nor
	vec3 wp = (u_World * vec4(pos,1.0)).xyz;
	v_Normal = mat3(u_World) * nor;
	v_Eye = cookVector( u_CameraPos.xyz - wp );
	v_Tex = vec2( a_Tex0.x, 1.0-a_Tex0.y );
	return wp;
}