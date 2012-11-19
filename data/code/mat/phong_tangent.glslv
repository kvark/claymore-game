#version 150 core

uniform vec4 u_CameraPos, u_LightPos, u_WorldQuat;

in	vec2 a_Tex0;
in	vec4 a_Quaternion;

out vec2 v_Tex;
out vec3 v_Light, v_Half;


vec3 qirot(vec4 q, vec3 v)	{
	return v - 2.0*cross(q.xyz, q.w*v - cross(q.xyz,v) );
}

vec3 w2tan(vec3 v)	{
	vec3 vo = qirot( u_WorldQuat, v );
	return qirot( a_Quaternion, vo );
}


void initMaterial(vec3 position)	{
	v_Tex = vec2( a_Tex0.x, 1.0-a_Tex0.y );
	v_Light = w2tan( u_LightPos.xyz - position );
	vec3 vCam = w2tan( u_CameraPos.xyz - position );
	v_Half = normalize(vCam) + normalize(v_Light);
}
