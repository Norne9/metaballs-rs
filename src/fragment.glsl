varying highp vec2 uv;

uniform sampler2D Texture;
uniform highp float aspect;

struct MetaBall {
    highp float r;
    highp vec2 pos;
    highp vec3 col;
};

highp vec4 BallSDF(MetaBall ball, highp vec2 uv) {
    highp float dst = ball.r / length(uv - ball.pos);
    dst = pow(dst, 1.5);
    return vec4(ball.col * dst, dst > 1.0 ? 2.0 : dst);
}

MetaBall decodeBall(int index) {
    MetaBall b;
    highp float y = (float(index) + 0.5) / float(BALL_COUNT);
    highp vec4 color = texture2D(Texture, vec2(0.5 / 4.0, y));
    b.pos.x = (color.r * 255.0 + color.g + color.b / 255.0) * (color.a * 2.0 - 1.0) / 20.0;
    color = texture2D(Texture, vec2(1.5 / 4.0, y));
    b.pos.y = (color.r * 255.0 + color.g + color.b / 255.0) * (color.a * 2.0 - 1.0) / 20.0;
    color = texture2D(Texture, vec2(2.5 / 4.0, y));
    b.r = (color.r * 255.0 + color.g + color.b / 255.0) * (color.a * 2.0 - 1.0) / 20.0;
    b.col = texture2D(Texture, vec2(3.5 / 4.0, y)).rgb;
    return b;
}

highp vec4 renderMetaBall(highp vec2 uv) {
    highp float total = 0.0;
    highp vec3 color = vec3(0.0);
    for (int i = 0; i < BALL_COUNT; i++) {
        MetaBall b = decodeBall(i);
        highp vec4 bd = BallSDF(b, uv);
        total += bd.a;
        color += bd.rgb;
    }
    total = smoothstep(0.0, 1.0, (total - 0.2) / 1.3);

    if (length(color) > 1.0) {
        color = normalize(color);
    }
    color *= total;
    if (total < 0.99) {
        color *= 0.5;
    }
    return vec4(color, total);
}

void main() {
    highp vec2 nuw = vec2(uv.x, 1.0 - uv.y) * 2.0 - 1.0;
    nuw *= vec2(aspect, 1.0);
    highp vec4 col = renderMetaBall(nuw);
    //gl_FragColor = vec4(vec3(col.a), 1.0);
    gl_FragColor = vec4(col.rgb, 1.0);
}