in ivec4	a_BoneIndex;
in vec4		a_BoneWeight;

const int MAX_BONES = 100;

struct Space	{ vec4 pos,rot; };
uniform vec4 bone_pos[MAX_BONES];
uniform vec4 bone_rot[MAX_BONES];

// Quaternion routines
vec4 qinv(vec4 q)	{
	return vec4(-q.xyz,q.w);
}
vec4 qmul(vec4 a, vec4 b)	{
	return vec4(cross(a.xyz,b.xyz) + a.xyz*b.w + b.xyz*a.w, a.w*b.w - dot(a.xyz,b.xyz));
}
vec3 qrot(vec4 q, vec3 v)	{
	return v + 2.0*cross(q.xyz, cross(q.xyz,v) + q.w*v);
}
vec3 transForward(Space s, vec3 v)	{
	return qrot(s.rot, v*s.pos.w) + s.pos.xyz;
}

// Dual-Quaternion routines
struct DualQuat	{
	vec4 re,im;
	float scale;
};

Space normalizeDq(DualQuat dq)	{
	float k = 1.0 / length(dq.re);
	vec4 tmp = qmul( dq.im, qinv(dq.re) );
	vec4 pos = vec4( 2.0*k*k*tmp.xyz, dq.scale );
	return Space( pos, k * dq.re );
}


Space trans = Space( vec4(0.0), vec4(0.0) );

vec3 modifyPosition(vec3 pos)	{
	DualQuat dq = DualQuat( vec4(0.0), vec4(0.0), 0.0 );
	for(int i=0; i<4; ++i)	{
		int bid = a_BoneIndex[i];
		float w = a_BoneWeight[i];
		Space s = Space( bone_pos[bid], bone_rot[bid] );
		vec4 pos = vec4(0.5 * s.pos.xyz, 0.0);
		dq.re += w * s.rot;
		dq.im += w * qmul( pos, s.rot );
		dq.scale += w * s.pos.w;
	}
	trans = normalizeDq(dq);
	return transForward(trans,pos);
}

vec3 modifyVector(vec3 vector)	{
	return qrot( trans.rot, vector );
}
