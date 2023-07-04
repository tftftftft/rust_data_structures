// pub struct Node<T> {
//     value: T,
//     next: Option<Box<Node<T>>>,
// }

// // Define the structure of the linked list
// pub struct SinglyLinkedList<T> {
//     head: Option<Box<Node<T>>>,
// }

// impl<T: std::fmt::Display + PartialEq> SinglyLinkedList<T> {
//     // Creates a new empty list
//     pub fn new() -> SinglyLinkedList<T> {
//         SinglyLinkedList { head: None }
//     }

//     pub fn push(&mut self, value: T){
//         let new_node = Box::new(Node {
//             value: value,
//             next: self.head.take(),
//         });
//         self.head = Some(new_node);
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         match self.head.take() {
//             None => None,
//             Some(mut old_head) => {
//                 self.head = old_head.next.take();
//                 Some(old_head.value)
//             }
//         }
//     }

//     pub fn replace_value(&mut self, value_to_replace: T, replace_value: &T)
//     where
//         T: Clone,
//     {
//         let mut current_node = self.head.as_mut(); // mutable reference
    
//         while let Some(node) = current_node {
//             if node.value == value_to_replace{
//                 node.value = replace_value.clone(); // clone the replace_value
//             };
//             current_node = node.next.as_mut(); // move to the next node
//         };
//     }
    
//     pub fn display_list(&self){
//         let mut current_node = &self.head;
//         while let Some(node) = current_node{
//             println!("{}", node.value);
//             current_node = &node.next;
//         } 
//     }

//     pub fn is_present(&self, value: T) -> bool{
//         let mut current_node = &self.head;
//         while let Some(node) = current_node {
//             if node.value == value{
//                 return true;
//             } else {
//                 current_node = &node.next;
//             };
//         };
//         false
//     }
// }

// fn main(){
//     let mut list:SinglyLinkedList<i32> = SinglyLinkedList::new();

//     list.push(1);
//     list.push(2);
//     list.push(3);
//     // list.pop();
//     list.replace_value(3, &666);
   

//     list.display_list();
//     println!("{:?}", list.is_present(1));
// }

fn main(){
    let n = 3;
    let mut nums1: Vec<i32> = vec![1,0,2,3,0,0,0];
    for i in 0..nums1.len(){
        println!("{}", i);
    }
    nums1.remove(2);
println!("{:?}", nums1);
}
