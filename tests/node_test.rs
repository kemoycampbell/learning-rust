use learning_rust::node::Node;

#[cfg(test)]
mod tests
{
    use super::*;


    #[test]
    fn test_node_creation()
    {
        let node = Node{
            value:42,
            next:None,
        };

        assert_eq!(node.value, 42);
        assert!(node.next.is_none())
    }

    #[test]
    fn test_linked_node()
    {
        let node = Node{
            value:42,
            next:None,
        };

        let node2 = Node{
            value:45,
            next:Some(Box::new(node))
        };

        assert_eq!(node2.value, 45);
        assert!(node2.next.is_some());
        assert_eq!(node2.next.unwrap().value, 42);
    }




}