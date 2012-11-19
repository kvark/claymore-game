#version 150 core

uniform vec4 		u_Color;
uniform sampler2D	t_Main, t_Normal;

in vec2 v_Tex;
in vec3 v_Light, v_Half;
in vec4 v_Color;

const float	c_Shininess	= 10.0;
const float	c_Ambient		= 0.1;
const vec3	c_ColorDiffuse	= vec3(1.0);
const vec3	c_ColorSpecular	= vec3(0.3);


//%meta getAlpha getFinalColor

float getAlpha()	{
	return texture(t_Main,v_Tex).a;
}

vec4 getFinalColor(vec3 vertexNormal)	{
	vec3 rawNormal = texture(t_Normal,v_Tex);
	vec3 normal = normalize(rawNormal);
	float kdiff = dot( normal, normalize(v_Light) );
	float kspec = dot( normal, normalize(v_Half) );
	float xd = max(0.0,kdiff) + c_Ambient;
	float xs = pow(max(0.01,kspec),c_Shininess);
	vec4 sample = texture(t_Main,v_Tex);
	vec3 color = xd*c_ColorDiffuse*sample.xyz + xs*c_ColorSpecular;
	return vec4( color, sample.a );
}
