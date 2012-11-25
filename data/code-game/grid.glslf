#version 150 core

uniform	sampler2D	t_Grid;
uniform	vec4		u_Size;
const float	c_AlphaPower	= 0.1;

in	vec2 v_TexCoords;
out vec4 o_Color;


void main()	{
	vec4 color = texture(t_Grid,v_TexCoords);
	vec2 cellId = floor( v_TexCoords / u_Size.xy );
	vec2 offCenter = v_TexCoords - (cellId+0.5)*u_Size.xy;
	vec2 offSide = 1.0 - abs(offCenter)*(2.0/u_Size.xy);
	float minOffset = min(offSide.x,offSide.y);
	float alpha = 1 - pow(minOffset,c_AlphaPower);
	o_Color = vec4( color.xyz, max(color.a,alpha) );
}