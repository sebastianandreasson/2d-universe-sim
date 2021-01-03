precision highp float;
uniform float t;
uniform float skyTime;

uniform float dpi;
uniform vec2 resolution;
uniform sampler2D backBuffer;
uniform sampler2D dataTexture;
uniform sampler2D lightTexture;
uniform sampler2D particleTexture;

varying vec2 uv;
const float PI2 = 2. * 3.14159265358979323846;

// clang-format off
#pragma glslify: hsv2rgb = require('glsl-hsv2rgb')
#pragma glslify: snoise3 = require(glsl-noise/simplex/3d)
#pragma glslify: snoise2 = require(glsl-noise/simplex/2d)
#pragma glslify: random = require(glsl-random)

// clang-format on

void main() {
  vec3 color;
  vec2 grid = floor(uv * (resolution / dpi));
  vec2 textCoord = (uv * vec2(0.5, -0.5)) + vec2(0.5);

  vec4 particleCell = texture2D(particleTexture, textCoord);
  if (particleCell.a > 0.0) {
    color = vec3(0.8, 0.9, 1.0);
    gl_FragColor = vec4(color, 0.5);
    return;
  }

  float noise = snoise3(vec3(grid, t * 0.05));
  vec2 noise_2d = vec2(floor(0.5 + noise),
                       floor(0.5 + snoise3(vec3(grid, (t + 20.) * 0.05))));

  vec2 sampleCoord = textCoord + (noise_2d / (resolution / 2.));

  vec4 data = texture2D(dataTexture, textCoord);
  vec4 lightCell = texture2D(lightTexture, textCoord);

  float lightValue = lightCell.r;
  float sparkleValue = lightCell.g;
  float blueLightValue = lightCell.b;

  vec4 lightSampleCell = texture2D(lightTexture, sampleCoord);
  float sampleLightValue = lightSampleCell.r;

  lightValue = 0.5 * lightValue + 0.5 * sampleLightValue;
  lightValue += sparkleValue * (0.5 + noise * 0.1);
  int type = int((data.r * 255.) + 0.1);
  float energy = data.g;

  float hue = 0.0;
  float saturation = 0.6;
  float lightness = 0.3 + energy * 0.5;
  float a = 1.0;
  float brightness = 0.0;

  if (type == 0) { // Empty
    hue = 0.1;
    saturation = 0.1;
    lightness = 0.1;
    a = 0.175;
  } else if (type == 1) { // Water
    hue = 0.58;
    saturation = 0.6;
    lightness = 0.5 + energy * 0.25 + noise * 0.1;
    a = 0.4;
  } else if (type == 2) { // Foam
    hue = 0.1;
    saturation = 0.01;
    lightness = 0.5 + energy * 0.25 + noise * 0.1;
    a = 0.4;
  } else if (type == 10) { // Rock
    hue = 0.05;
    saturation = 0.1;
    lightness = 0.5 - energy * 0.5;
  } else if (type == 11) { // Dirt
    hue = 0.09;
    saturation = 0.45;
    lightness = 0.5 - energy * 0.5;
  } else if (type == 12) { // Grass
    hue = 0.25;
    saturation = 0.6;
    lightness = 0.5;
    a = 0.4;
  }
  lightness *= 0.85 + cos(skyTime * PI2) * 0.2;
  lightness *= (0.975 + snoise2(floor(uv * resolution / dpi)) * 0.025);
  lightness += lightValue / 2.;

  saturation = min(saturation, 1.0);
  lightness = min(lightness, 1.0);
  color = hsv2rgb(vec3(hue, saturation, lightness));

  color += vec3(0.25, 0.25, 0.7) * 0.6 * (blueLightValue + lightSampleCell.b);
  a += blueLightValue + lightSampleCell.b;
  gl_FragColor = vec4(color, a);
}