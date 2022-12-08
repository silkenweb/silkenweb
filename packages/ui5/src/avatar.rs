use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use parse_display::Display;
use silkenweb::{
    attribute::{AsAttribute, Attribute},
    elements::CustomEvent,
    node::{
        element::{Element, GenericElement, ParentElement},
        Node,
    },
    prelude::{ElementEvents, HtmlElement, HtmlElementEvents},
    value::{SignalOrValue, Value},
    AriaElement, Element, ElementEvents, HtmlElement, HtmlElementEvents, Value,
};
use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};

use self::elements::{ui5_avatar, ui5_avatar_group, Ui5AvatarGroup};
use crate::{
    icon::Icon,
    macros::{attributes0, attributes1},
    SELECTED_ID,
};

#[derive(Display)]
#[display(style = "CamelCase")]
pub enum ColorScheme {
    Accent1,
    Accent2,
    Accent3,
    Accent4,
    Accent5,
    Accent6,
    Accent7,
    Accent8,
    Accent9,
    Accent10,
}

impl Attribute for ColorScheme {
    fn text(&self) -> Option<Cow<str>> {
        Some(self.to_string().into())
    }
}

impl AsAttribute<ColorScheme> for ColorScheme {}

#[derive(Display)]
#[display(style = "CamelCase")]
pub enum Shape {
    Circle,
    Square,
}

impl Attribute for Shape {
    fn text(&self) -> Option<Cow<str>> {
        Some(self.to_string().into())
    }
}

impl AsAttribute<Shape> for Shape {}

#[derive(Display)]
#[display(style = "UPPERCASE")]
pub enum Size {
    Xs,
    S,
    M,
    L,
    Xl,
}

impl Attribute for Size {
    fn text(&self) -> Option<Cow<str>> {
        Some(self.to_string().into())
    }
}

impl AsAttribute<Size> for Size {}

#[derive(Display, Value)]
#[display(style = "CamelCase")]
pub enum GroupType {
    Group,
    Individual,
}

impl Attribute for GroupType {
    fn text(&self) -> Option<Cow<str>> {
        Some(self.to_string().into())
    }
}

impl AsAttribute<GroupType> for GroupType {}

mod elements {
    use silkenweb::{custom_html_element, elements::CustomEvent, parent_element};

    use super::{ColorScheme, GroupClicked, GroupType, Shape, Size};
    use crate::icon::Icon;

    custom_html_element!(
        ui5_avatar = {
            dom_type: web_sys::HtmlElement;
            attributes {
                accessible_name: String,
                color_scheme: ColorScheme,
                icon: Icon,
                initials: String,
                interactive: bool,
                shape: Shape,
                size: Size,
            };
        }
    );

    parent_element!(ui5_avatar);

    custom_html_element!(
        ui5_avatar_group = { dom_type: web_sys::HtmlElement;
            attributes {
                r#type: GroupType,
            };

            custom_events {
                click: CustomEvent<GroupClicked>,
                overflow: web_sys::CustomEvent,
            };
        }
    );

    parent_element!(ui5_avatar_group);
}

pub fn avatar() -> Avatar {
    Avatar(ui5_avatar())
}

#[derive(Value, Element, HtmlElement, AriaElement, HtmlElementEvents, ElementEvents)]
pub struct Avatar(elements::Ui5Avatar);

impl Avatar {
    attributes0! {accessible_name: String, color_scheme: ColorScheme}

    pub fn icon(self, value: impl SignalOrValue<Item = Icon>) -> Self {
        Self(self.0.icon(value))
    }

    pub fn initials(self, value: impl SignalOrValue<Item = impl AsAttribute<String>>) -> Self {
        Self(self.0.initials(value))
    }

    pub fn interactive(self, value: impl SignalOrValue<Item = impl AsAttribute<bool>>) -> Self {
        Self(self.0.interactive(value))
    }

    pub fn shape(self, value: impl SignalOrValue<Item = impl AsAttribute<Shape>>) -> Self {
        Self(self.0.shape(value))
    }

    pub fn size(self, value: impl SignalOrValue<Item = impl AsAttribute<Size>>) -> Self {
        Self(self.0.size(value))
    }

    pub fn image(self, image: impl SignalOrValue<Item = impl Into<Node>>) -> Self {
        Self(self.0.child(image.map(|img| img.into())))
    }
}

impl From<Avatar> for GenericElement {
    fn from(elem: Avatar) -> Self {
        elem.0.into()
    }
}

pub fn avatar_group<Id>() -> AvatarGroup<Id> {
    AvatarGroup(ui5_avatar_group(), PhantomData)
}

/// Warning: Don't use. This isn't working properly yet.
#[derive(Element)]
pub struct AvatarGroup<Id>(Ui5AvatarGroup, PhantomData<Id>);

impl<Id> AvatarGroup<Id>
where
    Id: Display + FromStr,
    Id::Err: Debug,
{
    attributes1! {1, r#type: GroupType}

    pub fn overflow_button(self, button: impl HtmlElement + Into<Node> + Value + 'static) -> Self {
        Self(self.0.child(button.slot("overflowButton")), self.1)
    }

    pub fn children(self, children: impl IntoIterator<Item = (Id, Avatar)>) -> Self {
        Self(
            self.0.children(
                children
                    .into_iter()
                    .map(|(id, avatar)| avatar.attribute(SELECTED_ID, id.to_string()).0),
            ),
            PhantomData,
        )
    }

    pub fn children_signal(self, children: impl SignalVec<Item = (Id, Avatar)> + 'static) -> Self {
        Self(
            self.0.children_signal(
                children.map(|(id, avatar)| avatar.attribute(SELECTED_ID, id.to_string()).0),
            ),
            PhantomData,
        )
    }

    pub fn on_click(
        self,
        mut f: impl FnMut(CustomEvent<GroupClicked>, <Self as Element>::DomType, Option<Id>) + 'static,
    ) -> Self {
        Self(
            self.0.on_click(move |ev, target| {
                let id = if ev.detail().overflow_button_clicked() {
                    None
                } else {
                    Some(
                        ev.detail()
                            .target_ref()
                            .get_attribute(SELECTED_ID)
                            .unwrap_throw()
                            .parse()
                            .unwrap_throw(),
                    )
                };

                f(ev, target, id)
            }),
            self.1,
        )
    }

    pub fn on_overflow(
        self,
        f: impl FnMut(web_sys::CustomEvent, <Self as Element>::DomType) + 'static,
    ) -> Self {
        Self(self.0.on_overflow(f), self.1)
    }
}

impl<Id> HtmlElement for AvatarGroup<Id> {}

impl<Id> HtmlElementEvents for AvatarGroup<Id> {}

impl<Id> ElementEvents for AvatarGroup<Id> {}

impl<Id> From<AvatarGroup<Id>> for GenericElement {
    fn from(elem: AvatarGroup<Id>) -> Self {
        elem.0.into()
    }
}

#[wasm_bindgen]
extern "C" {
    pub type GroupClicked;

    #[wasm_bindgen(method, getter, js_name = "targetRef")]
    pub fn target_ref(this: &GroupClicked) -> web_sys::HtmlElement;

    #[wasm_bindgen(method, getter, js_name = "overflowButtonClicked")]
    pub fn overflow_button_clicked(this: &GroupClicked) -> bool;
}
