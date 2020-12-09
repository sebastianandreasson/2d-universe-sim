precision highp float;
uniform float t;
uniform float dpi;
uniform vec2 resolution;
uniform bool isSnapshot;
uniform sampler2D backBuffer;
uniform sampler2D data;

varying vec2 uv;

void main() {
  vec3 color = vec3(0.0, 0.0, 0.0);
  float a = 1.0;

  vec2 textCoord = (uv * vec2(0.5, -0.5)) + vec2(0.5);
  vec4 data = texture2D(data, textCoord);
  int type = int((data.r * 255.) + 0.1);

  if (type == 1) {
    color = vec3(0.0, 1.0, 0.0);
    a = 1.0;
  } else if (type == 2) {
    color = vec3(0.0, 0.0, 1.0);
    a = 1.0;
  }

  // calculate the dot product of
  // the light to the vertex normal

 
  gl_FragColor = vec4(color, a);
}