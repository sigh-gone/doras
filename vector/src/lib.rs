use std::iter::Sum;

pub struct Point3d<T> {
    x: T,
    y: T,
    z: T,
}

pub struct Point2d<T> {
    x: T,
    y: T,
}

pub struct PointOps;

impl PointOps {
    /*
    Points
     */

    pub fn add_2d_points<T>(point_vector: Vec<Point2d<T>>) -> Option<Point2d<T>>
    where
        T: std::ops::Add<Output = T>,
    {
        point_vector.into_iter().reduce(|acc, elem| Point2d {
            x: acc.x + elem.x,
            y: acc.y + elem.y,
        })
    }
    pub fn sub_2d_points<T>(point_vector: Vec<Point2d<T>>) -> Option<Point2d<T>>
    where
        T: std::ops::Sub<Output = T>,
    {
        point_vector.into_iter().reduce(|acc, elem| Point2d {
            x: acc.x - elem.x,
            y: acc.y - elem.y,
        })
    }

    pub fn add_3d_points<T>(point_vector: Vec<Point3d<T>>) -> Option<Point3d<T>>
    where
        T: std::ops::Add<Output = T>,
    {
        point_vector.into_iter().reduce(|acc, elem| Point3d {
            x: acc.x + elem.x,
            y: acc.y + elem.y,
            z: acc.z + elem.z,
        })
    }
    pub fn sub_3d_points<T>(point_vector: Vec<Point3d<T>>) -> Option<Point3d<T>>
    where
        T: std::ops::Sub<Output = T>,
    {
        point_vector.into_iter().reduce(|acc, elem| Point3d {
            x: acc.x - elem.x,
            y: acc.y - elem.y,
            z: acc.z - elem.z,
        })
    }

    pub fn translate_2d_points<T>(
        translation_point: Point2d<T>,
        point_vector: Vec<Point2d<T>>,
    ) -> Vec<Point2d<T>>
    where
        T: std::ops::Add + std::ops::Add<Output = T> + std::marker::Copy,
    {
        point_vector
            .into_iter()
            .map(|point| Point2d {
                x: point.x + translation_point.x,
                y: point.y + translation_point.y,
            })
            .collect()
    }

    pub fn translate_3d_points<T>(
        translation_point: Point3d<T>,
        point_vector: Vec<Point3d<T>>,
    ) -> Vec<Point3d<T>>
    where
        T: std::ops::Add + std::ops::Add<Output = T> + std::marker::Copy,
    {
        point_vector
            .into_iter()
            .map(|point| Point3d {
                x: point.x + translation_point.x,
                y: point.y + translation_point.y,
                z: point.z + translation_point.z,
            })
            .collect()
    }

    pub fn mult_scalar_3d<T>(point: Point3d<T>, scalar: T) -> Point3d<T>
    where
        T: std::ops::Mul + std::ops::Mul<Output = T> + std::marker::Copy,
    {
        Point3d {
            x: point.x * scalar,
            y: point.y * scalar,
            z: point.z * scalar,
        }
    }

    pub fn mult_scalar_2d<T>(point: Point2d<T>, scalar: T) -> Point2d<T>
    where
        T: std::ops::Mul + std::ops::Mul<Output = T> + std::marker::Copy,
    {
        Point2d {
            x: point.x * scalar,
            y: point.y * scalar,
        }
    }
}

pub fn dot_product_vec<T>(a: Vec<T>, b: Vec<T>) -> T
where
    T: std::ops::Mul + Sum + std::ops::Mul<Output = T> + std::marker::Copy,
{
    a.iter().zip(b.iter()).map(|(x, y)| *x * *y).sum()
}
