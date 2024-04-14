/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> { // 链表节点
    val: T,
    next: Option<NonNull<Node<T>>>, // 指向下一个的指针
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> 
where
    T: PartialOrd+Clone,
{
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,

}

impl<T> Default for LinkedList<T> 
where
    T: PartialOrd+Clone,
{
    fn default() -> Self {
        Self::new()
    }

}

impl<T> LinkedList<T> 
where
    T: PartialOrd+Clone,
{
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) { 
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) }); // 获取新建节点的指针
        println!("打印节点指针 {:?}",node_ptr);
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr }, // 如果非空 末尾指针的下一个指向该节点
        }
        self.end = node_ptr; // 更新末尾指针
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> { // 获取链表的第index个数字
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1), // 从下一个指针开始 取第index-1个数
            },
        }
    }
    
	pub fn merge(mut list_a:LinkedList<T>,mut list_b:LinkedList<T>) -> Self
	{
        let mut res = Self {
            length: 0,
            start: None,
            end: None,
        };
        let mut indexa:i32 = 0;
        let mut indexb:i32 = 0;
        
        while indexa < list_a.length as i32 || indexb <list_b.length as i32 {
            match (list_a.get(indexa),list_b.get(indexb)) {
                // 四种匹配模式
                (None,None) => return res,
                (Some(v1),None) => {
                    res.add(v1.clone());
                    indexa +=1;
                },
                (None,Some(v2)) => {
                    res.add(v2.clone());
                    indexb +=1;
                },
                (Some(v1),Some(v2)) =>{
                    // 从小到大 小的在前面
                    if v1 > v2 {
                        res.add(v2.clone());
                        indexb +=1;
                    }else {
                        res.add(v1.clone());
                        indexa +=1;
                    }
                }
                
            }
        }
        res
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display + PartialOrd+Clone,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}