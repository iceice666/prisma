use std::{cell::RefCell, rc::Rc};

use mlua::prelude::*;

use crate::{config::Config, core::Scene, textures::Textures};

mod camera;
mod scene;
mod textures;
mod utils;

pub struct Scripting {
    lua: Lua,
}

impl Scripting {
    pub fn new(textures: Rc<RefCell<Textures>>) -> LuaResult<Self> {
        let lua = Lua::new();

        textures::init(&lua, textures)?;

        let camera = lua.create_table()?;
        lua.globals().set("camera", camera)?;

        let scene = Scene::new();
        lua.globals().set("scene", scene)?;

        Ok(Self { lua })
    }

    pub fn load(self, config: &Config, script: &str) -> LuaResult<Scene> {
        self.lua.load(script).exec()?;
        let camera = camera::load(&self.lua, config)?;
        let mut scene: Scene = self.lua.globals().get("scene")?;
        scene.set_camera(camera);
        Ok(scene)
    }
}
