#version 100
varying highp vec2 uv;
uniform sampler2D Texture;
uniform highp vec2 resolution;

const highp float Pi = 6.28318530718;// Pi*2
// GAUSSIAN BLUR SETTINGS {{{
const highp float Directions = 16.0;// BLUR DIRECTIONS (Default 16.0 - More is better but slower)
const highp float Size = 2.0;// BLUR SIZE (Radius)
// GAUSSIAN BLUR SETTINGS }}}

void main() {
    highp vec2 Radius = Size/resolution;

    // Pixel colour
    highp vec4 blur = texture2D(Texture, uv);

    // Blur calculations
    for (highp float d = 0.0; d < Pi; d += Pi / Directions)
    {
        blur += texture2D(Texture, uv + vec2(cos(d), sin(d))*Radius);
    }

    // Output to screen
    blur /= Directions;

    highp vec3 og_color = blur.rgb;
    highp float total = blur.a;

    highp float mix_factor = smoothstep(0.0, 1.0, (total - 0.875) / 0.05);
    highp vec3 color = og_color * total;
    color = mix(color * 0.5, color, mix_factor);

    // line
    mix_factor = smoothstep(0.2, 0.8, abs(total - 0.9) / 0.05);
    color = mix(mix(og_color, vec3(1.0), 0.8), color, mix_factor);

    gl_FragColor = vec4(color, 1.0);
}