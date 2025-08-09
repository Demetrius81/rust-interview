#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Создаёт новый пустой стек
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    /// Кладёт элемент на вершину стека
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Снимает элемент с вершины стека
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Смотрит элемент на вершине стека без снятия
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    /// Проверяет, пуст ли стек
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Возвращает количество элементов
    fn size(&self) -> usize {
        self.items.len()
    }
}

pub fn run() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("Satck: {:?}", stack);
    println!("Top el: {:?}", stack.peek());
    println!("Size: {}", stack.size());

    println!("Pop: {:?}", stack.pop());
    println!("After pop: {:?}", stack);
}
