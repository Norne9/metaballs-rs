#define DSP_STR 1.5

varying highp vec2 uv;

uniform sampler2D Texture;
uniform highp float aspect;
uniform highp float zoom;

struct MetaBall {
    highp float r;
    highp vec2 pos;
    highp float col;
};

highp float BallSDF(MetaBall ball, highp vec2 uv) {
    return ball.r / length(uv - ball.pos);
}

highp float CalcAlpha(highp float dst) {
    dst = pow(dst, 3.0);
    return dst > 1.0 ? 2.0 : dst;
}

highp vec2 CalcColor(MetaBall ball, highp float dst) {
    return vec2(ball.col, dst > 1.0 ? pow(dst, 3.0) : dst);
}

MetaBall decodeBall(int index) {
    MetaBall b;
    highp float y = (float(index) + 0.5) / float(BALL_COUNT);
    highp vec4 color = texture2D(Texture, vec2(0.5 / 4.0, y));
    b.pos.x = (color.r * 255.0 + color.g + color.b / 255.0) * (color.a * 2.0 - 1.0) / 10.0;
    color = texture2D(Texture, vec2(1.5 / 4.0, y));
    b.pos.y = (color.r * 255.0 + color.g + color.b / 255.0) * (color.a * 2.0 - 1.0) / 10.0;
    color = texture2D(Texture, vec2(2.5 / 4.0, y));
    b.r = (color.r * 255.0 + color.g + color.b / 255.0) * (color.a * 2.0 - 1.0) / 10.0;
    color = texture2D(Texture, vec2(3.5 / 4.0, y));
    b.col = (color.r * 255.0 + color.g + color.b / 255.0) * (color.a * 2.0 - 1.0) / 10.0;
    return b;
}

highp vec3 hsv2rgb(highp vec3 c)
{
    highp vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    highp vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

highp float getsat(highp vec3 c)
{
    highp float mi = min(min(c.x, c.y), c.z);
    highp float ma = max(max(c.x, c.y), c.z);
    return (ma - mi)/(ma+ 1e-7);
}

//Improved rgb lerp
highp vec3 iLerp(highp vec3 a, highp vec3 b, highp float x)
{
    //Interpolated base color (with singularity fix)
    highp vec3 ic = mix(a, b, x) + vec3(1e-6, 0., 0.);

    //Saturation difference from ideal scenario
    highp float sd = abs(getsat(ic) - mix(getsat(a), getsat(b), x));

    //Displacement direction
    highp vec3 dir = normalize(vec3(2.*ic.x - ic.y - ic.z, 2.*ic.y - ic.x - ic.z, 2.*ic.z - ic.y - ic.x));
    //Simple Lighntess
    highp float lgt = dot(vec3(1.0), ic);

    //Extra scaling factor for the displacement
    highp float ff = dot(dir, normalize(ic));

    //Displace the color
    ic += DSP_STR*dir*sd*ff*lgt;
    return clamp(ic, 0., 1.);
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
        color = iLerp(color, hsv2rgb(vec3(ha.x, 1.0, 1.0)), ha.y / total_rgba);
        //color += hsv2rgb(vec3(ha.x, 1.0, 1.0)) * ha.y;

    }
    //color /= total_rgba;

    highp vec3 colorRgb = color;

    return vec4(colorRgb, total);
}

void main() {
    highp vec2 nuw = vec2(uv.x, 1.0 - uv.y) * 2.0 - 1.0;
    nuw *= vec2(aspect, 1.0) * zoom;
    highp vec4 col = renderMetaBall(nuw);
    gl_FragColor = col;
}