use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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
    let n = grid.len();
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((0, 0, n));
    let mut root: Option<Rc<RefCell<Node>>> = None;

    while let Some((top, left, size)) = queue.pop_front() {
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

        let node = Rc::new(RefCell::new(Node::new(if val == 1 { 1 } else { 0 }, all_same)));

        if root.is_none() {
            root = Some(Rc::clone(&node));
        }

        if !all_same {
            let half = size / 2;
            queue.push_back((top, left, half));
            queue.push_back((top, left + half, half));
            queue.push_back((top + half, left, half));
            queue.push_back((top + half, left + half, half));
        }
    }

    root
}

fn main() {
    let grid = vec![vec![1, 1], vec![1, 0]];
    let root = construct(grid);
    if let Some(r) = root {
        println!("{} {}", r.borrow().val, r.borrow().is_leaf);
    }
}
