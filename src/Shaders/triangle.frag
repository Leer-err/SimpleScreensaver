#version 450 core

#define point1 vec2(0.2, 0.3)
#define point2 vec2(0.8, 0.6)
#define scale 0.4

uniform float time;

out vec4 Color;

in vec2 TexCoords;
in vec2 ScreenCoords;

vec2 hash( in vec2 x )
{
    const vec2 k = vec2( 0.3183099, 0.3678794 );             // replace all of
    x = x*k + k.yx;                                          // this by something 
    float h = fract( 16.0 * k.x*fract( x.x*x.y*(x.x+x.y)) ); // better
    
    float a = 6.2831*h + time;
    return vec2( cos(a), sin(a) );
}

vec3 noised( in vec2 p )
{
    vec2 i = floor( p );
    vec2 f = fract( p );

    vec2 u = f*f*f*(f*(f*6.0-15.0)+10.0);
    vec2 du = 30.0*f*f*(f*(f-2.0)+1.0);
    
    vec2 ga = hash( i + vec2(0.0,0.0) );
    vec2 gb = hash( i + vec2(1.0,0.0) );
    vec2 gc = hash( i + vec2(0.0,1.0) );
    vec2 gd = hash( i + vec2(1.0,1.0) );
    
    float va = dot( ga, f - vec2(0.0,0.0) );
    float vb = dot( gb, f - vec2(1.0,0.0) );
    float vc = dot( gc, f - vec2(0.0,1.0) );
    float vd = dot( gd, f - vec2(1.0,1.0) );

    return vec3( va + u.x*(vb-va) + u.y*(vc-va) + u.x*u.y*(va-vb-vc+vd),   // value
                 ga + u.x*(gb-ga) + u.y*(gc-ga) + u.x*u.y*(ga-gb-gc+gd) +  // derivatives
                 du * (u.yx*(va-vb-vc+vd) + vec2(vb,vc) - va));
}

float sdfCircle(vec2 p, float r) { 
    return length(p) - r;
}

float udSegment( in vec2 p, in vec2 a, in vec2 b )
{
    vec2 ba = b-a;
    vec2 pa = p-a;
    float h =clamp( dot(pa,ba)/dot(ba,ba), 0.0, 1.0 );
    return length(pa-h*ba);
}

void main() {
    vec2 noiseCoords = ScreenCoords / scale - vec2(0, time);
    float noise = noised(noiseCoords).x;

    float dist = udSegment(ScreenCoords + vec2(noise), point1, point2);

    Color = vec4(vec3(1- clamp(noise + dist, 0.0, 1.0)), 0.0);

    
    // float yGradient = clamp(min(0.7 - TexCoords.y, TexCoords.y*6), 0.0, 1.0) * 3;
    // vec2 noiseCoords = TexCoords*4.0 + vec2(0, time*3);
    // float noise = noised(noiseCoords).x * 0.8;
    // vec2 offset = vec2(noise*0.2, noise * yGradient);
    // float sdf = -sdfCircle(TexCoords - vec2(0.5,0.7) + offset, 0.25);
    // Color = vec4(step(0.0, sdf), step(0.3, sdf + (TexCoords.y)*0.2 + 0.1), step(0.6, sdf + (TexCoords.y) * 0.5 + 0.1), 0.0);
}