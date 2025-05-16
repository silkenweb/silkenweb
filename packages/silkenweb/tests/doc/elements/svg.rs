use silkenweb::{
    dom::Dry,
    elements::svg::{
        attributes::Presentation,
        path,
        path::{Data, Offset::Abs},
        Path,
    },
};

pub fn path_data() {
    let path: Path<Dry> = path().d(Data::new()
        .move_to(Abs, 10.0, 10.0)
        .lines_to(Abs, [(20.0, 20.0), (30.0, 30.0)]));

    assert_eq!(
        r#"<path d="M 10,10 L 20,20 30,30"></path>"#,
        path.freeze().to_string()
    );
}
