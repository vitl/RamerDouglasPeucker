#[allow(non_camel_case_types)]
pub type float = f64;

pub type Point = (float, float);
pub type Line = (Point, Point);

pub trait HasPoint: Copy {
    fn to_point(self) -> Point;
}

impl HasPoint for Point {
    fn to_point(self) -> Point {self}
}

pub fn ramer_douglas_peucker<T: HasPoint>(v: Vec<T>, epsilon: float) -> Vec<T> {
    let length = v.len();
    if length < 3 {
        return v;
    }
    let mut stack = vec![(0, length - 1)];
    let mut result = Vec::new();
    let mut last_stack_index = -1;

    while let Some((start_index, end_index)) =  stack.pop() {
        // println!("start = {}, end = {}", start_index, end_index);
        let mut max_distance = 0.0 as float;
        let mut max_index = start_index;
        for i in start_index+1..end_index {
            let point = v[i].to_point();
            let line = (v[start_index].to_point(), v[end_index].to_point());
            let distance = distance_point_to_line(point, line);
            // println!("i = {}, distance = {}", i, distance);
            if distance > max_distance {
                max_distance = distance;
                max_index = i;
            }
        }
        if max_distance > epsilon {
            stack.push((max_index, end_index));
            stack.push((start_index, max_index));
        } else {
            if last_stack_index != start_index {
                result.push(v[start_index]);
            }
            result.push(v[end_index]);
            last_stack_index = end_index;
        }
    }
    result
}

pub fn distance_point_to_line(p: Point, l: Line) -> float {
    if l.0 == l.1 {
        return distance_point_to_point(p, l.0);
    }
    let a = (l.0).1 - (l.1).1;
    let b = (l.1).0 - (l.0).0;
    let c = (l.0).0 * (l.1).1 - (l.1).0 * (l.0).1;
    let result = (a * p.0 + b * p.1 + c) / (a * a + b * b).sqrt();
    result.abs()
}

pub fn distance_point_to_point(x: Point, y: Point) -> float {
    let a = y.0 - x.0;
    let b = y.1 - x.1;
    let result = (a * a + b * b).sqrt();
    result.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_vector() {
        // Minimal cases:
        assert_eq!(vec![(1.0, 1.0)], ramer_douglas_peucker(vec![(1.0, 1.0)], 0.5));
        assert_eq!(vec![(1.0, 1.0), (2.0, 2.0)], ramer_douglas_peucker(vec![(1.0, 1.0), (2.0, 2.0)], 0.5));
        assert_eq!(vec![(1.0, 1.0), (3.0, 3.0)], ramer_douglas_peucker(vec![(1.0, 1.0), (2.0, 2.0), (3.0, 3.0)], 0.5));
        // Effect of varying epsilon:
        assert_eq!(vec![(0.0, 2.0), (1.0, 1.0), (3.0, 0.0), (5.0, 1.0)], ramer_douglas_peucker(vec![(0.0, 2.0), (1.0, 1.0), (3.0, 0.0), (5.0, 1.0)], 0.1));
        assert_eq!(vec![(0.0, 2.0), (3.0, 0.0), (5.0, 1.0)], ramer_douglas_peucker(vec![(0.0, 2.0), (1.0, 1.0), (3.0, 0.0), (5.0, 1.0)], 0.5));
        // Tests with vertical segments:
        assert_eq!(vec![(10.0, 35.0), (20.0, 29.0)], ramer_douglas_peucker(vec![(10.0, 35.0), (15.0, 34.0), (15.0, 30.0), (20.0, 29.0)], 10.0));
        assert_eq!(vec![(10.0, 35.0), (15.0, 34.0), (15.0, 30.0), (20.0, 29.0)], ramer_douglas_peucker(vec![(10.0, 35.0), (15.0, 34.0), (15.0, 30.0), (20.0, 29.0)], 1.0));
        // Tests with horizontal segments:
        assert_eq!(vec![(10.0, 35.0), (15.0, 35.0), (16.0, 30.0), (21.0, 30.0)], ramer_douglas_peucker(vec![(10.0, 35.0), (15.0, 35.0), (16.0, 30.0), (21.0, 30.0)], 1.0));
        assert_eq!(vec![(10.0, 35.0), (21.0, 30.0)], ramer_douglas_peucker(vec![(10.0, 35.0), (15.0, 35.0), (16.0, 30.0), (21.0, 30.0)], 10.0));
        // Tests with vertical and horizontal segments:
        assert_eq!(vec![(10.0, 30.0), (50.0, 10.0)], ramer_douglas_peucker(vec![(10.0, 30.0), (30.0, 30.0), (30.0, 10.0), (50.0, 10.0)], 10.0));
        assert_eq!(vec![(10.0, 30.0), (50.0, 10.0)], ramer_douglas_peucker(vec![(10.0, 30.0), (30.0, 30.0), (30.0, 10.0), (50.0, 10.0)], 15.0));
        // A more complex curve:
        assert_eq!(vec![(3.5, 21.25), (23.2, 3.1), (54.6, 18.15), (71.5, 9.7), (101.3, 21.1)], ramer_douglas_peucker(vec![(3.5, 21.25), (7.3, 12.0), (23.2, 3.1), (37.2, 12.07), (54.6, 18.15), (62.2, 16.45), (71.5, 9.7), (101.3, 21.1)], 5.0));
        assert_eq!(vec![(0.0, 0.0), (0.5, 0.5), (1.25, -0.25), (1.5, 0.5)], ramer_douglas_peucker(vec![(0.0,0.0),(0.5,0.5),(1.0,0.0),(1.25,-0.25),(1.5,0.5)], 0.25));
        // Start point == end point
        assert_eq!(vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0), (0.0, 0.0)],ramer_douglas_peucker(vec![(0.0,0.0),(1.0,0.0),(2.0,0.0),(2.0,1.0),(2.0,2.0),(1.0,2.0),(0.0,2.0),(0.0,1.0),(0.0, 0.0)], 1.0));
    }

    #[test]
    fn calculate_point_to_line_distance() {
        assert!((distance_point_to_line((3.0, 2.0), ((-2.0, 0.0),(0.0, 2.0))) - 2.12132).abs() < 0.00001);
        assert!(distance_point_to_line((0.0, 0.0), ((-1.0, 0.0),(1.0, 0.0))).abs() < 0.00001);
    }

    #[test]
    fn calculate_point_to_point_distance() {
        assert!((distance_point_to_point((3.0, 2.0), (5.0, -1.0)) - 3.60555).abs() < 0.00001);
        assert!(distance_point_to_point((3.0, 2.0), (3.0, 2.0)).abs() < 0.00001);
    }

    // Reduce vector with extra data fieds
    impl HasPoint for (float, float, i32) {
        fn to_point(self) -> Point {(self.0, self.1)}
    }

    #[test]
    fn extra_fields_vector_reduce() {
        assert_eq!(vec![(1.0, 1.0, 1), (3.0, 3.0, 3)], ramer_douglas_peucker(vec![(1.0, 1.0, 1), (2.0, 2.0, 2), (3.0, 3.0, 3)], 0.5));
    }
}
