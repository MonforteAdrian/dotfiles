use crate::Iso;
use glam::{IVec2, IVec3, Vec2, Vec3};

impl From<(i32, i32, i32)> for Iso {
    #[inline]
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self::new(x, y, z)
    }
}

impl From<[i32; 3]> for Iso {
    #[inline]
    fn from(a: [i32; 3]) -> Self {
        Self::from_array(a)
    }
}

impl From<(f32, f32, f32)> for Iso {
    #[inline]
    fn from(v: (f32, f32, f32)) -> Self {
        Self::round(v.into())
    }
}

impl From<[f32; 3]> for Iso {
    #[inline]
    fn from(v: [f32; 3]) -> Self {
        Self::round(v)
    }
}

impl From<Iso> for IVec2 {
    #[inline]
    fn from(iso: Iso) -> Self {
        iso.as_ivec2()
    }
}

impl From<Vec3> for Iso {
    #[inline]
    fn from(value: Vec3) -> Self {
        Self::from(value.to_array())
    }
}

// TODO delete this and fix its dependencies
impl From<Vec2> for Iso {
    #[inline]
    fn from(value: Vec2) -> Self {
        Self::from(Vec3::new(value.x, value.y, 0.0))
    }
}

impl From<Iso> for IVec3 {
    #[inline]
    fn from(iso: Iso) -> Self {
        iso.as_ivec3()
    }
}

impl From<IVec2> for Iso {
    #[inline]
    fn from(v: IVec2) -> Self {
        Self::new(v.x, v.y, 0)
    }
}
