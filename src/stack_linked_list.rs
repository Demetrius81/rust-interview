#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Stack<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Stack<T> {
    /// Создать пустой стек
    fn new() -> Self {
        Stack {
            head: None,
            size: 0,
        }
    }

    /// Положить элемент на вершину стека
    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(), // забираем текущее head
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    /// Снять элемент с вершины стека
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.value
        })
    }

    /// Посмотреть элемент на вершине без снятия
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    /// Проверка на пустоту
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Получить размер стека
    fn size(&self) -> usize {
        self.size
    }
}

pub fn run() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("Верхний элемент: {:?}", stack.peek());
    println!("Размер: {}", stack.size());

    println!("Pop: {:?}", stack.pop());
    println!("После pop: {:?}", stack.pop());
    println!("После второго pop: {:?}", stack.pop());
    println!("Пустой? {}", stack.is_empty());
}
