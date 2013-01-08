//%meta initMatPure

uniform mat4	u_LightProj;


void main()	{
	vec3 wp = initMatPure();
	gl_Position = u_LightProj * vec4(wp,1.0);
}
