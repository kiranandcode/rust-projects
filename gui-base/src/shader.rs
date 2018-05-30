use sdl2::event::Event;
use gl::{GLuint, GLfloat, GLboolean, GLchar, GLsizei};

pub struct Shader {
    id : GLuint
}

impl Shader {
    pub fn new() -> Self {

        Shader {
            id: 0
        }
    }
    pub fn setActive(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }


    pub fn compile(&mut self,
                   vertexSource : &[GLchar],
                   fragmentSource : &[GLchar]) {
        sVertex: GLuint;
        sFragment : GLuint;
        gShader : GLuint;

        sVertex = gl::glCreateShader(gl::VERTEX_SHADER);
        glShaderSource(sVertex, 1 as GLsizei, &vertexSource)

    }

    pub fn setFloat(&mut self, value : GLfloat, useShader : bool) {}
    pub fn setInt(&mut self, value : GLuint, useShader : bool) {}
    pub fn setVector2f(&mut self, value : &[GLfloat; 2], useShader : GLboolean) {}
    pub fn setVector3f(&mut self, value : &[GLfloat; 3], useShader : GLboolean) {}
    pub fn setMatrix4(&mut self, value : &[GLfloat; 16], useShader : GLboolean) {}


    fn checkCompileErrors(&mut self) {}
}

