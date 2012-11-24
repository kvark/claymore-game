#version 150 core

uniform	sampler2D	t_Grid;
const vec2	c_Size			= vec2(0.1,0.1);
const float	c_AlphaPower	= 0.1;

in	vec2 v_TexCoords;
out vec4 o_Color;


void main()	{
	vec2 cellId = floor( v_TexCoords / c_Size );
	vec3 color = texture(t_Grid,cellId).xyz;
	vec2 offCenter = v_TexCoords - (cellId+0.5)*c_Size;
	vec2 offSide = 1.0 - abs(offCenter)*(2.0/c_Size);
	float minOffset = min(offSide.x,offSide.y);
	float alpha = 1 - pow(minOffset,c_AlphaPower);
	o_Color = vec4( color, alpha );
}