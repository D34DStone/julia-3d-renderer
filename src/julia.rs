use nalgebra as na;
use super::texture::Texture;
use na::Vector4 as v4;
use na::Vector3 as v3;
use na::Vector2 as v2;
use na::Matrix4 as mat4;

trait GeometryVertex {
    type InputVertex;
    type Vertex;
}

