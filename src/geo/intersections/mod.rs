use std::ops::Index;
use crate::geo::{Intersection, Intersectable};
pub mod intersection;
pub mod intersectable;

pub struct Intersections {
    data: Vec<Intersection>,
}

impl Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Self {
        let mut data = intersections;
        data.sort_unstable_by(|a,b| a.t.partial_cmp(&b.t).unwrap());
        Self { data  }
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.data.iter().find(|i| i.t >= 0.0)
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::Intersections;
    use crate::geo::{Intersection, Intersectable, Sphere};

    #[test]
    fn it_aggregate_intersections() {
        let s = Sphere::unit();
        let i1 = Intersection::new(1., Intersectable::Sphere(s));
        let i2 = Intersection::new(2., Intersectable::Sphere(s));
        let i = Intersections::new(vec![i1, i2]);
        assert_eq!(i.count(), 2);
        assert_eq!(i[0].t, 1.);
        assert_eq!(i[1].t, 2.)
    }


    #[test]
    fn it_calculates_the_hit_value_when_all_intersections_have_a_positive_t_value() {
        let s = Sphere::unit();
        let i1 = Intersection::new(1., Intersectable::Sphere(s));
        let i2 = Intersection::new(2., Intersectable::Sphere(s));
        let xs = Intersections::new(vec![i1, i2]);
        let expected = Intersection::new(1., Intersectable::Sphere(s));
        assert_eq!(xs.hit(), Some(&expected));
    }

    #[test]
    fn it_calculates_the_hit_value_when_some_intersections_have_a_negative_t_value() {
        let s = Sphere::unit();
        let i1 = Intersection::new(-1., Intersectable::Sphere(s));
        let i2 = Intersection::new(2., Intersectable::Sphere(s));
        let xs = Intersections::new(vec![i1, i2]);
        let expected  = Intersection::new(2., Intersectable::Sphere(s));
        assert_eq!(xs.hit(), Some(&expected));
    }

    #[test]
    fn it_has_a_none_hit_value_when_all_intersections_have_a_negative_t_value() {
        let s = Sphere::unit();
        let i1 = Intersection::new(-2., Intersectable::Sphere(s));
        let i2 = Intersection::new(-1., Intersectable::Sphere(s));
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn it_calculates_the_hit_value_by_using_the_intersection_with_the_lowest_positive_value() {
        let s = Sphere::unit();
        let i1 = Intersection::new(5., Intersectable::Sphere(s));
        let i2 = Intersection::new(7., Intersectable::Sphere(s));
        let i3 = Intersection::new(-3., Intersectable::Sphere(s));
        let i4 = Intersection::new(2., Intersectable::Sphere(s));
        let xs = Intersections::new(vec![i1, i2, i3, i4]);
        let expected = Intersection::new(2., Intersectable::Sphere(s));
        assert_eq!(xs.hit(), Some(&expected));
    }

}
