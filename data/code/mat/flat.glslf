//%meta initSurface computeLight getWorldNormal getColor

const vec4	c_Color		= vec4(0.0,0.0,0.0,1.0);

vec3 getWorldNormal()	{
	return vec3(0.0,0.0,1.0);
}

vec4 getColor()	{
	return c_Color;
}

vec4 initSurface()	{
	return c_Color;
}

vec3 computeLight(float ambient, float reflected, vec3 light)	{
    return vec3(0.0);
}

