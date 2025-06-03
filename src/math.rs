#![allow(dead_code)]
    
pub const TWO_PI: f32 = std::f32::consts::PI * 2.;


pub fn pos_translation(matrix: &mut vecmath::Matrix4<f32>, position: vecmath::Vector3<f32>) 
{
    matrix[3][0] = position[0]; // x
    matrix[3][1] = position[1]; // y
    matrix[3][2] = position[2]; // z
}

pub fn scale_translation(matrix: &mut vecmath::Matrix4<f32>, position: vecmath::Vector3<f32>) 
{
    matrix[0][0] = position[0]; // x
    matrix[1][1] = position[1]; // y
    matrix[2][2] = position[2]; // z
}

pub fn rotation_y(angle_rad: f32) -> [[f32; 4]; 4] 
{
    let cos = angle_rad.cos();
    let sin = angle_rad.sin();

    [[  cos,  0.0,  sin,  0.0],
     [  0.0,  1.0,  0.0,  0.0],
     [ -sin,  0.0,  cos,  0.0],
     [  0.0,  0.0,  0.0,  1.0],]
}

pub fn rotation_x(angle_rad: f32) -> [[f32; 4]; 4] 
{
    let cos = angle_rad.cos();
    let sin = angle_rad.sin();

    [[  1.0,  0.0,  0.0,  0.0],
     [  0.0,  cos, -sin,  0.0],
     [  0.0,  sin,  cos,  0.0],
     [  0.0,  0.0,  0.0,  1.0],]
}

pub fn rotation_z(angle_rad: f32) -> [[f32; 4]; 4] 
{
    let cos = angle_rad.cos();
    let sin = angle_rad.sin();

    [[  cos, -sin,  0.0,  0.0],
     [  sin,  cos,  0.0,  0.0],
     [  0.0,  0.0,  1.0,  0.0],
     [  0.0,  0.0,  0.0,  1.0],]
}

