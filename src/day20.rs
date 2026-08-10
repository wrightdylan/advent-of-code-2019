use std::borrow::Cow;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Floor,
    Wall,
    Label(char),
    Portal(PortalInfo),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortalType {
    Inner,
    Outer,
    Entrance,
    Exit,
}

struct ScanTrack {
    start_x: i32,
    start_y: i32,
    dx: i32,
    dy: i32,
    length: i32,
    facing: Ortho,
    portal_type: PortalType,
}

#[derive(Debug)]
pub struct MazeData {
    pub ou: i32,
    pub ol: i32,
    pub ob: i32,
    pub or: i32,
    pub iu: i32,
    pub il: i32,
    pub ib: i32,
    pub ir: i32,
    pub grid: Grid<Tile>,
    pub portals: HashMap<([char; 2], PortalType), (i32, i32)>
}

impl MazeData {
    fn find_portals(&mut self) {
        let tracks = self.get_tracks();

        for track in &tracks {
            for i in 0..=track.length {
                let x = track.start_x + (i * track.dx);
                let y = track.start_y + (i * track.dy);

                if self.grid[(x, y)] == Tile::Wall {
                    continue;
                }

                let label_data = match track.facing {
                    Ortho::North => {
                        if let (Tile::Label(ch1), Tile::Label(ch2)) = (self.grid[(x, y - 2)], self.grid[(x, y - 1)]) {
                            Some(([ch1, ch2], Ortho::South))
                        } else { None }
                    }
                    Ortho::South => {
                        if let (Tile::Label(ch1), Tile::Label(ch2)) = (self.grid[(x, y + 1)], self.grid[(x, y + 2)]) {
                            Some(([ch1, ch2], Ortho::North))
                        } else { None }
                    }
                    Ortho::West => {
                        if let (Tile::Label(ch1), Tile::Label(ch2)) = (self.grid[(x - 2, y)], self.grid[(x - 1, y)]) {
                            Some(([ch1, ch2], Ortho::East))
                        } else { None }
                    }
                    Ortho::East => {
                        if let (Tile::Label(ch1), Tile::Label(ch2)) = (self.grid[(x + 1, y)], self.grid[(x + 2, y)]) {
                            Some(([ch1, ch2], Ortho::West))
                        } else { None }
                    }
                };

                if let Some((label, direction)) = label_data {
                    let portal_type = match label {
                        ['A', 'A'] => PortalType::Entrance,
                        ['Z', 'Z'] => PortalType::Exit,
                        _ => track.portal_type,
                    };
                    let info = PortalInfo {
                        label,
                        portal_type,
                        orientation: direction,
                        twin_node_idx: None,
                    };
                    
                    self.portals.insert((label, portal_type), (x, y));
                    self.grid[(x, y)] = Tile::Portal(info);
                }
            }
        }
    }

    fn get_tracks(&self) -> [ScanTrack; 8] {
        [
            // === OUTER BOUNDARIES ===
            // Outer Top
            ScanTrack { start_x: self.ol, start_y: self.ou, dx: 1, dy: 0, length: self.or - self.ol, facing: Ortho::North, portal_type: PortalType::Outer },
            // Outer Bottom
            ScanTrack { start_x: self.ol, start_y: self.ob, dx: 1, dy: 0, length: self.or - self.ol, facing: Ortho::South, portal_type: PortalType::Outer },
            // Outer Left
            ScanTrack { start_x: self.ol, start_y: self.ou, dx: 0, dy: 1, length: self.ob - self.ou, facing: Ortho::West, portal_type: PortalType::Outer },
            // Outer Right
            ScanTrack { start_x: self.or, start_y: self.ou, dx: 0, dy: 1, length: self.ob - self.ou, facing: Ortho::East, portal_type: PortalType::Outer },

            // === INNER BOUNDARIES ===
            // Inner Top
            ScanTrack { start_x: self.il, start_y: self.iu, dx: 1, dy: 0, length: self.ir - self.il, facing: Ortho::South, portal_type: PortalType::Inner },
            // Inner Bottom
            ScanTrack { start_x: self.il, start_y: self.ib, dx: 1, dy: 0, length: self.ir - self.il, facing: Ortho::North, portal_type: PortalType::Inner },
            // Inner Left
            ScanTrack { start_x: self.il, start_y: self.iu, dx: 0, dy: 1, length: self.ib - self.iu, facing: Ortho::East, portal_type: PortalType::Inner },
            // Inner Right
            ScanTrack { start_x: self.ir, start_y: self.iu, dx: 0, dy: 1, length: self.ib - self.iu, facing: Ortho::West, portal_type: PortalType::Inner },
        ]
    }

    fn graphify(&self) -> Graph {
        let mut graph = Graph::new();

        // Initial nodes
        graph.nodes = self.portals
            .values()
            .map(|coord| Node::new_from_coord(*coord, self))
            .collect();

        let mut nodes_dict: HashMap<(i32, i32), usize> = graph.nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.coord, index))
            .collect();

        // Cross link inner and outer portals
        for (label, p_type) in self.portals.keys() {
            if *p_type == PortalType::Inner {
                if let Some(&inner_coord) = self.portals.get(&(*label, PortalType::Inner)) {
                    if let Some(&outer_coord) = self.portals.get(&(*label, PortalType::Outer)) {
                        let idx_inner = nodes_dict[&inner_coord];
                        let idx_outer = nodes_dict[&outer_coord];
                        
                        if let Some(ref mut info) = graph.nodes[idx_inner].portal {
                            info.twin_node_idx = Some(idx_outer);
                        }
                        if let Some(ref mut info) = graph.nodes[idx_outer].portal {
                            info.twin_node_idx = Some(idx_inner);
                        }
                    }
                }
            }
        }

        // Find all junctions
        for y in self.ou..self.ob {
            for x in self.ol..self.or {
                let coord = (x as i32, y as i32);
                
                if self.grid[(x, y)] == Tile::Floor && !nodes_dict.contains_key(&coord) {
                    let open_paths = [(coord.0 + 1, coord.1), (coord.0 - 1, coord.1), (coord.0, coord.1 + 1), (coord.0, coord.1 - 1)]
                        .iter()
                        .filter(|&&c| {
                            self.grid[(c.0 as usize, c.1 as usize)] == Tile::Floor ||
                            matches!(self.grid[(c.0 as usize, c.1 as usize)], Tile::Portal(_))
                        })
                        .count();

                    if open_paths >= 3 {
                        let idx = graph.nodes.len();
                        graph.nodes.push(Node {
                            coord,
                            portal: None,
                            edge_idx: Vec::new(),
                        });
                        nodes_dict.insert(coord, idx);
                    }
                }
            }
        }
        
        // Flood-fill to create edges
        for current_idx in 0..graph.nodes.len() {
            let start_coord = graph.nodes[current_idx].coord;
            let is_portal = graph.nodes[current_idx].portal.is_some();

            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            visited.insert(start_coord);

            if is_portal {
                // Portals use their exact entry orientation
                let info = graph.nodes[current_idx].portal.as_ref().unwrap();
                let delta = info.orientation.to_dir();
                queue.push_back(((start_coord.0 + delta.0, start_coord.1 + delta.1), 1));
            } else {
                for dir in Ortho::iter() {
                    let delta = dir.to_dir();
                    let opt = (start_coord.0 + delta.0, start_coord.1 + delta.1);
                    
                    if self.grid[(opt.0 as usize, opt.1 as usize)] == Tile::Floor {
                        queue.push_back((opt, 1));
                    }
                }
            }

            // Run the corridor crawler
            while let Some((pos, dist)) = queue.pop_front() {
                if !visited.insert(pos) { continue; }

                // Node check
                if let Some(&target_idx) = nodes_dict.get(&pos) {
                    if current_idx < target_idx {
                        let edge_idx = graph.edges.len();
                        graph.edges.push(Edge {
                            node_a: current_idx,
                            node_b: target_idx,
                            length: dist,
                            is_warp: false,
                        });
                        
                        // Cross-link edge IDs to both node endpoints
                        graph.nodes[current_idx].edge_idx.push(edge_idx);
                        graph.nodes[target_idx].edge_idx.push(edge_idx);
                    }
                    continue;
                }

                // Standard straight corridor propagation
                for dir in Ortho::iter() {
                    let delta = dir.to_dir();
                    let opt = (pos.0 + delta.0, pos.1 + delta.1);
                    
                    if self.grid[(opt.0 as usize, opt.1 as usize)] != Tile::Wall {
                        queue.push_back((opt, dist + 1));
                    }
                }
            }
        }

        graph
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct SolverState {
    steps: usize,
    node_idx: usize,
    level: i32,
}

impl Ord for SolverState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
            .then_with(|| self.level.cmp(&other.level))
            .then_with(|| self.node_idx.cmp(&other.node_idx))
    }
}

impl PartialOrd for SolverState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PortalInfo {
    pub label: [char; 2],
    pub portal_type: PortalType,
    pub orientation: Ortho,
    pub twin_node_idx: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub coord: (i32, i32),
    pub portal: Option<PortalInfo>,
    pub edge_idx: Vec<usize>,
}

impl Node {
    pub fn new_from_coord(coord: (i32, i32), input: &MazeData) -> Self {
        let portal = match input.grid[coord] {
            Tile::Portal(info) => Some(info),
            _ => None,
        };
        Self { coord, portal, edge_idx: Vec::new() }
    }

    pub fn is_portal(&self) -> bool {
        self.portal.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub node_a: usize,
    pub node_b: usize,
    pub length: usize,
    pub is_warp: bool,
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: Vec::new(), edges: Vec::new() }
    }

    pub fn compress_nodes(&mut self) {
        for node_idx in 0..self.nodes.len() {
            if self.nodes[node_idx].portal.is_some() {
                continue;
            }

            // Only compress 2-way junctions
            if self.nodes[node_idx].edge_idx.len() == 2 {
                let edge_1_idx = self.nodes[node_idx].edge_idx[0];
                let edge_2_idx = self.nodes[node_idx].edge_idx[1];

                let edge_1 = self.edges[edge_1_idx].clone();
                let edge_2 = self.edges[edge_2_idx].clone();

                let neighbor_a_idx = if edge_1.node_a == node_idx { edge_1.node_b } else { edge_1.node_a };
                let neighbor_b_idx = if edge_2.node_a == node_idx { edge_2.node_b } else { edge_2.node_a };

                let combined_length = edge_1.length + edge_2.length;

                // Recycle Edge 1
                self.edges[edge_1_idx].node_a = neighbor_a_idx.min(neighbor_b_idx);
                self.edges[edge_1_idx].node_b = neighbor_a_idx.max(neighbor_b_idx);
                self.edges[edge_1_idx].length = combined_length;

                // Update Neighbour B's Edge list
                let b_edges = &mut self.nodes[neighbor_b_idx].edge_idx;
                if let Some(pos) = b_edges.iter().position(|&idx| idx == edge_2_idx) {
                    b_edges[pos] = edge_1_idx;
                }

                // Strip the junction of Edges
                self.nodes[node_idx].edge_idx.clear();

                // Nullify replaced edge
                self.edges[edge_2_idx].node_a = 0;
                self.edges[edge_2_idx].node_b = 0;
                self.edges[edge_2_idx].length = 0;
            }
        }
    }

    pub fn get_portal_str(&self, id: usize) -> Cow<'static, str> {
        self.nodes[id].portal.as_ref().map_or(
            Cow::Borrowed("None"),
            |p| Cow::Owned(format!("{:?}, {:?}", p.label, p.portal_type))
        )
    }

    pub fn get_portal_type(&self, id: usize) -> Option<([char; 2], PortalType)> {
        self.nodes[id].portal.map(|p| (p.label, p.portal_type))
    }

    pub fn list_edges(&self) {
        for (idx, edge) in self.edges.iter().enumerate() {
            println!("Edge id {}, node a: {} ({}), node b: {} ({})",
                idx, edge.node_a,
                self.get_portal_str(edge.node_a),
                edge.node_b,
                self.get_portal_str(edge.node_b)
            );
        }
    }

    pub fn list_nodes(&self) {
        for (idx, node) in self.nodes.iter().enumerate() {
            println!("Node id {}, portal: {}, edges: {}", idx, self.get_portal_str(idx), node.edge_idx.len());
        }
    }

    pub fn solve(&self, entrance_idx: usize, exit_idx: usize, is_part_2: bool) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        // Initial state is always at the 'AA' Entrance on Level 0 with 0 steps taken
        heap.push(SolverState { steps: 0, node_idx: entrance_idx, level: 0 });

        while let Some(SolverState { steps, node_idx, level }) = heap.pop() {
            if node_idx == exit_idx && level == 0 {
                return Some(steps);
            }

            // Deduplicate visited 3D states
            if !visited.insert((node_idx, level)) {
                continue;
            }

            let current_node = &self.nodes[node_idx];

            // Choice to walk standard Edge
            for &edge_idx in &current_node.edge_idx {
                let edge = &self.edges[edge_idx];
                let neighbor_idx = if edge.node_a == node_idx { edge.node_b } else { edge.node_a };

                heap.push(SolverState {
                    steps: steps + edge.length,
                    node_idx: neighbor_idx,
                    level,
                });
            }

            // Choice to walk portal Edfe
            if let Some(ref portal_info) = current_node.portal {
                if let Some(twin_idx) = portal_info.twin_node_idx {
                    
                    if is_part_2 {
                        match portal_info.portal_type {
                            PortalType::Inner => {
                                // Inner portals go up 1 level
                                heap.push(SolverState {
                                    steps: steps + 1,
                                    node_idx: twin_idx,
                                    level: level + 1,
                                });
                            }
                            PortalType::Outer => {
                                // Outer portals go down 1 level, but stop at 0
                                if level > 0 {
                                    heap.push(SolverState {
                                        steps: steps + 1,
                                        node_idx: twin_idx,
                                        level: level - 1,
                                    });
                                }
                            }
                            _ => {} // Entrance/Exit don't warp
                        }
                    } else {
                        // Part 1 - all portals are open. Only 1 level.
                        heap.push(SolverState {
                            steps: steps + 1,
                            node_idx: twin_idx,
                            level: 0, 
                        });
                    }
                }
            }
        }

        None
    }
}

fn detect_bound(grid: &Grid<Tile>, centre: &(i32, i32), dir: Ortho) -> i32 {
    let mut x = centre.0;
    let mut y = centre.1;

    while grid[(x, y)] == Tile::Empty {
        match dir {
            Ortho::North => y -= 1,
            Ortho::East  => x += 1,
            Ortho::South => y += 1,
            Ortho::West  => x -= 1,
        }
    }

    if grid[(x,y)] == Tile::Wall {
        return match dir {
            Ortho::East | Ortho::West => x,
            Ortho::North | Ortho::South => y,
        }
    } else {
        return match dir {
            Ortho::East => x + 2,
            Ortho::West => x - 2,
            Ortho::North => y - 2,
            Ortho::South => y + 2,
        }
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> MazeData {
    let width = input.lines().next().unwrap_or("").len();
    let height = input.lines().count();
    let mut label_count = 0;

    let entity = input.bytes()
        .filter(|&b| b != b'\n')
        .map(|b| match b {
            b' ' => Tile::Empty,
            b'.' => Tile::Floor,
            b'#' => Tile::Wall,
            b'A'..=b'Z' => {
                label_count += 1;
                Tile::Label(b as char)
            },
            _ => unreachable!(),
        })
        .collect();

    let grid = Grid::new(width, height, entity);

    // Doughnut bounds
    let centre = (grid.width() as i32 / 2, grid.height() as i32 / 2);
    let ou = 2;
    let ol = 2;
    let ob = grid.height() as i32 - 3;
    let or = grid.width() as i32 - 3;
    let iu = detect_bound(&grid, &centre, Ortho::North);
    let il = detect_bound(&grid, &centre, Ortho::West);
    let ib = detect_bound(&grid, &centre, Ortho::South);
    let ir = detect_bound(&grid, &centre, Ortho::East);

    // Portal finder
    let mut maze = MazeData { ou, ol, ob, or, iu, il, ib, ir, grid, portals: HashMap::new() };
    maze.find_portals();

    maze
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &MazeData) -> usize {
    let mut graph = input.graphify();
    graph.compress_nodes();
    
    let mut entrance_idx = 0;
    let mut exit_idx = 0;
    for (idx, node) in graph.nodes.iter().enumerate() {
        if let Some(ref portal) = node.portal {
            if portal.label == ['A'; 2] { entrance_idx = idx; }
            if portal.label == ['Z'; 2] { exit_idx = idx; }
        }
    }
    
    graph.solve(entrance_idx, exit_idx, false).unwrap_or(0)
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &MazeData) -> usize {
    let mut graph = input.graphify();
    graph.compress_nodes();
    
    let mut entrance_idx = 0;
    let mut exit_idx = 0;
    for (idx, node) in graph.nodes.iter().enumerate() {
        if let Some(ref portal) = node.portal {
            if portal.label == ['A'; 2] { entrance_idx = idx; }
            if portal.label == ['Z'; 2] { exit_idx = idx; }
        }
    }
    
    graph.solve(entrance_idx, exit_idx, true).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";

    const TEST2: &str = "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";

    const TEST3: &str = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";

    #[test]
    fn part1_test1() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 23);
    }

    #[test]
    fn part1_test2() {
        assert_eq!(solve_part1(&input_generator(TEST2)), 58);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST3)), 396);
    }
}