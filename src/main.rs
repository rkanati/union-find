
mod uf {
    struct Repr<T> {
        pub rank: usize,
        pub data: Option<T> // TODO pain in the ass
    }

    impl<T> Repr<T> {
        fn update(&mut self, mut other: Repr<T>, merge: impl FnOnce(T, T) -> T) {
            if self.rank == other.rank {
                self.rank += 1;
            }
            // TODO is Option<T> generally useful?
            let self_data = self.data.take().unwrap();
            let other_data = other.data.take().unwrap();
            let new_data = merge(self_data, other_data);
            self.data = Some(new_data);
        }
    }

    enum Node<T> {
        Repr(Repr<T>),
        Alias(usize)
    }

    impl<T> Node<T> {
        fn to_repr(self) -> Repr<T> {
            match self {
                Node::Repr(repr) => repr,
                _                => panic!()
            }
        }

        fn repr(&mut self) -> &mut Repr<T> {
            match self {
                Node::Repr(repr) => repr,
                _                => panic!()
            }
        }

        fn rank(&self) -> usize {
            match self {
                Node::Repr(repr) => repr.rank,
                _                => panic!()
            }
        }

        fn update(&mut self, absorb: Node<T>, merge: impl FnOnce(T, T) -> T) {
            let self_repr = self.repr();
            let other_repr = absorb.to_repr();
            self_repr.update(other_repr, merge);
        }

        fn new_repr(data: T) -> Node<T> {
            Node::Repr(Repr {
                rank: 0,
                data: Some(data)
            })
        }
    }

    pub struct UF<T = ()> {
        vec: Vec<Node<T>>
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Point(usize);

    impl<T> UF<T> {
        pub fn new() -> UF<T> {
            UF { vec: Vec::new() }
        }

        pub fn make_point(&mut self, data: T) -> Point {
            let index = self.vec.len();
            self.vec.push(Node::new_repr(data));
            Point(index)
        }

        fn node(&self, index: usize) -> &Node<T> {
            self.vec.get(index).unwrap()
        }

        fn node_mut(&mut self, index: usize) -> &mut Node<T> {
            self.vec.get_mut(index).unwrap()
        }

        fn parent(&self, index: usize) -> usize {
            match self.node(index) {
                Node::Alias(parent) => *parent,
                Node::Repr { .. }   => index
            }
        }

        pub fn find(&mut self, mut point: Point) -> Point {
            while let Node::Alias(to) = *self.node(point.0) {
                *self.node_mut(point.0) = Node::Alias(self.parent(to));
                point = Point(to);
            }
            point
        }

        pub fn get(&mut self, point: Point) -> &T {
            self.find_and_get(point).1
        }

        pub fn find_and_get(&mut self, point: Point) -> (Point, &T) {
            let point = self.find(point);
            if let Node::Repr(Repr { data, .. }) = self.node(point.0) {
                (point, data.as_ref().unwrap())
            }
            else {
                panic!()
            }
        }

        pub fn union(&mut self, a: Point, b: Point, merge: impl FnOnce(T, T) -> T) -> Point {
            let a = self.find(a);
            let b = self.find(b);
            if a == b {
                return a;
            }

            let node_a = self.node(a.0);
            let node_b = self.node(b.0);

            if node_a.rank() < node_b.rank() {
                let mut node_a = Node::Alias(b.0);
                std::mem::swap(&mut node_a, self.node_mut(a.0));
                self.node_mut(b.0).update(node_a, merge);
                b
            }
            else {
                let mut node_b = Node::Alias(a.0);
                std::mem::swap(&mut node_b, self.node_mut(b.0));
                self.node_mut(a.0).update(node_b, merge);
                a
            }
        }
    }

    #[cfg(tests)]
    mod tests {
        use super::*;

        #[test]
        fn test_uf() {
            let mut uf: UF<i32> = UF::new();
            let p = uf.make_point(1);
            let q = uf.make_point(2);
            assert_not_eq!(p, q);

            uf.union(p, q, |a, b| a);

            let p = uf.find(p);
            let q = uf.find(q);
            assert_eq!(p, q);

            assert_eq!(uf.get(p), 1);
        }


    }
}

fn main() {
}

