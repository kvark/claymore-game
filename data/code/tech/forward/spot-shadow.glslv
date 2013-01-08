//%meta initMatRich cookVector

uniform mat4 u_ViewProj, u_LightProj;
uniform vec4 u_LightPos;

out vec3 v_LightMaterial;
out vec4 v_LightShadow;

void main()	{
	vec3 wp = initMatRich();
	v_LightMaterial	= cookVector(u_LightPos.xyz - wp * u_LightPos.w);
	v_LightShadow	= u_LightProj	* vec4(wp,1.0);
	// transform from NDC [-1,1] to texture [0,1] space
	v_LightShadow.xyz = 0.5*(v_LightShadow.xyz + v_LightShadow.w);
	gl_Position		= u_ViewProj	* vec4(wp,1.0);
}
