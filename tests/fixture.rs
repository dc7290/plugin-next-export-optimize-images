use std::path::PathBuf;

use swc_core::{testing_transform::test_fixture, visit::as_folder};
use swc_ecma_parser::{EsConfig, Syntax};
use swc_plugin_next_export_optimize_images::TransformVisitor;

#[testing::fixture("tests/fixtures/**/input.jsx")]
fn fixture(input: PathBuf) {
    let output = input.with_file_name("output.js");

    test_fixture(
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        &|_| {
            as_folder(TransformVisitor {
                is_next_image_import: false,
            })
        },
        &input,
        &output,
    );
}
