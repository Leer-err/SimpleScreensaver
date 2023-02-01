#version 450 core

layout(location = 0) in vec3 pos;

out float dist;

void main() {
  float x = abs(pos.x / pos.z);
  float y = abs(pos.y / pos.z);
  dist = -pos.z;
  gl_Position = vec4(x, y, 0.0, 1.0);
}