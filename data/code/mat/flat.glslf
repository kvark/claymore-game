//%meta initSurface computeLight getWorldNormal getColor

const vec4	c_Emissive		= vec4(0.0,0.0,0.0,1.0);
const vec3	c_Ambient		= vec3(0.2);
const vec3	c_Diffuse		= vec3(0.2);

vec3 getWorldNormal()	{
	return vec3(0.0,0.0,1.0);
}

vec4 getColor()	{
	return c_Emissive;
}


bool initAlpha()	{
	return getColor().a > 0.1;
}

vec4 initSurface()	{
	return getColor();
}

vec3 computeLight(float ambient, float reflected, vec3 light)	{
    return ambient*c_Ambient + reflected*c_Diffuse;
}

