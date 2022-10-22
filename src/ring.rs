pub struct Ring<T> {
    vec: Vec<T>,
    ptr: usize
}

impl<T: Clone> Ring<T> {
    pub fn new(len: usize, def: T) -> Self {
        Ring {
            vec: vec![def; len],
            ptr: 0
        }
    }
    pub fn push(&mut self, input: T) {
        self.vec[self.ptr] = input;
        self.ptr += 1;
        if self.ptr >= self.vec.len() {
            self.ptr = 0;
        }
    }
}

pub struct RingIter<'a, T> {
    ptr: usize,
    ring: &'a Ring<T>
}

impl<'a, T: Copy> Iterator for RingIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr += 1;
        if self.ptr == self.ring.vec.len() {
            self.ptr = 0;
        }
        if self.ptr == self.ring.ptr {
            return Option::None;
        }
        Option::Some(self.ring.vec[self.ptr])
    }
}

impl<T> Ring<T> {
    pub fn iter(&self) -> RingIter<T> {
        RingIter { ptr : self.ptr, ring : self }
    }
}
