//%meta initMatRich

uniform mat4 u_ViewProj;

void main()	{
	vec3 wp = initMatRich();
	gl_Position = u_ViewProj * vec4(wp,1.0);
}
