#version 450 core

out vec4 Color;

in float dist;

void main() { Color = vec4(1.0, 1.0, 1.0, 0.0) * (1.0 - dist) * 2.0; }