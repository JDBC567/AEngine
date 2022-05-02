#version 460 core

in vec2 uv;

layout(binding=0) uniform sampler2D tex;

out vec4 color;

void main() {
    color=texture(tex,uv);
}
