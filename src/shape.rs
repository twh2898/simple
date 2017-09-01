use glium;
use cgm;

use color::Color;
use std::rc::Rc;
use glium::texture::Texture2d;
use std::iter::{Chain, Once, once};

/// Trait for structs to be drawn with `Frame::draw`
pub trait Shape: IntoIterator<Item = RendTri> {}

impl<S> Shape for S where S: IntoIterator<Item = RendTri> {}

/// Renderable triangle which includes color and texture information.
pub struct RendTri {
    pub(crate) tri: Tri,
    pub(crate) texture: Option<Rc<Texture2d>>,
}

impl From<Tri> for RendTri {
    fn from(tri: Tri) -> Self {
        RendTri {
            tri: tri,
            texture: None,
        }
    }
}

/// Three positions which form a matrix for shader purposes
#[derive(Copy, Clone, Debug)]
pub struct Positions(pub [[f32; 2]; 3]);

/// A triangle primitive which enters the shader pipeline as a single vertex and is the only primitive in nest
#[derive(Copy, Clone, Debug)]
pub struct Tri {
    /// The three space vertices of the triangles
    pub positions: Positions,
    /// The three texture coordinates of the above vertices
    pub texcoords: Positions,
    /// The color of this triangle.
    pub color: [f32; 4],
}

impl Tri {
    /// Create a new triangle with points and tex coordinates specified
    #[inline]
    pub fn new<P: Into<cgm::Point2<f32>> + Copy, T: Into<cgm::Point2<f32>> + Copy, C: Into<Color>>(
        positions: [P; 3],
        texcoords: [T; 3],
        color: C,
    ) -> Tri {
        Tri {
            positions: Positions(
                [
                    positions[0].into().into(),
                    positions[1].into().into(),
                    positions[2].into().into(),
                ],
            ),
            texcoords: Positions(
                [
                    texcoords[0].into().into(),
                    texcoords[1].into().into(),
                    texcoords[2].into().into(),
                ],
            ),
            color: color.into().0,
        }
    }

    /// Create a new triangle with points with coordinates to create a single-color triangle
    #[inline]
    pub fn new_pos<P: Into<cgm::Point2<f32>> + Copy>(positions: [P; 3]) -> Tri {
        Tri::new(
            [
                positions[0].into(),
                positions[1].into(),
                positions[2].into(),
            ],
            [cgm::Point2::new(0.0, 0.0); 3],
            Color::WHITE,
        )
    }
}

implement_vertex!(Tri, positions, texcoords, color);

unsafe impl glium::vertex::Attribute for Positions {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32x2x3
    }
}

/// Two points make a rectangle.
#[derive(Copy, Clone, Debug)]
pub struct Rect(pub [f32; 2], pub [f32; 2]);

impl IntoIterator for Rect {
    type IntoIter = Chain<Once<RendTri>, Once<RendTri>>;
    type Item = RendTri;
    fn into_iter(self) -> Self::IntoIter {
        once(Tri::new_pos(
            [
                [self.0[0], self.0[1]],
                [self.1[0], self.0[1]],
                [self.0[0], self.1[1]],
            ],
        ).into()).chain(once(Tri::new_pos(
            [
                [self.1[0], self.1[1]],
                [self.0[0], self.1[1]],
                [self.1[0], self.0[1]],
            ],
        ).into()))
    }
}
