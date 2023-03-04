// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>
};
@group(1) @binding(0) // 1.
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>
}

struct InstanceInput {
    @location(3) instance_matrix_1: vec4<f32>,
    @location(4) instance_matrix_2: vec4<f32>,
    @location(5) instance_matrix_3: vec4<f32>,
    @location(6) instance_matrix_4: vec4<f32>,
    @location(7) texture_index: u32
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) texture_index: u32
}

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    let world_position = vec4<f32>(model.position, 1.0);

    let model_matrix = mat4x4<f32>(
            instance.instance_matrix_1,
            instance.instance_matrix_2,
            instance.instance_matrix_3,
            instance.instance_matrix_4,
    );

    var out: VertexOutput;
    out.clip_position = camera.view_proj * model_matrix * world_position;
    out.tex_coords = model.tex_coords;
    out.texture_index = instance.texture_index;
    return out;
}


// Fragment Shader

@group(0) @binding(0)
var s_diffuse: sampler;
@group(0) @binding(1)
var t_diffuse: binding_array<texture_2d<f32>, 3>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse[in.texture_index], s_diffuse, in.tex_coords);
}