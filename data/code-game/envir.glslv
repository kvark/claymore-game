#version 150 core

uniform mat4	u_ViewProjInverse;
uniform vec4	u_CameraPos;

in	vec2 a_Vertex;
out	vec4 v_TexCoords;


void main()	{
	vec2 pos = 2.0*a_Vertex-1.0;
	vec4 cx = u_ViewProjInverse * vec4( 1.0, 0.0, 0.0, 0.0 );
	vec4 cy = u_ViewProjInverse * vec4( 0.0, 1.0, 0.0, 0.0 );
	vec3 bx = normalize(cx.xyz), by = normalize(cy.xyz);
	vec3 bz = cross(bx,by);
	v_TexCoords = u_ViewProjInverse * vec4( pos, 1.0, 0.0 );
	//vec4 pw = u_ViewProjInverse * vec4( pos, 0.0, 1.0 );
	//v_TexCoords = pw/pw.w - u_CameraPos;
	gl_Position = vec4( pos, 1.0, 1.0 );
}