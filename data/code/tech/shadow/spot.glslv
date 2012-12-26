//%meta initMatPure

uniform mat4	u_LightProj;

out	vec4 v_Light;


void main()	{
	vec3 wp = initMatRich();
	v_Light = u_LightProj * vec4(wp,1.0);
}
