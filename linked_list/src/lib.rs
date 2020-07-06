use std::cell::RefCell;
use std::rc::Rc;

/*struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>  
}*/

struct TransactionLog {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub length: u64
}

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node{
    value: String,
    next: SingleLink,
}

impl Node {
    //La forma corta de crear un nodo
    fn new(value: String) -> Rc<RefCell<Node>>{
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

impl TransactionLog{
    //Se crea una lista vacia
    pub fn new_empty() -> TransactionLog{
        TransactionLog {
            head: None,
            tail: None,
            length: 0
        }
    }
}

pub fn push(&mut self, value: String){
    let new = Node::new(value);
    match self.tail.take(){
        Some(old) => old.borrow_mut().next = Some(new.clone()),
        None => self.head = Some(new.clone())
    };
    self.length += 1;
    self.tail = Some(new);
}

pub fn pop(&mut self) -> Option<String>{
    self.head.take().map(|head|{
        if let Some(next) = head.borrow_mut().next.take(){
            self.head = Some(next);
        }else {
            self.tail.take();
        }
        self.length -=1;
        Rc::try_unwrap(head)
        .ok()
        .expect("Algo ha ido mal")
        .into_inner()
        .value()
    })
}
