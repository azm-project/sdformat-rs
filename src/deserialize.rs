use num_traits::Zero;
use serde::Deserialize;
use std::str::FromStr;

macro_rules! value_node {
    ( $name:ident, $t:ty ) => {
        #[derive(Debug, Deserialize, Default, Clone)]
        pub struct $name {
            #[serde(rename = "$value")]
            pub value: $t,
        }
    };
}

value_node!(Mass, f64);
value_node!(Ixx, f64);
value_node!(Ixy, f64);
value_node!(Ixz, f64);
value_node!(Iyy, f64);
value_node!(Iyz, f64);
value_node!(Izz, f64);
value_node!(Radius, f64);
value_node!(Length, f64);
value_node!(Name, String);
value_node!(Uri, String);
value_node!(Damping, f64);
value_node!(Friction, f64);
value_node!(Lower, f64);
value_node!(Upper, f64);
value_node!(UseParentModelFrame, bool);
value_node!(Parent, String);
value_node!(Child, String);

pub fn string_to_array<T, const N: usize>(s: &str) -> Option<[T; N]>
where
    T: Zero + FromStr + Copy,
{
    let vec = s
        .split(' ')
        .filter_map(|x| x.parse::<T>().ok())
        .collect::<Vec<_>>();
    if vec.len() != N {
        return None;
    }
    let mut arr = [T::zero(); N];
    arr.copy_from_slice(&vec);
    Some(arr)
}

mod sdf_fvec6 {
    use serde::{self, Deserialize, Deserializer};
    pub fn deserialize<'a, D>(deserializer: D) -> Result<[f64; 6], D::Error>
    where
        D: Deserializer<'a>,
    {
        let s = String::deserialize(deserializer)?;
        let some_arr = crate::string_to_array::<f64, 6>(&s);
        if let Some(arr) = some_arr {
            Ok(arr)
        } else {
            return Err(serde::de::Error::custom(format!(
                "failed to parse float array in {}",
                s
            )));
        }
    }
}

mod sdf_fvec3 {
    use serde::{self, Deserialize, Deserializer};
    pub fn deserialize<'a, D>(deserializer: D) -> Result<[f64; 3], D::Error>
    where
        D: Deserializer<'a>,
    {
        let s = String::deserialize(deserializer)?;
        let some_arr = crate::string_to_array::<f64, 3>(&s);
        if let Some(arr) = some_arr {
            Ok(arr)
        } else {
            return Err(serde::de::Error::custom(format!(
                "failed to parse float array in {}",
                s
            )));
        }
    }
}

mod sdf_ivec3 {
    use serde::{self, Deserialize, Deserializer};
    pub fn deserialize<'a, D>(deserializer: D) -> Result<[u32; 3], D::Error>
    where
        D: Deserializer<'a>,
    {
        let s = String::deserialize(deserializer)?;
        let some_arr = crate::string_to_array::<u32, 3>(&s);
        if let Some(arr) = some_arr {
            Ok(arr)
        } else {
            return Err(serde::de::Error::custom(format!(
                "failed to parse u32 array in {}",
                s
            )));
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Pose {
    #[serde(rename = "$value", with = "sdf_fvec6")]
    pub value: [f64; 6],
}

impl Default for Pose {
    fn default() -> Pose {
        Pose { value: [0f64; 6] }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Size {
    #[serde(rename = "$value", with = "sdf_fvec3")]
    pub value: [f64; 3],
}

#[derive(Debug, Deserialize, Clone)]
pub struct Xyz {
    #[serde(rename = "$value", with = "sdf_ivec3")]
    pub value: [u32; 3],
}

#[derive(Debug, Deserialize, Clone)]
pub struct Inertia {
    pub ixx: Ixx,
    pub ixy: Ixy,
    pub ixz: Ixz,
    pub iyy: Iyy,
    pub iyz: Iyz,
    pub izz: Izz,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Inertial {
    pub pose: Pose,
    pub inertia: Inertia,
    pub mass: Mass,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Geometry {
    Box { size: Size },
    Cylinder { radius: Radius, length: Length },
}

impl Default for Geometry {
    fn default() -> Geometry {
        Geometry::Box {
            size: Size {
                value: [0.0f64, 0.0, 0.0],
            },
        }
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Collision {
    pub name: String,
    pub pose: Pose,
    pub geometry: Geometry,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Script {
    pub uri: Uri,
    pub name: Name,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Material {
    pub script: Script,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Visual {
    pub name: String,
    pub pose: Pose,
    pub geometry: Geometry,
    pub material: Material,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Link {
    #[serde(default)]
    pub pose: Pose,
    pub inertial: Option<Inertial>,
    #[serde(rename = "collision", default)]
    pub collisions: Vec<Collision>,
    #[serde(rename = "visual", default)]
    pub visuals: Vec<Visual>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dynamics {
    pub damping: Damping,
    pub friction: Friction,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Limit {
    pub lower: Lower,
    pub upper: Upper,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Axis {
    pub dynamics: Dynamics,
    pub limit: Option<Limit>,
    pub xyz: Xyz,
    pub use_parent_model_frame: UseParentModelFrame,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Joint {
    pub name: String,
    #[serde(rename = "type")]
    pub joint_type: String,
    pub parent: Parent,
    pub child: Child,
    pub axis: Axis,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Model {
    pub name: String,
    #[serde(rename = "link", default)]
    pub links: Vec<Link>,
    #[serde(rename = "joint", default)]
    pub joints: Vec<Joint>,
}
