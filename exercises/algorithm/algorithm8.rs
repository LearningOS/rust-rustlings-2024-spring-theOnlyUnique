/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
// I AM NO

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) { // 末尾加入一个元素
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> { // 如果vec非空 删除首个元素
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> { // 返回首个元素
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	myvec:Vec<T>,
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
            myvec:Vec::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        self.myvec.push(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        if let Some(v) = self.myvec.pop() {
             Ok(v)
        }else {
            Err("Stack is empty")
        }
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        if self.myvec.len() == 0 { true}
        else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}