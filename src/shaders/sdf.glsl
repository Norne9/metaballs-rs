varying highp vec2 uv;

uniform sampler2D Texture;
uniform highp float aspect;

struct MetaBall {
    highp float r;
    highp vec2 pos;
    highp float col;
};

highp float BallSDF(MetaBall ball, highp vec2 uv) {
    return ball.r / length(uv - ball.pos);
}

highp float CalcAlpha(highp float dst) {
    dst = pow(dst, 1.5);
    return dst > 1.0 ? 2.0 : dst;
}

highp vec2 CalcColor(MetaBall ball, highp float dst) {
    return vec2(ball.col, pow(dst, 10.0));
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
    color = texture2D(Texture, vec2(3.5 / 4.0, y));
    b.col = (color.r * 255.0 + color.g + color.b / 255.0) * (color.a * 2.0 - 1.0) / 20.0;
    return b;
}

highp vec3 rgb2hsv(highp vec3 c)
{
    highp vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    highp vec4 p = mix(vec4(c.bg, K.wz), vec4(c.gb, K.xy), step(c.b, c.g));
    highp vec4 q = mix(vec4(p.xyw, c.r), vec4(c.r, p.yzx), step(p.x, c.r));

    highp float d = q.x - min(q.w, q.y);
    highp float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

highp vec3 hsv2rgb(highp vec3 c)
{
    highp vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    highp vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

highp vec4 renderMetaBall(highp vec2 uv) {
    highp float total = 0.0;
    highp float total_rgba = 0.0;
    highp vec3 color = vec3(0.0);

    for (int i = 0; i < BALL_COUNT; i++) {
        MetaBall b = decodeBall(i);
        highp float dst = BallSDF(b, uv);

        highp float a = CalcAlpha(dst);
        total += a;

        highp vec2 ha = CalcColor(b, dst);
        total_rgba += ha.y;
        color += hsv2rgb(vec3(ha.x, 1.0, 1.0)) * ha.y;
    }
    color /= total_rgba;

    highp vec3 colorRgb = hsv2rgb(vec3(rgb2hsv(color).x, 1.0, 1.0));

    total = smoothstep(0.0, 1.0, (total - 0.9) / 0.45);

    return vec4(colorRgb, total);
}

void main() {
    highp vec2 nuw = vec2(uv.x, 1.0 - uv.y) * 2.0 - 1.0;
    nuw *= vec2(aspect, 1.0);
    highp vec4 col = renderMetaBall(nuw);
    gl_FragColor = col;
}