//%meta initMaterial cookVector

uniform mat4 u_ViewProj;
uniform vec4 u_LightPos;

out vec3 v_Light;

void main()	{
	vec3 wp = initMaterial();
	v_Light = cookVector(u_LightPos.xyz - wp * u_LightPos.w);
	gl_Position = u_ViewProj * vec4(wp,1.0);
}
