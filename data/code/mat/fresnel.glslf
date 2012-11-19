#version 150 core

uniform vec4 		u_Color;
uniform sampler2D	t_Main, t_Reflect;

in vec2 v_Tex;
in vec3 v_Light, v_Eye;
in vec4 v_Color;

const vec3	c_ColorReflect	= vec3(1.0,0.0,0.0);
const vec3	c_ColorFront	= vec3(0.0,1.0,0.0);
const vec3	c_ColorBack		= vec3(0.0,0.0,1.0);
const vec4	c_FresnelParams	= vec4(0.2,2.0,0.0,0.0);	//bias,power


//%meta getFresnel getAlpha getFinalColor

float getFresnel(float NdV)	{
	// http://http.developer.nvidia.com/GPUGems2/gpugems2_chapter19.html
	return max( 0.0, (1.0-c_FresnelParams.x) * pow(1.0-NdV,c_FresnelParams.y) );
}

float getAlpha()	{
	return texture(t_Main,v_Tex).a;
}

vec4 getFinalColor(vec3 normal)	{
	float NdV = max( 0.0, dot( normal, normalize(v_Eye) ));
	float fresnel = getFresnel(NdV);
	vec3 diffuse = mix( c_ColorFront, c_ColorBack*NdV, NdV );
	vec4 sample = texture(t_Main,v_Tex);
	return vec4(diffuse*sample.xyz + fresnel*c_ColorReflect, sample.a);
}
