#version 150 core

uniform vec4 u_CameraPos, u_LightPos;

in	vec2 a_Tex0;

out vec2 v_Tex;
out vec3 v_Light, v_Half;


void initMaterial(vec3 position)	{
	v_Tex = vec2( a_Tex0.x, 1.0-a_Tex0.y );
	v_Light = u_LightPos.xyz - position;
	vec3 vCam = u_CameraPos.xyz - position;
	v_Half = normalize(vCam) + normalize(v_Light);
}
