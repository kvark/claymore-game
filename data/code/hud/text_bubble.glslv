#version 150 core

uniform vec4 u_Transform;
uniform vec4 u_Bubble;

in	vec2 a_Vertex;
out vec4 v_BubbleCoord;
out	vec2 v_TextCoord;

vec2 flip(vec2 v)	{
	return vec2(v.x,1.0-v.y);
}

void main()	{
	vec2 tc = flip(a_Vertex);
	v_BubbleCoord = vec4( tc, 1.0/u_Bubble.zw );
	vec2 vt = tc + (step(u_Bubble.xy,tc) - u_Bubble.xy) * u_Bubble.zw;
	v_TextCoord = vt;
	gl_Position = vec4( flip(vt)*u_Transform.xy + u_Transform.zw, 0.0, 1.0 );	
}
