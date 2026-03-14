use bevy::asset::{AssetLoader, LoadContext};
use bevy::asset::io::Reader;
use bevy::prelude::TypePath;
use crate::resources;

#[derive(Default,TypePath)]
pub(crate) struct GameConfigAssetLoader;

impl AssetLoader for GameConfigAssetLoader{
    type Asset = resources::GameConfig;
    type Settings = ();
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        //here we provide an annomus life time, i don't care about its name, let the compiler figure it out
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        //"If this is Ok(value), unwrap it and give me the value. If it's Err(e), return that error immediately from the current function."
        // the ? is early return on error, kinda like throw ex in java.
        reader.read_to_end(&mut bytes).await?;// read file bytes, or return error
        let asset = ron::de::from_bytes::<resources::GameConfig>(&bytes)?;// parse ron, or return error
        Ok(asset)// everything worked, return the asset wrapped in Ok
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }

}