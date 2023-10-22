#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeodeticCoordinate {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct SphericalCoordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl SphericalCoordinate {
    pub fn from_node(node: &GeodeticCoordinate) -> Self {
        let lat_rad = node.lat.to_radians();
        let lon_rad = node.lon.to_radians();

        Self {
            x: lat_rad.cos() * lon_rad.cos(),
            y: lat_rad.cos() * lon_rad.sin(),
            z: lat_rad.sin(),
        }
    }

    pub fn cross_product(self, other: &Self) -> Self {
        SphericalCoordinate {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn divide_by_scalar(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }

    pub fn normalize(&self) -> SphericalCoordinate {
        let mag = self.magnitude();
        if mag == 0.0 {
            panic!("Cannot normalize a zero vector");
        }
        SphericalCoordinate {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn dot_product(&self, other: &SphericalCoordinate) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn angle_between(&self, other: &SphericalCoordinate) -> f64 {
        let angle = self.dot_product(other);
        let angle = angle / (self.magnitude() * other.magnitude());
        angle.acos()
    }
}

pub fn subtended_angle(
    a: &GeodeticCoordinate,
    b: &GeodeticCoordinate,
    c: &GeodeticCoordinate,
) -> f64 {
    let a_spherical = SphericalCoordinate::from_node(a);
    let b_spherical = SphericalCoordinate::from_node(b);
    let c_spherical = SphericalCoordinate::from_node(c);

    let vector_ab = a_spherical.cross_product(&b_spherical);

    let vector_ac = a_spherical.cross_product(&c_spherical);

    vector_ab.angle_between(&vector_ac)
}
