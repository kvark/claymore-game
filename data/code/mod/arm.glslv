in ivec4	a_BoneIndex;
in vec4		a_BoneWeight;

const int MAX_BONES = 100;

struct Space { vec4 pos; vec4 rot; };
uniform vec4 bone_pos[MAX_BONES];
uniform vec4 bone_rot[MAX_BONES];

vec3 qrot(vec4 q, vec3 v)	{
	return v + 2.0*cross(q.xyz, cross(q.xyz,v) + q.w*v);
}
vec3 transForward(Space s, vec3 v)	{
	return qrot(s.rot, v*s.pos.w) + s.pos.xyz;
}

Space trans = Space( vec4(0.0), vec4(0.0) ); 

vec3 modifyInit(vec3 pos)	{
	for(int i=0; i<4; ++i)	{
		int bid = a_BoneIndex[i];
		float w = a_BoneWeight[i];
		Space s = Space( bone_pos[bid], bone_rot[bid] );
		trans.pos += w * vec4( transForward(s,pos), 1.0);
		trans.rot += w * s.rot;
	}
	trans.rot = normalize( trans.rot );
	return trans.pos.xyz;
}

vec3 modifyVector(vec3 vector)	{
	return qrot( trans.rot, vector );
}
