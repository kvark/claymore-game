uniform mat4 u_World, u_ViewProj;

in	vec3 a_Position;
in	vec4 a_Normal;

out vec3 v_Normal;


void main()	{
	vec3 pos = a_Position;
	vec3 nor = a_Normal.xyz;
	//%modify pos nor
	vec4 wp = u_World * vec4(pos,1.0);
	initMaterial( wp.xyz );
	v_Normal = mat3(u_World) * nor;
	gl_Position = u_ViewProj * wp;
}
