#version 150 core

uniform vec4 		u_Color;
uniform sampler2D	t_Main;

in vec2 v_Tex;
in vec3 v_Light, v_Half;
in vec4 v_Color;

const float	shininess	= 10.0;
const float	ambient		= 0.1;
const vec3	colorDiffuse	= vec3(1.0);
const vec3	colorSpecular	= vec3(0.3);


//%meta getAlpha getFinalColor

lowp float getAlpha()	{
	return texture(t_Main,v_Tex).a;
}

lowp vec4 getFinalColor(vec3 normal)	{
	float kdiff = dot( normal, normalize(v_Light) );
	float kspec = dot( normal, normalize(v_Half) );
	float xd = max(0.0,kdiff) + ambient;
	float xs = pow(max(0.01,kspec),shininess);
	vec4 sample = texture(t_Main,v_Tex);
	vec3 color = xd*colorDiffuse*sample.xyz + xs*colorSpecular;
	return vec4( color, sample.a );
}
