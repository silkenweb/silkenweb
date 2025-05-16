#[cfg(feature = "declarative-shadow-dom")]
#[test]
fn component() {
    use silkenweb::{
        dom::Dry,
        elements::html::{div, span},
        node::{
            element::{Const, GenericElement, ParentElement, TextParentElement},
            Component,
        },
    };

    let mut comp = Component::<Dry>::styled(r#"* { color: red }"#);
    let slot_0 = comp.slot(span().text("slot 0"));
    let slot_1 = comp.slot(span().text("slot 1"));

    let comp: GenericElement<Dry, Const> = comp
        .child(div().children([div().child(slot_0), div().child(slot_1)]))
        .into();

    assert_eq!(
        comp.to_string(),
        r#"<div><template shadowroot="open"><style>* { color: red }</style><div><div><slot name="0"></slot></div><div><slot name="1"></slot></div></div></template><span slot="0">slot 0</span><span slot="1">slot 1</span></div>"#
    );
}
