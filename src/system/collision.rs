use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use system::EMap;
use vector_2d::V2;
use X_LEN;
use Y_LEN;

const NUM_CELLS: usize = 10;

pub fn find_collisions(outlines: &HashSet<usize>, entities: &mut EMap) -> Vec<(usize, usize)> {
    let outlines: HashMap<usize, Vec<V2>> = outlines
        .iter()
        .flat_map(|id| {
            let entity = entities.get(id);
            entity
                .and_then(|e| e.outline.as_ref())
                .and_then(|o| {
                    if let Some(m) = entity.and_then(|e| e.momentum.as_ref()) {
                        return Some((*id, o.compute_outline(&m)));
                    }
                    None
                })
                .into_iter()
        })
        .collect();

    grid_search(&outlines)
}

fn grid_search(outlines: &HashMap<usize, Vec<V2>>) -> Vec<(usize, usize)> {
    let mut grid = vec![HashSet::<usize>::new(); NUM_CELLS * NUM_CELLS];

    for (idx, o) in outlines.iter() {
        for maybe_cell in o.iter().map(to_cell_id) {
            if let Some(cell) = maybe_cell {
                grid[cell].insert(*idx);
            }
        }
    }

    // Get unique pairs of outlines whose points share cells
    let mut matches = HashSet::new();
    for cell in grid.iter_mut() {
        let cell: Vec<usize> = cell.drain().collect();
        let n = cell.len();
        for i in 0..n {
            for j in i + 1..n {
                let pair = if cell[i] < cell[j] {
                    (cell[i], cell[j])
                } else {
                    (cell[j], cell[i])
                };
                matches.insert(pair);
            }
        }
    }

    // find collisions among pairs
    let mut collisions = Vec::new();
    'outlines: for (i, j) in matches.drain() {
        let i_outline = outlines.get(&i).unwrap();
        let j_outline = outlines.get(&j).unwrap();
        let n = i_outline.len();
        let m = j_outline.len();

        for ii in 0..n {
            for jj in ii..m {
                let i_seg = LineSegment(i_outline[ii], i_outline[(ii + 1) % n]);
                let j_seg = LineSegment(j_outline[jj], j_outline[(jj + 1) % m]);

                if i_seg.intersects(&j_seg) {
                    collisions.push((i, j));
                    break 'outlines;
                }
            }
        }
    }
    collisions
}

// Note that collisions off screen will not be tracked
fn to_cell_id(point: &V2) -> Option<usize> {
    let V2(x, y) = *point;
    if x < 0.0 || x > X_LEN || y < 0.0 || y > Y_LEN {
        return None;
    }
    let row = x.div_euc(X_LEN / NUM_CELLS as f32) as usize;
    let col = y.div_euc(X_LEN / NUM_CELLS as f32) as usize;
    let idx = row * NUM_CELLS + col;
    if idx < (NUM_CELLS * NUM_CELLS) as usize {
        Some(idx)
    } else {
        print!("wat {:?}", point);
        None
    }
}

// Based on
// https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/

struct LineSegment(V2, V2);

fn orientation(a: V2, b: V2, c: V2) -> Ordering {
    // Less => Counterclockwise, Equal => Colinear, More => clockwise
    (a - b).cross(c - b).partial_cmp(&0.0).unwrap()
}

impl LineSegment {
    fn bounding_box_contains_point(&self, p: &V2) -> bool {
        let LineSegment(V2(ax, ay), V2(bx, by)) = *self;
        let top = ay.max(by);
        let bottom = ay.min(by);
        let left = ax.min(bx);
        let right = ax.max(bx);
        left <= p.0 && p.0 <= right && bottom <= p.1 && p.1 <= top
    }

    fn intersects(&self, other: &LineSegment) -> bool {
        let o1 = orientation(self.0, self.1, other.0);
        let o2 = orientation(self.0, self.1, other.1);
        let o3 = orientation(other.0, other.1, self.0);
        let o4 = orientation(other.0, other.1, self.1);

        (o1 != o2 && o3 != o4)  // General case
        || (o1 == Ordering::Equal && self.bounding_box_contains_point(&other.0)) // Colinear cases
        || (o2 == Ordering::Equal && self.bounding_box_contains_point(&other.1))
        || (o3 == Ordering::Equal && other.bounding_box_contains_point(&self.0))
        || (o4 == Ordering::Equal && other.bounding_box_contains_point(&self.1))
    }
}

#[cfg(test)]
mod segments {
    use super::*;

    #[test]
    fn collinear_no_overlap() {
        let a = LineSegment(V2(0.0, 0.0), V2(1.0, 1.0));
        let b = LineSegment(V2(2.0, 2.0), V2(3.0, 3.0));
        assert!(!a.intersects(&b));
        assert!(!b.intersects(&a));
    }
    #[test]
    fn collinear_overlap() {
        let a = LineSegment(V2(0.0, 0.0), V2(1.0, 1.0));
        let b = LineSegment(V2(0.5, 0.5), V2(1.5, 1.5));
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
    }
    #[test]
    fn perpendicular_overlap() {
        let a = LineSegment(V2(-1.0, -1.0), V2(1.0, 1.0));
        let b = LineSegment(V2(1.0, -1.0), V2(-1.0, 1.0));
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
    }
    #[test]
    fn perpendicular_no_overlap() {
        let a = LineSegment(V2(0.1, 0.1), V2(1.0, 1.0));
        let b = LineSegment(V2(0.0, 0.0), V2(-1.0, 1.0));
        assert!(!a.intersects(&b));
        assert!(!b.intersects(&a));
    }
    #[test]
    fn endpoints_overlap() {
        let a = LineSegment(V2(0.0, 0.0), V2(1.0, 1.0));
        let b = LineSegment(V2(2.2, -3.3), V2(0.5, 0.5));
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
    }
}

// #[cfg(test)]
// mod outlines {
//     use super::*;
//     #[test]
//     fn overlapping_squares() {
//         let outlines = vec![
//             vec![V2(0.0, 0.0), V2(1.0, 0.0), V2(1.0, 1.0), V2(0.0, 1.0)],
//             vec![V2(0.5, 0.5), V2(1.5, 0.5), V2(1.5, 1.5), V2(0.5, 1.5)],
//         ];
//         assert!(find_collisions(&outlines).len() == 1);
//     }
//     #[test]
//     fn non_overlapping_squares() {
//         let outlines = vec![
//             vec![V2(0.0, 0.0), V2(1.0, 0.0), V2(1.0, 1.0), V2(0.0, 1.0)],
//             vec![V2(1.5, 1.5), V2(2.5, 1.5), V2(2.5, 2.5), V2(1.5, 2.5)],
//         ];
//         assert!(find_collisions(&outlines).len() == 0);
//     }
//     #[test]
//     fn overlapping_triangle_and_line() {
//         let outlines = vec![
//             vec![V2(1.0, 2.0), V2(3.0, 2.0)],
//             vec![V2(2.0, 1.0), V2(2.0, 3.0), V2(2.5, 3.0)]
//         ];
//         assert!(find_collisions(&outlines).len() == 1);
//     }
//     #[test]
//     fn non_overlapping_triangle_and_line() {
//         let outlines = vec![
//             vec![V2(10.0, 20.0), V2(30.0, 20.0)],
//             vec![V2(2.0, 1.0), V2(2.0, 3.0), V2(2.5, 3.0)]
//         ];
//         assert!(find_collisions(&outlines).len() == 0);
//     }
// }
