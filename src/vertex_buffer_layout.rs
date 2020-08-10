use gl;

pub struct VertexBufferElement {
    pub el_type: u32,
    pub count: i32,
    pub normalized: bool,
}

impl VertexBufferElement {
    pub fn type_size(&self) -> i32 {
        match self.el_type {
            gl::FLOAT => 4,
            gl::UNSIGNED_INT => 4,
            gl::UNSIGNED_BYTE => 1,
            _ => unimplemented!(),
        }
    }
}

pub trait Push<T> {
    fn push(&mut self, _count: i32) {
        unimplemented!()
    }
}

pub struct VertexBufferLayout {
    elements: Vec<VertexBufferElement>,
    stride: i32,
}

impl VertexBufferLayout {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            stride: 0,
        }
    }
    //
    pub fn get_stride(&self) -> i32 {
        self.stride
    }
    //
    pub fn get_elements(&self) -> &[VertexBufferElement] {
        &self.elements
    }
}

impl VertexBufferLayout {
    pub fn push<T>(&mut self, count: i32) where Self: Push<T> {
        Push::<T>::push(self, count);
    }
}

impl Push<f32> for VertexBufferLayout {
    fn push(&mut self, count: i32) {
        let vbe = VertexBufferElement {
            el_type: gl::FLOAT,
            count: count,
            normalized: false,
        };
        self.stride += vbe.type_size() * count;
        self.elements.push(vbe);
    }
}

impl Push<u32> for VertexBufferLayout {
    fn push(&mut self, count: i32) {
        let vbe = VertexBufferElement {
            el_type: gl::UNSIGNED_INT,
            count: count,
            normalized: false,
        };
        self.stride += vbe.type_size() * count;
        self.elements.push(vbe);
    }
}

impl Push<u8> for VertexBufferLayout {
    fn push(&mut self, count: i32) {
        let vbe = VertexBufferElement {
            el_type: gl::UNSIGNED_BYTE,
            count: count,
            normalized: false,
        };
        self.stride += vbe.type_size() * count;
        self.elements.push(vbe);
    }
}
