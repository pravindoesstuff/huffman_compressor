// Implemented as a MaxHeap
#[derive(Debug)]
pub struct Heap<T> {
    heap: Vec<T>,
}

impl<T: std::cmp::PartialOrd> Heap<T> {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }
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
    pub fn insert(&mut self, value: T) {
        self.heap.push(value);
        self.bubble_up(self.heap.len() - 1)
    }
    fn bubble_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }
        let parent_index = Self::parent(index);
        if self.heap[index] > self.heap[parent_index] {
            self.heap.swap(index, parent_index);
            self.bubble_up(parent_index);
        }
    }
    fn pop(&mut self, index: usize) {}
    fn sift_down(&mut self, index: usize) {}
}
