//%meta initMaterial cookVector 

uniform mat4 u_World;
uniform vec4 u_CameraPos, u_WorldQuat;

in	vec3 a_Position;
in	vec4 a_Quaternion;
in	vec2 a_Tex0;

out vec3 v_Eye;
out vec2 v_Tex;


vec3 qirot(vec4 q, vec3 v)	{
	return v - 2.0*cross(q.xyz, q.w*v - cross(q.xyz,v) );
}

vec3 cookVector(vec3 v)	{
	// convet to tangent space
	vec3 vo = qirot( u_WorldQuat, v );
	return qirot( a_Quaternion, vo );
}


vec3 initMaterial(vec3 position)	{
	vec3 pos = a_Position;
	//%modify pos
	v_Tex = vec2( a_Tex0.x, 1.0-a_Tex0.y );
	v_Eye = cookVector( u_CameraPos.xyz - position );
	return (u_World * vec4(pos,1.0)).xyz;
}


vec3 initMaterial()	{
	vec3 pos = a_Position;
	//%modify pos
	v_Eye = cookVector( u_CameraPos.xyz - pos );
	v_Tex = vec2( a_Tex0.x, 1.0-a_Tex0.y );
	return (u_World * vec4(pos,1.0)).xyz;
}
