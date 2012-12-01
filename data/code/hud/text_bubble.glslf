#version 150 core

uniform sampler2D	t_Text;
uniform sampler2D	t_Bubble;
uniform vec4		u_Color;
uniform vec4		u_Bubble;

in	vec4 v_BubbleCoord;
in	vec2 v_TextCoord;
out vec4 o_Color;


void main()	{
	vec2 bc = u_Bubble.xy + v_BubbleCoord.zw *
		(step(u_Bubble.xy,v_BubbleCoord.xy) - v_BubbleCoord.xy);
	vec4 bubble = texture( t_Bubble, bc );
	vec4 text = texture( t_Text, v_TextCoord ).x * u_Color;
	o_Color = vec4( text.xyz*text.a + bubble.xyz*(1-text.a), text.a+bubble.a );
}