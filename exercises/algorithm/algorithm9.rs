/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM NO

use std::cmp::Ord;
use std::default::Default;
use std::fmt::Debug;
#[derive(Debug)]
pub struct Heap<T>
where
    T: Default + Debug,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default+ Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 坑逼作者，给个空值占位😓
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()-1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 1
    }

    pub fn add(&mut self, value: T) {
        //TODO
        if self.items.len()+1 ==1 { //没错
            self.items.push(value);
            return;
        }
        self.items.push(value);
        let mut idx = self.len() ;
        let mut parent_id = self.parent_idx(idx);
        let mut_ptr = self.items.as_mut_ptr();
        // 如果还有其他元素，要将该元素上浮到对应为止
        while idx>= parent_id {
            parent_id = self.parent_idx(idx); // 需要在写一份 动态改变
            if (self.comparator)(&self.items[idx],&self.items[parent_id]) {
                // 如果子节点的优先级高 需要与父节点交换
                unsafe {
                    std::ptr::swap(mut_ptr.offset(idx as isize),mut_ptr.offset(parent_id as isize)) ; 
                }
                idx = parent_id;
            }else { // 当达到最优先的时候 或者合适的位置比较器比较不出true了 中断即可
                break;
            }
        } 
    }
    // 返回父节点id
    fn parent_idx(&self, idx: usize) -> usize {
        if idx == 1 {
            return 1;
        }
        idx / 2
    }
    // 这是什么意思？？？？像是判断该节点有没有子节点
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.len()
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        if self.left_child_idx(idx) + 1 <=self.len() {
            self.left_child_idx(idx)+1
        }else {
            self.left_child_idx(idx)
        }
    }
    // 如果左孩子小 返回左孩子id 否则返回右孩子id  但是这样写还需要判断要不要下浮 就不用了
    // fn smallest_child_idx(&self, idx: usize) -> usize {
    //     //TODO
	// 	if idx*2<=self.len() && idx*2+1<=self.len() {
    //         // 两个子节点都存在 选择优先级高的
    //         if (self.comparator)(&self.items[idx*2] ,&self.items[idx*2+1]) {
    //             idx *2
    //         }else {
    //             idx*2+1
    //         }
    //     } else  {
    //         // 默认左子节点存在
    //         idx*2
    //     }
    // }
    fn next_elem(&mut self) -> Option<T> {
        // 空元素堆的next返回None
        // print!("打印{:?}",self.items.len());
        // println!("打印长度:{:?},{:?}", self.len(),self.items);
        if self.len()+1 == 1 { // goushi
            return None;
        }
        let mut currnet = 1;
        let mut end  = self.len(); // 获取最后一个元素的坐标
        // println!("{:?}",end);
        // 获取items元素的裸指针
        let mut_ptr = self.items.as_mut_ptr();
        unsafe {
            std::ptr::swap(mut_ptr.offset(currnet as isize),mut_ptr.offset(end as isize)) ; //这个是让指向的元素交换
            // println!("{:?}",self.items);
            //将最后一个元素和第一个元素交换 方便调用pop()方法 将优先级最高的元素出栈  
            // 同时让这个元素重新下浮 根据算法让正确的优先级的子节点正确上浮 这样就只使用了O(logn)完成了交换并保持平衡
            let pop_elem = self.items.pop();// 要将pop的元素返回 Some(elem)
            // println!("返回的值{:?}",pop_elem);
            while self.children_present(currnet) { // 如果current其存在子节点 这里用while是因为需要改变current来一直交换，使得首个元素下沉
                let lc = self.left_child_idx(currnet);
                let rc = self.right_child_idx(currnet); // 这里要判断越界所以建议调方法
                match ((self.comparator)(&self.items[currnet],&self.items[lc]),(self.comparator)(&self.items[currnet],&self.items[rc])) {
                    (true,true) => break, //当前节点优先级比子节点都高 不要下浮了
                    (true,false) => {
                        // 右节点优先级比当前节点高
                        std::ptr::swap(mut_ptr.offset(currnet as isize),mut_ptr.offset(rc as isize)) ; //也可以封装下
                        currnet = rc;// 交换指针（下标）
                    }
                    (false,true) => {
                        // 左节点优先级比当前节点高
                        std::ptr::swap(mut_ptr.offset(currnet as isize),mut_ptr.offset(lc as isize)) ; //也可以封装下
                        currnet = lc;// 交换指针（下标）
                    }
                    (false,false) => {
                        // 需要选一个优先级高的
                        if (self.comparator)(&self.items[lc],&self.items[rc]) {
                            // 左节点优先级高
                            std::ptr::swap(mut_ptr.offset(currnet as isize),mut_ptr.offset(lc as isize)) ; //也可以封装下
                            currnet = lc;// 交换指针（下标）
                        }else {
                             // 右节点优先级高
                             std::ptr::swap(mut_ptr.offset(currnet as isize),mut_ptr.offset(rc as isize)) ; //也可以封装下
                             currnet = rc;// 交换指针（下标）
                        }
                    }
                }
            };
            pop_elem
        }
        // None
    }
}

impl<T> Heap<T>
where
    T: Default + Ord+ Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b) // 传入匿名函数即可
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default+ Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		self.next_elem()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord+ Debug,
    {
        Heap::new(|a, b| a < b) // 从小到大
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord+ Debug,
    {
        Heap::new(|a, b| a > b)// 从大到小
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        println!("打印数组:{:?}", heap);
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        println!("打印小数组:{:?}", heap);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        println!("打印大数组:{:?}", heap);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}