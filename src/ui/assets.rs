use macroquad::prelude::*;

pub struct Assets{
    pub background: Texture2D,
    pub dark_wood: Texture2D,
    pub white_wood: Texture2D,
    pub white_egg: Texture2D,
    pub white_tadpole: Texture2D,
    pub white_frog: Texture2D,
    pub dark_egg: Texture2D,
    pub dark_tadpole: Texture2D,
    pub dark_frog: Texture2D
}

pub async fn load_assets() -> Assets{
    let background = load_texture("ressources/pond_background.png").await.unwrap();
    let dark_wood = load_texture("ressources/dark_wood.png").await.unwrap();
    let white_wood = load_texture("ressources/white_wood.png").await.unwrap();
    let white_egg = load_texture("ressources/white_egg.png").await.unwrap();
    let white_tadpole = load_texture("ressources/white_tadpole.png").await.unwrap();
    let white_frog = load_texture("ressources/white_frog.png").await.unwrap();
    let dark_egg = load_texture("ressources/dark_egg.png").await.unwrap();
    let dark_tadpole = load_texture("ressources/dark_tadpole.png").await.unwrap();
    let dark_frog = load_texture("ressources/dark_frog.png").await.unwrap();

    Assets {
        background,
        dark_wood,
        white_wood,
        white_egg,
        white_tadpole,
        white_frog,
        dark_egg,
        dark_tadpole,
        dark_frog
    }
}