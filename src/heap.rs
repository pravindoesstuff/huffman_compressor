pub struct Heap<T> {
    heap: Vec<T>,
}

impl<T: std::cmp::PartialOrd> Heap<T> {
    pub fn peek(&self) -> Option<&T> {
        self.heap.first()
    }
    fn left_child(index: usize) -> usize {
        2 * index + 1
    }
    fn right_child(index: usize) -> usize {
        2 * index
    }
    fn parent(index: usize) -> usize {
        index / 2
    }
    pub fn insert(&mut self, value: T) {}
    fn bubble_up(&mut self, index: usize) {}
    fn pop(&mut self, index: usize) {}
    fn sift_down(&mut self, index: usize) {}
}
