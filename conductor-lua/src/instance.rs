use conductor::instance::{InstanceId, InstanceSettings};
use mlua::prelude::*;

pub struct LInstanceId(pub InstanceId);

impl LuaUserData for LInstanceId {}

pub struct LInstanceSettings(pub InstanceSettings);

impl<'lua> FromLua<'lua> for LInstanceSettings {
	fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
		Ok(LInstanceSettings(match lua_value {
			LuaNil => InstanceSettings::default(),
			LuaValue::Table(table) => {
				let mut settings = InstanceSettings::default();
				if table.contains_key("volume")? {
					settings.volume = table.get("volume")?;
				}
				if table.contains_key("pitch")? {
					settings.pitch = table.get("pitch")?;
				}
				if table.contains_key("position")? {
					settings.position = table.get("position")?;
				}
				if table.contains_key("fade_in_duration")? {
					settings.fade_in_duration = Some(table.get("fade_in_duration")?);
				}
				settings
			}
			_ => panic!(),
		}))
	}
}