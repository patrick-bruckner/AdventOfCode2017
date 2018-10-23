use std::ops::Deref;
use std::ops::DerefMut;

const INPUT: u32 = 361527;

#[derive(Debug)]
struct Arena {
    nodes: Vec<Tile>,
    position: usize
}

#[derive(Debug)]
struct Tile {
    val: u32,
    up_left: Option<usize>,
    up: Option<usize>,
    up_right: Option<usize>,
    right: Option<usize>,
    down_right: Option<usize>,
    down: Option<usize>,
    down_left: Option<usize>,
    left: Option<usize>
}

impl Arena {
    fn new() -> Arena {
        let mut a = Arena {nodes: vec![], position: 0};
        a.add_node();
        return a;
    }

    fn add_node(&mut self) {
        let next_node = self.nodes.len();

        self.nodes.push(Tile {
            val: 0,
            up_left: None,
            up: None,
            up_right: None,
            right: None,
            down_right: None,
            down: None,
            down_left: None,
            left: None
        });

        self.position = next_node;
    }

    fn previous(&mut self) -> &mut Tile {
        &mut self.nodes[self.position - 1]
    }

    fn previous_idx(&self) -> usize {
        self.position - 1
    }
}

impl Deref for Arena {
    type Target = Tile;

    fn deref(&self) -> &Tile {
        &self.nodes[self.position]
    }
}

impl DerefMut for Arena {
    fn deref_mut(&mut self) -> &mut Tile {
        &mut self.nodes[self.position]
    }
}

fn check_match(check: Tile) {
    if check.val > INPUT {
        print!("{}", check.val);
    }
}

fn main() {
    let mut graph = Arena::new();
    graph.val = 1;
    loop {
        // right
        graph.add_node();
        graph.val += graph.previous().val;
        graph.up_left = graph.previous().up;
        graph.left = Some(graph.previous_idx());
    }
    // let mut cur_tile = Box::into_raw(Box::new(Tile::new()));
    // (*cur_tile).val = 1;
    //
    // loop {
    //     // right
    //     let mut new_tile = Box::into_raw(Box::new(Tile::new()));
    //     (*new_tile).val += (*cur_tile).val;
    //     (*new_tile).up_left = (*cur_tile).up;
    //     (*new_tile).left = Some(cur_tile);
    //     cur_tile = new_tile;
    // }
}
