#version 300 es
precision mediump float;

in vec2 v_TexCoord;
out vec4 Target0;
uniform sampler2D t_Color;

void main() {
    vec4 tex = texture(t_Color, v_TexCoord);
    float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
    Target0 = mix(tex, vec4(0.0,0.0,0.0,1.0), blend*1.0);
}
