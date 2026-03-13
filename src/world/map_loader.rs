use bevy::asset::{AssetLoader, LoadContext};
use bevy::asset::io::Reader;
use bevy::prelude::{Asset, TypePath};
use serde::Deserialize;


//bevvy needs to identify the types at run time by name, so we add typePath derrivation so bevy can generate it
#[derive(Asset, TypePath, Deserialize, Debug)]
pub(crate) struct MapAsset {
    pub(crate) tiles: Vec<String>,
}
#[derive(Default,TypePath)]
pub(crate) struct MapAssetLoader;
impl AssetLoader for MapAssetLoader {
    /**
        here we have associated types,
    the assetLoader trait needs me to tell it what types i will use, so i give it the types.
    i can think of the asset trait as being defined like this
    trait AssetLoader {
        type Asset;
        type Settings;
        type Error;

        async fn load(...) -> Result<Self::Asset, Self::Error>;
    }
    kinda like how in java with generiks i do AssetLoader<MapAsset, Void, Exception>
    so here Self::Asset will be MapAsset

    */
    type Asset = MapAsset;
    type Settings = ();
    //this defension i can think about it as whatever implements the provided traits, and sense its
    //not known at compile time, we need to put it in the heap with box,
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
        let asset = ron::de::from_bytes::<MapAsset>(&bytes)?;// parse ron, or return error
        Ok(asset)// everything worked, return the asset wrapped in Ok
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
