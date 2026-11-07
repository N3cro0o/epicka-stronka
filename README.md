## Megaśna stronka aka porfolio

### Todo

* ~Techstack~
* ~Layout~
* ~Shit part~
* WASM part
  * ~WASM-HTML connection~
  * ~Main layout~
  * ~Background shader implementation~
  * ~Remove not working gif in iced wasm~
  * Add gradient background to iced wasm (damn doesn't work)
  * ~Moved handles initialization to MainLayout::new()~
* ~More to come~
* ~Background shader~
  * ~Create canvas element~
  * ~Translate Godot shader to WebGL2 shader~
  * ~Implement TIME variable~
  * ~Include noise texture~
* Change HTML design
* ~Change Github actions workflow to implement background wasm~ 
* ~Reduced images resolutions~

### Techstack

**Shit part:** pure madness (html + css + js)
**WASM:** Rust + iced

#### Shit part 

**Options:**
* HTML + CSS + JS
* React (either js or ts)
* Bootstrap

#### WASM part

Rust + iced

#### Shader tests

Godot 4

### Godot Shader

```
shader_type canvas_item;

group_uniforms Background;
uniform vec4 space_bg: source_color = vec4(0.05, 0.0, 0.2, 1.0);
uniform vec4 nebula_colour1: source_color = vec4(0.2, 0.0, 0.5, 1.0);
uniform vec4 nebula_colour2: source_color = vec4(0.4, 0.0, 0.3, 1.0);
uniform float nebula_visibility: hint_range(0.0, 1.0, 0.02) = 0.2;
uniform vec2 nebula_speed = vec2(0.1, 1);

uniform sampler2D noise_texture : repeat_enable, filter_linear;

group_uniforms Stars;

uniform sampler2D gradientA;
uniform int star_seed: hint_range(0, 2048, 1) = 0;
uniform int iterations: hint_range(1, 5, 1) = 2;
uniform float star_frequency: hint_range(0.9, 1, 0.001) = 0.99;
uniform vec2 star_speed = vec2(-0.1, 0.2);
uniform float star_brightness: hint_range(0.0, 1.0, 0.001) = 0.85;

const float star_size = 100.0;

float rand(vec2 st, float seed) {
	return fract(sin(dot(st.xy, vec2(seed + 12.9898, 78.233))) * 43758.5453123);
}

void fragment() {
	// BG
	vec4 BG = space_bg;
	gl_FragColor = BG;
	vec2 inv_fragcoord = vec2(FRAGCOORD.x * SCREEN_PIXEL_SIZE.x, FRAGCOORD.y * SCREEN_PIXEL_SIZE.y);
	vec2 travel1 = TIME * nebula_speed / 15.0;
	vec4 nebula1 = texture(noise_texture, inv_fragcoord + travel1);
	vec2 travel2 = TIME * -nebula_speed / 25.0;
	vec4 nebula2 = texture(noise_texture, inv_fragcoord + travel2);
	gl_FragColor += nebula1 * nebula_colour1 * nebula_visibility+ nebula2 * nebula_colour2 * nebula_visibility;
	// Stars
	vec2 star_travel = star_speed * TIME;
	for (int i = 1; i <= iterations; i++) {
		float size = star_size / float(i);
		vec2 pos = vec2(floor((1.0 / size * FRAGCOORD.x) + star_travel.x), floor((1.0 / size * FRAGCOORD.y) + star_travel.y));
		float star_value = rand(pos, float(star_seed));
		if (star_value > star_frequency) {
			vec2 center = size * pos + vec2(size, size) * 0.5;
			vec2 modifiedCoords = vec2(FRAGCOORD.x + size * star_travel.x, FRAGCOORD.y + size * star_travel.y);
			float color_f = star_brightness / float(i);
			color_f = color_f / clamp(distance(modifiedCoords.y, center.y), 0.5, size / 2.0 - 1.0);
			color_f = color_f / clamp(distance(modifiedCoords.x, center.x), 0.5, size / 2.0 - 1.0);
			float color_value = (star_value - star_frequency) / (1.0 - star_frequency);
			vec4 colormapA = texture(gradientA, vec2(color_value));
			gl_FragColor += colormapA * color_f;
		}
	}
}
```

### WebGL shader

```
#version 300 es
precision highp float;

out vec4 outColor;

uniform float TIME;
uniform vec2 SCREEN_PIXEL_SIZE;

const vec4 bg_space_bg = vec4(0.05, 0.0, 0.2, 1.0);
const vec4 bg_nebula_colour1 = vec4(0.2, 0.0, 0.5, 1.0);
const vec4 bg_nebula_colour2 = vec4(0.4, 0.0, 0.3, 1.0);
const float bg_nebula_visibility = 0.3;
const vec2 bg_nebula_speed = vec2(0.2, 1);

uniform sampler2D bg_noise_texture;

uniform int star_seed;

const int iterations = 2;
const float star_frequency = 0.90;
const vec2 star_speed = vec2(-0.1, -0.2);
const float star_brightness = 0.85;

const float star_size = 120.0;

float rand(vec2 st, float seed) {
  return fract(sin(dot(st.xy, vec2(seed + 12.9898, 78.233))) * 43758.5453123);
}

vec4 star_gradient(float rand) {
    if (rand < 0.2) {
        return vec4(0.65, 0.03, 0.55, 1.0);
    }
    else if (rand < 0.6) {
        return vec4(0.55, 0.35, 0.05, 1.0);
    }
    else {
        return vec4(0.15, 0.50, 0.17, 1.0);
    }
}

void main() {
  // BG
  vec4 BG = bg_space_bg;
  outColor = BG;
  vec2 inv_fragcoord = vec2(gl_FragCoord.x / SCREEN_PIXEL_SIZE.x, gl_FragCoord.y / SCREEN_PIXEL_SIZE.y);
  vec2 travel1 = TIME * bg_nebula_speed / 15.0;
  vec4 nebula1 = texture(bg_noise_texture, inv_fragcoord + travel1);
  vec2 travel2 = TIME * -bg_nebula_speed / 25.0;
  vec4 nebula2 = texture(bg_noise_texture, inv_fragcoord + travel2);
  outColor += nebula1 * bg_nebula_colour1 * bg_nebula_visibility+ nebula2 * bg_nebula_colour2 * bg_nebula_visibility;
  // Stars
    // Inspiration: https://godotshaders.com/shader/star-shaders-v2-0/
  vec2 star_travel = star_speed * TIME;
  for (int i = 1; i <= iterations; i++) {
    float size = star_size / float(i);
    vec2 pos = vec2(floor((1.0 / size * gl_FragCoord.x) + star_travel.x), floor((1.0 / size * gl_FragCoord.y) + star_travel.y));
    float star_value = rand(pos, float(star_seed));
    if (star_value > star_frequency) {
      vec2 center = size * pos + vec2(size, size) * 0.5;
      vec2 modifiedCoords = vec2(gl_FragCoord.x + size * star_travel.x, gl_FragCoord.y + size * star_travel.y);
      float color_f = star_brightness / float(i);
      color_f = color_f / clamp(distance(modifiedCoords.y, center.y), 0.5, size / 2.0 - 1.0);
      color_f = color_f / clamp(distance(modifiedCoords.x, center.x), 0.5, size / 2.0 - 1.0);
      float color_value = (star_value - star_frequency) / (1.0 - star_frequency);
      vec4 colormapA = star_gradient(color_value);
      outColor += colormapA * color_f;
    }
  }
}
```
