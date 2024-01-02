use crate::node::Node; 
pub struct Stack<T>
{
    top:Option<Node<T>>,
    size: i32,
}

impl<T> Stack<T>
{
    //constructor
      // Constructor
      pub fn new() -> Stack<T> {
        Stack { top: None, size: 0 }
    }

    /// This returns the size of the stack
    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn is_empty(&self) -> bool
    {
        return self.size == 0;
    }

    //This takes and element and add it to the top
    pub fn push(&mut self, element: T) {
        let node = Node{
            value:element,
            next:self.top.take()
        };

        self.top = Some(Box::new(node));
        self.size+=1;

    }

    pub fn pop(&mut self) -> T
    {
        let value:Node<T> = self.top.take().value;

        self.top = value.next.unrwap();
        self.size-=1;
        return value.value;
    }
}