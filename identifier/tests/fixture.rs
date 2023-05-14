use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use swc_core::{
    ecma::parser::{EsConfig, Syntax},
    ecma::{transforms::testing::test_fixture, visit::as_folder},
};

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[testing::fixture("tests/fixtures/**/input.jsx")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    let remote_images = Rc::new(RefCell::new(Vec::<String>::new()));

    test_fixture(
        syntax(),
        &|_| {
            let folder = as_folder(next_image_identifiers::IdentifierVisitor::new(Rc::clone(
                &remote_images,
            )));

            return folder;
        },
        &input,
        &output,
        Default::default(),
    );

    assert_eq!(
        Rc::try_unwrap(remote_images).ok().unwrap().into_inner(),
        vec!["https://sample.com/images/test.png".to_string()],
    )
}
