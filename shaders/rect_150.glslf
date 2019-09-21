#version 150 core

uniform sampler2D t_Awesome;
uniform int i_Switch;

in vec4 v_Color;
in vec2 v_Uv;
out vec4 Target0;

void main() {
    vec3 aw = texture(t_Awesome, v_Uv).rgb;

    if(i_Switch == 0) {
        if(aw == vec3(0.0, 0.0, 0.0)) {
            Target0 = 0.20 * v_Color;
        } else {
            Target0 = vec4(aw, 1.0);
        }
    } else {
        Target0 = v_Color;
    }
}