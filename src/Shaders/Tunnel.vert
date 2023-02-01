#version 450 core

layout(location = 0) in vec2 pos;

uniform vec3 center;

out float dist;

void main() {
  float x = (pos.x + center.x) / -center.z;
  float y = (pos.y + center.y) / -center.z;
  dist = -center.z;
  gl_Position = vec4(x, y, 0.0, 1.0);
}