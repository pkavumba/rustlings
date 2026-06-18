// Pattern matching and destructuring let you pull fields out of structs. Use
// borrowed patterns when you want to keep using the original value afterwards.

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
}

fn horizontal_length(segment: &Segment) -> i32 {
    let Segment {
        start: Point { x: start_x, .. },
        end: Point { x: end_x, .. },
    } = segment;

    (end_x - start_x).abs()
}

fn into_points(segment: Segment) -> (Point, Point) {
    let Segment { start, end } = segment;
    (start, end)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructures_by_borrow_and_by_move() {
        let segment = Segment {
            start: Point { x: -3, y: 4 },
            end: Point { x: 5, y: 4 },
        };

        assert_eq!(horizontal_length(&segment), 8);
        assert_eq!(segment.start.y, 4);

        let (start, end) = into_points(segment);
        assert_eq!(start.x, -3);
        assert_eq!(end.x, 5);
    }
}
