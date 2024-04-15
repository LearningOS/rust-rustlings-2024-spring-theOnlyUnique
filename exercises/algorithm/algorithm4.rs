/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Clone,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let v = value.clone();
        //TODO
        if self.search(v) == true {
            return ;
        }
        // 如果一个二叉搜索树指向的根节点为空
        if self.root.is_none() {
            // 将数据插入到根节点
            self.root.replace(Box::new(TreeNode {
                value,
                left:None,
                right:None,
            }));
        }
        else {
            // 否则递归 指向的最末端的TreeNode进行insert 这里我直接往一边插入
            // as_derf_mut 获取Box包裹的TreeNode
            self.root.as_deref_mut().unwrap().insert(value);
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut new_ptr = &self.root;
        while let Some(x) = new_ptr {
            if (x.value == value ){
                return true;
            }else {
                new_ptr = &x.right;
            }
        }
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        // 往最右边找 找到None就插入
        match &mut self.right {
            Some(ref mut r) => r.insert(value),
            None => {
                self.right.replace(Box::new(TreeNode {
                    value,
                    left:None,
                    right:None,
                }));
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


