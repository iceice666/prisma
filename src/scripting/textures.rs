use crate::core::{Texture2, Texture3};
use crate::textures::{Image, ImageHdr, Panorama};
use mlua::{prelude::*, UserData};
use std::sync::Arc;

#[derive(FromLua, Clone)]
pub struct Texture2Ptr {
    pub ptr: Arc<dyn Texture2>,
}

impl UserData for Texture2Ptr {}

#[derive(FromLua, Clone)]
pub struct Texture3Ptr {
    pub ptr: Arc<dyn Texture3>,
}

impl UserData for Texture3Ptr {}

pub fn init(lua: &Lua) -> LuaResult<()> {
    let texture_image = lua.create_table()?;
    texture_image.set(
        "new",
        lua.create_function(|_lua, path: String| {
            let image = Image::new(&path).map_err(|err| err.into_lua_err())?;
            Ok(Texture2Ptr {
                ptr: Arc::new(image),
            })
        })?,
    )?;
    lua.globals().set("Image", texture_image)?;

    let texture_image_hdr = lua.create_table()?;
    texture_image_hdr.set(
        "new",
        lua.create_function(|_lua, path: String| {
            let image_hdr = ImageHdr::new(&path).map_err(|err| err.into_lua_err())?;
            Ok(Texture2Ptr {
                ptr: Arc::new(image_hdr),
            })
        })?,
    )?;
    lua.globals().set("ImageHdr", texture_image_hdr)?;

    let texture_panorama = lua.create_table()?;
    texture_panorama.set(
        "new",
        lua.create_function(|_lua, path: String| {
            let panorama = Panorama::new(&path).map_err(|err| err.into_lua_err())?;
            Ok(Texture3Ptr {
                ptr: Arc::new(panorama),
            })
        })?,
    )?;
    lua.globals().set("Panorama", texture_panorama)?;

    Ok(())
}