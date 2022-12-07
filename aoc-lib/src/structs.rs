use std::ops::Index;

type NodeIndex = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct BidirectionalTree<T> {
    nodes: Vec<TreeNode<T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode<T> {
    pub data: T,
    pub index: NodeIndex,
    parent_index: Option<NodeIndex>,
    children: Vec<NodeIndex>,
}

impl<T> BidirectionalTree<T> {
    pub fn new(root_data: T) -> Self {
        Self {
            nodes: vec![TreeNode {
                index: 0,
                parent_index: None,
                data: root_data,
                children: Vec::new(),
            }]
        }
    }

    pub fn root(&self) -> &TreeNode<T> {
        &self.nodes[0]
    }

    pub fn index_of_parent(&self, node_index: NodeIndex) -> Option<NodeIndex> {
        self.nodes[node_index].parent_index.map(|index| self.nodes[index].index)
    }


    pub fn children_of(&self, node_index: NodeIndex) -> Vec<&TreeNode<T>> {
        self.nodes[node_index].children.iter().map(|&index| &self.nodes[index]).collect()
    }

    pub fn insert(&mut self, parent_index: NodeIndex, data: T) -> &TreeNode<T> {
        let node_index = self.nodes.len();
        self.nodes[parent_index].children.push(node_index);

        let node = TreeNode {
            index: node_index,
            parent_index: Some(parent_index),
            data,
            children: Vec::new(),
        };

        self.nodes.push(node);
        &self.nodes[node_index]
    }
}

impl<T> Index<NodeIndex> for BidirectionalTree<T> {
    type Output = TreeNode<T>;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.nodes[index]
    }
}