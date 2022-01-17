use yew::prelude::*;
use crate::utils::helpers::Type;
use crate::utils::icons::{GET_ICONS, IconOfType};
use crate::utils::config::CONFIG;

pub struct ByIcon;

const KEY: &str = "RitesWay.icon.self";

pub enum Msg {
    // Add(String),
    // Edit((usize, String)),
    // Remove(usize),
    // SetFilter(Filter),
    // ToggleAll,
    // ToggleEdit(usize),
    // Toggle(usize),
    // ClearCompleted,
    // Focus,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    my_type: Type,
    component: String,
    pack: String,
    icon: String,
    size: String,
    customSize: String,
    customClass: String,
    both: bool // This is used internally to show both MDI and FA icon
}

impl Component for ByIcon {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // match msg {
        //     Msg::Add(description) => {
        //         if !description.is_empty() {
        //             let entry = Entry {
        //                 description: description.trim().to_string(),
        //                 completed: false,
        //                 editing: false,
        //             };
        //             self.state.entries.push(entry);
        //         }
        //     }
        // }
        // LocalStorage::set(KEY, &self.state.entries).expect("failed to set");
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let cls = "rootClasses";
        let newPack = {
            if &_ctx.props().pack==""{
                &CONFIG.default_icon_pack
            }
            else {
                &_ctx.props().pack
            }
        };
        let icon_config:&IconOfType = &GET_ICONS.get(newPack);
        let icon_prefix = match icon_config{
            IconOfType::mdi(x) => x.icon_prefix.clone(),
            IconOfType::fa(y) => y.icon_prefix.as_str().unwrap().to_string(),
        };
        let new_icon = {
            if !&_ctx.props().both{
                format!("{}{}",icon_prefix,&_ctx.props().icon)
            }
            else{
                match icon_config{
                    IconOfType::mdi(x) => format!("{}{}",icon_prefix,&_ctx.props().icon),
                    IconOfType::fa(y) => match &*_ctx.props().icon.clone() {
                        "information"=>  format!("{}{}",icon_prefix,y.internal_icons.information),
                        "alert"=>  format!("{}{}",icon_prefix,y.internal_icons.alert),
                        "alert-circle"=>  format!("{}{}",icon_prefix,y.internal_icons.alert_circle),
                        "chevron-right"=>  format!("{}{}",icon_prefix,y.internal_icons.chevron_right),
                        "chevron-left"=>  format!("{}{}",icon_prefix,y.internal_icons.chevron_left),
                        "chevron-down"=>  format!("{}{}",icon_prefix,y.internal_icons.chevron_down),
                        "eye-off"=>  format!("{}{}",icon_prefix,y.internal_icons.eye_off),
                        "menu-down"=>  format!("{}{}",icon_prefix,y.internal_icons.menu_down),
                        "menu-up"=>  format!("{}{}",icon_prefix,y.internal_icons.menu_up),
                        "close-circle"=>  format!("{}{}",icon_prefix,y.internal_icons.close_circle),
                        _ => format!("{}{}",icon_prefix,&_ctx.props().icon)
                    }
                    // format!("{}{}",icon_prefix,y.internal_icons),
                }
            }
        };

            // let newType= {
            //     let splitType= match &_ctx.props().my_type{
            //         Type::None => todo!(),
            //         Type::String(str) => str.clone().split('-'),
            //         Type::Object(_) => todo!(),
            //     };
                // if (typeof this.type === 'string') {
                //     splitType = this.type.split('-')
                // } else {
                //     for (let key in this.type) {
                //         if (this.type[key]) {
                //             splitType = key.split('-')
                //             break
                //         }
                //     }
                // }
                // if (len(splitType) <= 1) {return ""}
                // let st = splitType.join('-');
                // return format!("has-text-{}",st)
            // },
        //     newCustomSize() {
        //         return this.customSize || this.customSizeByPack
        //     },
        //     customSizeByPack() {
        //         if (this.iconConfig && this.iconConfig.sizes) {
        //             if (this.size && this.iconConfig.sizes[this.size] !== undefined) {
        //                 return this.iconConfig.sizes[this.size]
        //             } else if (this.iconConfig.sizes.default) {
        //                 return this.iconConfig.sizes.default
        //             }
        //         }
        //         return null
        //     },
        //     useIconComponent() {
        //         return this.component || config.defaultIconComponent
        //     }
        // },



        let new_type =  "" ;
        let size =  &_ctx.props().size;
        html! {
            <template>
            <span class={classes!("icon", new_type, size)}>
                // <i
                //     v-if="!useIconComponent"
                //     :class="[newPack, newIcon, newCustomSize, customClass]"/>
        
                // <component
                //     v-else
                //     :is="useIconComponent"
                //     :icon="[newPack, newIcon]"
                //     :size="newCustomSize"
                //     :class="[customClass]"/>
            </span>
        </template>
        }
    }
}
