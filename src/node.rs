/// A node in the tree, generic over the value type it holds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node<T> {
    pub id: usize,
    pub value: T,
}

impl<T> Node<T> {
    pub fn new(id: usize, value: T) -> Self {
        Node { id, value }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }
}
