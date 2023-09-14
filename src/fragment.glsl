#version 100
varying highp vec2 uv;

uniform sampler2D Texture;
uniform highp float aspect;
uniform highp int ballCount;

struct MetaBall {
    highp float r;
    highp vec2 pos;
    highp vec3 col;
};

highp vec4 BallSDF(MetaBall ball, highp vec2 uv) {
    highp float dst = ball.r / length(uv - ball.pos);
    return vec4(ball.col * dst, dst);
}

MetaBall decodeBall(int index) {
    MetaBall b;
    highp float y = (float(index) + 0.5) / float(ballCount);
    highp vec4 color = texture2D(Texture, vec2(0.5 / 4.0, y));
    b.pos.x = (color.r * 255.0 + color.g + color.b / 255.0) * (color.a * 2.0 - 1.0);
    color = texture2D(Texture, vec2(1.5 / 4.0, y));
    b.pos.y = (color.r * 255.0 + color.g + color.b / 255.0) * (color.a * 2.0 - 1.0);
    color = texture2D(Texture, vec2(2.5 / 4.0, y));
    b.r = (color.r * 255.0 + color.g + color.b / 255.0) * (color.a * 2.0 - 1.0);
    b.col = texture2D(Texture, vec2(3.5 / 4.0, y)).rgb;
    return b;
}

highp vec3 renderMetaBall(highp vec2 uv) {
    highp float total = 0.0;
    highp vec3 color = vec3(0.0);
    for (int i = 0; i < ballCount; i++) {
        MetaBall b = decodeBall(i);
        highp vec4 bd = BallSDF(b, uv);
        total += bd.a;
        color += bd.rgb;
    }

    if (length(color) > 1.0) {
        color = normalize(color);
    }
    if (total < 1.0) {
        color = pow(color, vec3(0.5));
        color *= total / 4.0;
    }
    return color;
}

void main() {
    highp vec2 nuw = vec2(uv.x, 1.0 - uv.y) * 2.0 - 1.0;
    nuw *= vec2(aspect, 1.0);
    highp vec3 col = renderMetaBall(nuw);
    gl_FragColor = vec4(col, 1.0);
}