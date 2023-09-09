use bevy::prelude::*;

use crate::networking::*;

const VEC3_TYPE_ID: u16 = 0;

impl Serializable for Vec3 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.x.to_le_bytes());
        bytes.extend_from_slice(&self.y.to_le_bytes());
        bytes.extend_from_slice(&self.z.to_le_bytes());
        bytes
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
