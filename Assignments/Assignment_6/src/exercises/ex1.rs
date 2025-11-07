use std::fmt::Debug;

#[derive(Debug)]
pub struct TreeNode<T: Debug + PartialOrd + Clone> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Debug + PartialOrd + Clone> TreeNode<T> {
  pub fn new(value: T) -> TreeNode<T> {
        TreeNode{
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        if value < self.value {
            match self.left {
                None => { self.left = Some(Box::new(TreeNode::new(value))); },
                Some(ref mut left) => {left.insert(value); },
            }
        }else {
            match self.right {
                None => { self.right = Some(Box::new(TreeNode::new(value))); },
                Some(ref mut right) => { right.insert(value); },
            }
        }
    }

    pub fn from_vec(vec: &[T]) -> Self{
        let mut tree = TreeNode::new(vec[0].clone());
        for val in vec.iter().skip(1){
            tree.insert(val.clone());
        }
        tree
    }

    //left, root, right
    pub fn in_order(&self){

        if let Some(ref left) = self.left {
            left.in_order();
        }

        print!("{:?}, ", self.value);

        if let Some(ref right) = self.right {
            right.in_order();
        }

    }


    //root, left, right
    pub fn pre_order(&self){

        print!("{:?}, ", self.value);

        if let Some(ref left) = self.left {
            left.in_order();
        }

        if let Some(ref right) = self.right {
            right.in_order();
        }

    }

    //left, right, root
    pub fn post_order(&self){


        if let Some(ref left) = self.left {
            left.in_order();
        }

        if let Some(ref right) = self.right {
            right.in_order();
        }

        print!("{:?}, ", self.value);


    }

}
