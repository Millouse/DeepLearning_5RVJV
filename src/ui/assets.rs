use macroquad::prelude::*;

pub struct Assets{
    pub background: Texture2D,
    pub dark_wood: Texture2D,
    pub white_wood: Texture2D
}

pub async fn load_assets() -> Assets{
    let background = load_texture("ressources/pond_background.png").await.unwrap();
    let dark_wood = load_texture("ressources/dark_wood.png").await.unwrap();
    let white_wood = load_texture("ressources/white_wood.png").await.unwrap();

    Assets {
        background,
        dark_wood,
        white_wood
    }
}