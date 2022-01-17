use lazy_static::lazy_static;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use crate::utils::config::CONFIG;

lazy_static! {
    pub static ref GET_ICONS: GetIcons = {
        let mdi: MdiIcons = serde_json::from_str(include_str!("../configs/mdiIcons.json")).unwrap();
        let cfg = &CONFIG;
        let mut fa: FaIcons = serde_json::from_str(include_str!("../configs/faIcons.json"))
            .expect("Oh no, got an Err!");

        let myval = match &cfg.default_icon_component {
            Value::String(str) => str.clone(),
            _ => "fa-".to_string(),
        };
        fa.sizes.is_medium =
            Value::String(format!("{}{}", myval, fa.sizes.is_medium.as_str().unwrap()));
        fa.sizes.is_large =
            Value::String(format!("{}{}", myval, fa.sizes.is_large.as_str().unwrap()));
        fa.icon_prefix = Value::String(myval);
        GetIcons { mdi, fa }
    };
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes {
    pub default: Value,
    #[serde(rename = "is-small")]
    pub is_small: Value,
    #[serde(rename = "is-medium")]
    pub is_medium: Value,
    #[serde(rename = "is-large")]
    pub is_large: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalIcons {
    pub information: String,
    pub alert: String,
    #[serde(rename = "alert-circle")]
    pub alert_circle: String,
    #[serde(rename = "chevron-right")]
    pub chevron_right: String,
    #[serde(rename = "chevron-left")]
    pub chevron_left: String,
    #[serde(rename = "chevron-down")]
    pub chevron_down: String,
    #[serde(rename = "eye-off")]
    pub eye_off: String,
    #[serde(rename = "menu-down")]
    pub menu_down: String,
    #[serde(rename = "menu-up")]
    pub menu_up: String,
    #[serde(rename = "close-circle")]
    pub close_circle: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MdiIcons {
    pub sizes: Sizes,
    pub icon_prefix: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaIcons {
    pub sizes: Sizes,
    pub icon_prefix: Value,
    pub internal_icons: InternalIcons,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIcons {
    pub mdi: MdiIcons,
    pub fa: FaIcons,
}
pub enum IconOfType {
    mdi(MdiIcons),
    fa(FaIcons),
}

impl GetIcons{
    pub fn get(&self, str: &str) -> IconOfType{
        let ret:IconOfType = match str {
            "foo" => IconOfType::mdi(self.mdi.clone()),
            "bar" => IconOfType::fa(self.fa.clone()),
            _ => panic!("unknown field"),
         };
         ret
    }
}

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mdi = &GET_ICONS.mdi;
        assert_eq!(mdi.sizes.is_medium, "mdi-36px");
    }
}

mod tests2 {
    use super::*;
    #[test]
    fn it_works() {
        let mdi = &GET_ICONS.fa;
        assert_eq!(mdi.sizes.is_medium, "fa-lg");
    }
}
