use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub is_leaf: bool,
    pub top_left: Option<Rc<RefCell<Node>>>,
    pub top_right: Option<Rc<RefCell<Node>>>,
    pub bottom_left: Option<Rc<RefCell<Node>>>,
    pub bottom_right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32, is_leaf: bool) -> Self {
        Node {
            val,
            is_leaf,
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
        }
    }
}

fn construct(grid: Vec<Vec<i32>>) -> Option<Rc<RefCell<Node>>> {
    fn dfs(grid: &Vec<Vec<i32>>, top: usize, left: usize, size: usize) -> Option<Rc<RefCell<Node>>> {
        let val = grid[top][left];
        let mut all_same = true;

        for i in top..top + size {
            for j in left..left + size {
                if grid[i][j] != val {
                    all_same = false;
                    break;
                }
            }
            if !all_same {
                break;
            }
        }

        if all_same {
            Some(Rc::new(RefCell::new(Node::new(if val == 1 { 1 } else { 0 }, true))))
        } else {
            let half = size / 2;
            let mut node = Node::new(1, false);
            node.top_left = dfs(grid, top, left, half);
            node.top_right = dfs(grid, top, left + half, half);
            node.bottom_left = dfs(grid, top + half, left, half);
            node.bottom_right = dfs(grid, top + half, left + half, half);
            Some(Rc::new(RefCell::new(node)))
        }
    }

    dfs(&grid, 0, 0, grid.len())
}

fn main() {
    let grid = vec![vec![1, 1], vec![1, 0]];
    let root = construct(grid);
    if let Some(r) = root {
        println!("{} {}", r.borrow().val, r.borrow().is_leaf);
    }
}
