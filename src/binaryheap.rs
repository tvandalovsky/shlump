pub struct BinaryHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> BinaryHeap<T> {
    pub fn new() -> BinaryHeap<T> {
        BinaryHeap { data: Vec::new() }
    }

    fn parent(&mut self, i: usize) -> usize {
        let f_index = (((i - 1) / 2) as f64).floor() as usize;
        return f_index;
    }

    fn left(&mut self, i: usize) -> usize {
        let l_index = (2 * i) + 1;
        return l_index;
    }

    fn right(&mut self, i: usize) -> usize {
        let r_index = (2 * i) + 2;
        return r_index;
    }

    pub fn peek(&mut self) -> Option<&T> {
        if self.data.len() == 0 {
            return None;
        }
        return Some(&self.data[0]);
    }

    // pub fn pop() -> Option<T> {

    // }

    // pub fn push(&mut self, value: T) {

    // }

    fn shift_down(&mut self, i: usize) {

    }

    fn shift_up(&mut self, i: usize) {

    }
}
