extern crate presort;

use presort::PresortedVec;

struct TreeNode<T> {
    data: T,
    children: Vec<Tree<T>>,
}

type Tree<T> = Box<TreeNode<T>>;

fn tree<T>(data: T, children: Vec<Tree<T>>) -> Tree<T> {
    Box::new(TreeNode { data: data, children: children })
}

fn dump<T: Clone+Ord>(tree: &Tree<T>, vec: &mut PresortedVec<T>) {
    vec.push(tree.data.clone());
    for ref kid in &tree.children { dump(kid, vec); }
}

#[test]
fn test_tree() {
    let tree = tree(37, vec![]);
    let mut vec = PresortedVec::new();
    dump(&tree, &mut vec);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&37]);
}