use std::error::Error;

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let points: Vec<_> = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',');
            let x = parts.next()?.trim().parse::<i64>().ok()?;
            let y = parts.next()?.trim().parse::<i64>().ok()?;
            let z = parts.next()?.trim().parse::<i64>().ok()?;
            Some(Point3D { x, y, z })
        })
        .collect();

    let (p1, p2) = build_graph(&points, 1000);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

fn dist2(u: Point3D, v: Point3D) -> i64 {
    (u.x - v.x).pow(2) + (u.y - v.y).pow(2) + (u.z - v.z).pow(2)
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            // flatten each subgraph
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            self.count -= 1;
        }
    }

    fn top3(&self) -> usize {
        let mut sizes: Vec<usize> = (0..self.parent.len())
            .filter(|&i| self.parent[i] == i)
            .map(|i| self.size[i])
            .collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a));
        sizes.iter().take(3).product()
    }
}

fn build_graph(points: &[Point3D], num_iter: usize) -> (usize, i64) {
    let mut pairs = Vec::with_capacity(points.len() * points.len() / 2); // (idx1, idx2, distance)
    let mut snapshot_size = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            pairs.push((i, j, dist2(points[i], points[j])));
        }
    }

    pairs.sort_unstable_by_key(|&(_, _, d)| d);

    let mut uf = UnionFind::new(points.len());
    for (i, &(u, v, _)) in pairs.iter().enumerate() {
        if i == num_iter {
            snapshot_size = uf.top3();
        }
        uf.union(u, v);
        if uf.count == 1 {
            return (snapshot_size, points[u].x * points[v].x);
        }
    }

    (0, 0)
}
