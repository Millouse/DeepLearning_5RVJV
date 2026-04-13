use macroquad::prelude::*;

pub struct Assets{
    pub background: Texture2D,
}

pub async fn load_assets() -> Assets{
    let background = load_texture("ressources/pond_background.png").await.unwrap();

    Assets {
        background
    }
}