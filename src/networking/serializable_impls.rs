/* 
 * @Author: Sofia Langer-Osuna
 */
use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use crate::networking::*;

//So other modules don't need to use bevy_trait_query
pub trait RegisterSerializable {
    fn register_serializable<T>(&mut self) 
        -> &mut Self where T: Serializable + Component;
}

impl RegisterSerializable for App {
    fn register_serializable<T>(&mut self) 
        -> &mut Self where T: Serializable + Component {
        self.register_component_as::<dyn Serializable, T>();
        self
    }
}

const VEC3_TYPE_ID: u16 = 0;

impl Serializable for Vec3 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.x.to_le_bytes());
        bytes.extend_from_slice(&self.y.to_le_bytes());
        bytes.extend_from_slice(&self.z.to_le_bytes());
        bytes
    }

    fn get_length(&self) -> usize {
        std::mem::size_of::<f32>() * 3
    }

    fn from_bytes(&mut self, bytes: &[u8]) {
        let mut offset = 0;
        self.x = f32::from_le_bytes([bytes[offset], bytes[offset + 1],
                                    bytes[offset + 2], bytes[offset + 3]]);
        offset += std::mem::size_of::<f32>();
        self.y = f32::from_le_bytes([bytes[offset], bytes[offset + 1],
                                    bytes[offset + 2], bytes[offset + 3]]);
        offset += std::mem::size_of::<f32>();
        self.z = f32::from_le_bytes([bytes[offset], bytes[offset + 1],
                                    bytes[offset + 2], bytes[offset + 3]]);
    }

    fn get_type_id(&self) -> u16 {
        VEC3_TYPE_ID
    }
}

const QUAT_TYPE_ID: u16 = 1;

impl Serializable for Quat {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.x.to_le_bytes());
        bytes.extend_from_slice(&self.y.to_le_bytes());
        bytes.extend_from_slice(&self.z.to_le_bytes());
        bytes.extend_from_slice(&self.w.to_le_bytes());
        bytes
    }

    fn get_length(&self) -> usize {
        std::mem::size_of::<f32>() * 4
    }

    fn from_bytes(&mut self, bytes: &[u8]) {
        let mut offset = 0;
        self.x = f32::from_le_bytes([bytes[offset], bytes[offset + 1],
                                    bytes[offset + 2], bytes[offset + 3]]);
        offset += std::mem::size_of::<f32>();
        self.y = f32::from_le_bytes([bytes[offset], bytes[offset + 1],
                                    bytes[offset + 2], bytes[offset + 3]]);
        offset += std::mem::size_of::<f32>();
        self.z = f32::from_le_bytes([bytes[offset], bytes[offset + 1],
                                    bytes[offset + 2], bytes[offset + 3]]);
        offset += std::mem::size_of::<f32>();
        self.w = f32::from_le_bytes([bytes[offset], bytes[offset + 1],
                                    bytes[offset + 2], bytes[offset + 3]]);
    }

    fn get_type_id(&self) -> u16 {
        QUAT_TYPE_ID
    }
}

const TRANSFORM_TYPE_ID: u16 = 2;

impl Serializable for Transform {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.translation.to_bytes());
        bytes.extend_from_slice(&self.rotation.to_bytes());
        bytes.extend_from_slice(&self.scale.to_bytes());
        bytes
    }

    fn get_length(&self) -> usize {
        std::mem::size_of::<f32>() * 10
    }

    fn from_bytes(&mut self, bytes: &[u8]) {
        let mut offset = 0;
        self.translation.from_bytes(&bytes[offset..]);
        offset += std::mem::size_of::<Vec3>();
        self.rotation.from_bytes(&bytes[offset..]);
        offset += std::mem::size_of::<Quat>();
        self.scale.from_bytes(&bytes[offset..]);
    }

    fn get_type_id(&self) -> u16 {
        TRANSFORM_TYPE_ID
    }
}

//TODO make impls for other types
