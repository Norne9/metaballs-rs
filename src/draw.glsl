#version 100
varying highp vec2 uv;
uniform sampler2D Texture;
uniform highp vec2 resolution;

const highp float Pi = 6.28318530718;// Pi*2
// GAUSSIAN BLUR SETTINGS {{{
const highp float Directions = 16.0;// BLUR DIRECTIONS (Default 16.0 - More is better but slower)
const highp float Size = 4.0;// BLUR SIZE (Radius)
// GAUSSIAN BLUR SETTINGS }}}

void main() {
    highp vec2 Radius = Size/resolution;

    // Pixel colour
    highp vec4 blur = texture2D(Texture, uv);
    /*
        // Blur calculations
        for (highp float d = 0.0; d < Pi; d += Pi / Directions)
        {
            blur += texture2D(Texture, uv + vec2(cos(d), sin(d))*Radius);
        }

        // Output to screen
        blur /= Directions;*/

    highp vec3 color = blur.rgb;
    highp float total = blur.a;

    highp float mix_factor = smoothstep(0.0, 1.0, (total - 0.85) / 0.1);
    color = mix(color * total * 0.5, color, mix_factor);

    //gl_FragColor = vec4(color, 1.0);
    gl_FragColor = vec4(vec3(total), 1.0);
}