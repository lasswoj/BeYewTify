use yew::prelude::*;

pub struct ByIcon;


pub struct Props {
    pub component: String,
    pub pack: String,
    pub icon: String,
    pub size: String,
    pub customSize: String,
    pub customClass: String,
    pub both: Boolean
}


impl Component for ByIcon {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <span class="icon" :class="[newType, size]">
                <i
                    v-if="!useIconComponent"
                    :class="[newPack, newIcon, newCustomSize, customClass]"/>
        
                <component
                    v-else
                    :is="useIconComponent"
                    :icon="[newPack, newIcon]"
                    :size="newCustomSize"
                    :class="[customClass]"/>
            </span>
        }
    }
}
