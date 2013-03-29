//%meta getIntencity getColor

uniform	mat4	u_ViewInverse;
uniform	vec4	c_LightPos;

const	vec4	c_Color		= vec4(1.0,1.0,1.0,1.0);


float getIntencity(vec3 wp)	{
	return 1.0;
}

vec4 getColor()	{
	return c_Color;
}
