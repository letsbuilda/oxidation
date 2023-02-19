use oxi_engine::graphics::window::Window;
use oxi_engine::graphics::gl_wrapper;

use std::ptr;

use gl::types::*;

fn main() {
    let mut window = Window::new(800, 600, "Oxidation");
    
    let vertices: [f32; 12] = [
        0.5, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0, // top left
    ];
    let indices: [i32; 6] = [
        0, 1, 3, // first triangle
        1, 2, 3, // second triangle
    ];
    window.init_gl();
    let vao = gl_wrapper::Vao::new();
    vao.bind();

    let vbo = gl_wrapper::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.buffer_data_f32(&vertices);

    let ebo = gl_wrapper::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();

    ebo.buffer_data_i32(&indices);

    let position_attribute = gl_wrapper::VertexAttribute::new(
        0, 
        3, 
        gl::FLOAT, 
        gl::FALSE,
        3 * std::mem::size_of::<GLfloat>() as GLsizei,
        ptr::null());
    
    let index_attr = gl_wrapper::VertexAttribute::new(
        1, 
        3, 
        gl::FLOAT, 
        gl::FALSE,
        3 * std::mem::size_of::<GLfloat>() as GLsizei,
        ptr::null());
    
    position_attribute.enable();
    index_attr.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        window.update();
    }
}