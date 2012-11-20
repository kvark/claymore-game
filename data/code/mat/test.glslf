//%meta initSurface computeLight getColor

vec4 initSurface()	{
	return vec4(0.1,0.2,0.3,1.0);
}

vec3 computeLight(float, float, vec3 l)	{
	return l;
}

vec4 getColor()	{
	return vec4(0.0,0.5,1.0,1.0);
}
