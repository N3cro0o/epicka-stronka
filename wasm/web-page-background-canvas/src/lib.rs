use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

mod js;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("bg-canvas").unwrap();
    let mut canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let screen_size = js::get_page_size().unwrap();
    canvas.set_width(screen_size.0);
    canvas.set_height(screen_size.1);

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
 
        in vec4 position;

        void main() {
        
            gl_Position = position;
        }
        "##,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
        precision highp float;

        out vec4 outColor;

        uniform float TIME;
        uniform vec2 SCREEN_PIXEL_SIZE;

        const vec4 bg_space_bg = vec4(0.05, 0.0, 0.2, 1.0);
        const vec4 bg_nebula_colour1 = vec4(0.2, 0.0, 0.5, 1.0);
        const vec4 bg_nebula_colour2 = vec4(0.4, 0.0, 0.3, 1.0);
        const float bg_nebula_visibility = 0.2;
        const vec2 bg_nebula_speed = vec2(0.1, 1);

        uniform sampler2D bg_noise_texture;

        const int star_seed = 0;
        const int iterations = 2;
        const float star_frequency = 0.99;
        const vec2 star_speed = vec2(-0.1, 0.2);
        const float star_brightness = 0.85;

        const float star_size = 100.0;

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
          vec2 inv_fragcoord = vec2(gl_FragCoord.x * SCREEN_PIXEL_SIZE.x, gl_FragCoord.y * SCREEN_PIXEL_SIZE.y);
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
        "##,
    )?;

    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices: [f32; 18] = [
        -1.0, -1.0, 0.0,
        -1.0, 1.0, 0.0,
        1.0, -1.0, 0.0,
        -1.0, 1.0, 0.0,
        1.0, -1.0, 0.0,
        1.0, 1.0, 0.0
    ];

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.bind_vertex_array(Some(&vao));

    let vert_count = (vertices.len() / 3) as i32;
    draw(&context, vert_count);

    Ok(())
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(context: &WebGl2RenderingContext, vert_shader: &WebGlShader, frag_shader: &WebGlShader,) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
