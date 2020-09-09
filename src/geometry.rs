#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

const ERR_2D_UNSUPP_AXIS: &str = "Unsupported Axis. 2D supports X and Y.";

pub trait TCoordinate {
    fn get_axis_value(&self, axis: Axis) -> &i32;

    fn set_axis_value(&mut self, axis: Axis, value: i32);
}

pub trait TTransformable {
    fn shift<T: TCoordinate>(&mut self, coord: &T);

    fn shift_axis(&mut self, axis: Axis, value: i32);

    fn flip_axis(&mut self, axis1: Axis, axis2: Axis);

    fn reflect(&mut self);

    fn reflect_axis(&mut self, axis: Axis);
}

pub trait TPoint: PartialEq +  PartialOrd + Copy + Clone + Default + TTransformable + TCoordinate {}
impl<T: PartialEq +  PartialOrd + Copy + Clone + Default + TTransformable + TCoordinate> TPoint for T {}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum EPoint {
    SPoint3D (SPoint3D),
    SPoint2D (SPoint2D),
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct SPoint3D {
    x: i32,
    y: i32,
    z: i32,
}

impl TTransformable for SPoint3D {
    fn shift<T: TCoordinate>(&mut self, coord: &T) {
        self.x += coord.get_axis_value(Axis::X);
        self.y += coord.get_axis_value(Axis::Y);
        self.z += coord.get_axis_value(Axis::Z);
    }

    fn shift_axis(&mut self, axis: Axis, value: i32) {
        match axis {
            Axis::X => self.x += value,
            Axis::Y => self.y += value,
            Axis::Z => self.z += value,
        }
    }

    fn flip_axis(&mut self, axis1: Axis, axis2: Axis) {
        let orig_axis2_value = *self.get_axis_value(axis2);
        self.set_axis_value(axis2, *self.get_axis_value(axis1));
        self.set_axis_value(axis1, orig_axis2_value);
    }

    fn reflect(&mut self) {
        self.x *= -1;
        self.y *= -1;
        self.z *= -1;
    }

    fn reflect_axis(&mut self, axis: Axis) {
        match axis {
            Axis::X => self.x *= -1,
            Axis::Y => self.y *= -1,
            Axis::Z => self.z *= -1,
        }
    }
}

impl TCoordinate for SPoint3D {
    fn get_axis_value(&self, axis: Axis) -> &i32 {
        match axis {
            Axis::X => return &self.x,
            Axis::Y => return &self.y,
            Axis::Z => return &self.z,
        }
    }

    fn set_axis_value(&mut self, axis: Axis, value: i32) {
        match axis {
            Axis::X => self.x = value,
            Axis::Y => self.y = value,
            Axis::Z => self.z = value,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct SPoint2D {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
impl SPoint2D {
    pub fn flip(&mut self) {
        self.flip_axis(Axis::X, Axis::Y);
    }
}

impl TTransformable for SPoint2D {
    fn shift<T: TCoordinate>(&mut self, coord: &T) {
        self.x += coord.get_axis_value(Axis::X);
        self.y += coord.get_axis_value(Axis::Y);
    }

    fn shift_axis(&mut self, axis: Axis, value: i32) {
        match axis {
            Axis::X => self.x += value,
            Axis::Y => self.y += value,
            _ => panic!(ERR_2D_UNSUPP_AXIS),
        }
    }

    fn flip_axis(&mut self, axis1: Axis, axis2: Axis) {
        let orig_axis2_value = *self.get_axis_value(axis2);
        self.set_axis_value(axis2, *self.get_axis_value(axis1));
        self.set_axis_value(axis1, orig_axis2_value);
    }

    fn reflect(&mut self) {
        self.x *= -1;
        self.y *= -1;
    }

    fn reflect_axis(&mut self, axis: Axis) {
        match axis {
            Axis::X => self.x *= -1,
            Axis::Y => self.y *= -1,
            _ => panic!(ERR_2D_UNSUPP_AXIS),
        }
    }
}

impl TCoordinate for SPoint2D {
    fn get_axis_value(&self, axis: Axis) -> &i32 {
        match axis {
            Axis::X => return &self.x,
            Axis::Y => return &self.y,
            Axis::Z => return &0,
        }
    }

    fn set_axis_value(&mut self, axis: Axis, value: i32) {
        match axis {
            Axis::X => self.x = value,
            Axis::Y => self.y = value,
            _ => panic!(ERR_2D_UNSUPP_AXIS),
        }
    }
}

#[allow(dead_code)]
pub struct Line(EPoint, EPoint);

// TODO: supporting point generics

// impl Line {
//     pub fn intersection(&self, line: &Line) -> Option<Box<dyn TPoint>> {
//         let a = &self.0;
//         let b = &self.1;
//         let c = &line.0;
//         let d = &line.1;

//         // Line AB represented as a1x + b1y = c1
//         let a1 = b.y - a.y;
//         let b1 = a.x - b.x;
//         let c1 = a1 * (a.x) + b1 * (a.y);
  
//         // Line CD represented as a2x + b2y = c2  
//         let a2 = d.y - c.y;
//         let b2 = c.x - d.x;
//         let c2 = a2 * (c.x) + b2 * (c.y);
  
//         let determinant = a1 * b2 - a2 * b1;

//         if determinant == 0 {
//             // lines are parallel, there is no intersection point
//             return None;
//         }
//         else {
//             let x = (b2 * c1 - b1 * c2) / determinant;
//             let y = (a1 * c2 - a2 * c1) / determinant;
//             return Some(Point2D { x, y, });
//         }
//     }
// }

// #[derive(Debug)]
// pub struct Line2D(Point2D, Point2D);

// impl Line2D {
//     pub fn intersection(&self, line: &Line2D) -> Option<Point2D> {
//         let a = &self.0;
//         let b = &self.1;
//         let c = &line.0;
//         let d = &line.1;

//         // Line AB represented as a1x + b1y = c1
//         let a1 = b.y - a.y;
//         let b1 = a.x - b.x;
//         let c1 = a1 * (a.x) + b1 * (a.y);
  
//         // Line CD represented as a2x + b2y = c2  
//         let a2 = d.y - c.y;
//         let b2 = c.x - d.x;
//         let c2 = a2 * (c.x) + b2 * (c.y);
  
//         let determinant = a1 * b2 - a2 * b1;

//         if determinant == 0 {
//             // lines are parallel, there is no intersection point
//             return None;
//         }
//         else {
//             let x = (b2 * c1 - b1 * c2) / determinant;
//             let y = (a1 * c2 - a2 * c1) / determinant;
//             return Some(Point2D { x, y, });
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     // import all names for outer scope
//     use super::*;

//     #[test]
//     fn test_line2d_intersection() {
//         let l1 = Line2D( Point2D {x: 1, y: 3,}, Point2D {x: 3, y: 1,} );
//         let l2 = Line2D( Point2D {x: 1, y: 1,}, Point2D {x: 4, y: 1,} );

//         assert_eq!( l1.intersection(&l2), Some(Point2D {x: 3, y: 1,}) );
//     }

//     #[test]
//     // intersection occurs at point not on defined line, but projected
//     fn test_line2d_intersection_project() {
//         let l1 = Line2D( Point2D {x: 1, y: 3,}, Point2D {x: 2, y: 2,} );
//         let l2 = Line2D( Point2D {x: 1, y: 1,}, Point2D {x: 4, y: 1,} );

//         assert_eq!( l1.intersection(&l2), Some(Point2D {x: 3, y: 1,}) );
//     }

//     #[test]
//     // intersection math is rounded, precision dropped
//     fn test_line2d_intersection_decimal() {
//         let l1 = Line2D( Point2D {x: 1, y: 1,}, Point2D {x: 4, y: 4,} );
//         let l2 = Line2D( Point2D {x: 1, y: 8,}, Point2D {x: 2, y: 4,} );

//         assert_eq!( l1.intersection(&l2), Some(Point2D {x: 2, y: 2,}) );
//     }

//     #[test]
//     // two lines that are parrallel and do not intersect
//     fn test_line2d_intersection_none() {
//         let l1 = Line2D( Point2D {x: 1, y: 1,}, Point2D {x: 4, y: 1,} );
//         let l2 = Line2D( Point2D {x: 1, y: 2,}, Point2D {x: 4, y: 2,} );

//         assert_eq!( l1.intersection(&l2), None );
//     }
// }