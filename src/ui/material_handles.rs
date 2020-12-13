use crate::data::AssetType;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct MaterialHandles {
    material_handles: HashMap<AssetType, Handle<ColorMaterial>>,
}

impl MaterialHandles {
    pub fn get(&self, asset_type: AssetType) -> Option<&Handle<ColorMaterial>> {
        self.material_handles.get(&asset_type)
    }

    pub fn insert(
        &mut self,
        asset_type: AssetType,
        material_handle: Handle<ColorMaterial>,
    ) -> Option<Handle<ColorMaterial>> {
        self.material_handles.insert(asset_type, material_handle)
    }
}
