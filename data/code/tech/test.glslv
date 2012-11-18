in vec3 a_Position;

uniform mat4 u_World, u_ViewProj;

void main()	{
	vec3 pos = a_Position;
	//%modify pos
	gl_Position = u_ViewProj * u_World * vec4(pos,1.0);
}
