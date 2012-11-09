#version 150 core
in vec2 texCoords;

uniform float color=0.5;
uniform sampler2D image;

out vec4 result;

void main()	{
	result = texture(image,texCoords);
}