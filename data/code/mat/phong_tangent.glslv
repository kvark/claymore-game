//%meta initMaterial cookVector 

uniform mat4 u_World;
uniform vec4 u_CameraPos, u_WorldQuat;

in	vec3	a_Position;
in	vec4	a_Normal;
in	vec4	a_Tangent;
in	vec2	a_Tex0;

out vec3	v_Eye,v_NormalWorld;
out vec2	v_Tex;


vec3 qrot(vec4 q, vec3 v)	{
	return v + 2.0*cross( q.xyz, q.w*v + cross(q.xyz,v) );
}

vec3 qirot(vec4 q, vec3 v)	{
	return v - 2.0*cross( q.xyz, q.w*v - cross(q.xyz,v) );
}


mat3 TBN;

vec3 cookVector(vec3 v)	{
	return v * TBN;
}

vec3 initMaterial()	{
	vec3 pos = a_Position;
	vec3 nor = a_Normal.xyz;
	vec3 tan = a_Tangent.xyz;
	//%modify pos nor tan
	mat3 m3w = mat3(u_World);
	vec3 wp = m3w*pos + u_World[3].xyz;
	vec3 normal		= normalize( m3w * nor );
	vec3 tangent	= normalize( m3w * tan );
	vec3 bitangent	= cross(normal,tangent) * a_Tangent.w;
	TBN = mat3(tangent,bitangent,normal);
	v_Eye = cookVector( u_CameraPos.xyz - wp );
	v_NormalWorld = normal;
	v_Tex = vec2( a_Tex0.x, 1.0-a_Tex0.y );
	return wp;
}
