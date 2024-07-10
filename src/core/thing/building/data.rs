use super::BuildingAsset;

pub struct Building {
    
    asset: BuildingAsset,
    
    count: i32,
    active_count: i32,
    
}

impl From<BuildingAsset> for Building {
    
    fn from(asset: BuildingAsset) -> Self {
        
        Self {
            asset,
            count: 0,
            active_count: 0,
        }
        
    }
    
}

impl Building {
    
    pub fn asset(&self) -> &BuildingAsset {
        
        &self.asset
        
    }
    
}