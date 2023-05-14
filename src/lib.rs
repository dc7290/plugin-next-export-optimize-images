use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::{cell::RefCell, rc::Rc};

use serde_json::json;
use swc_core::{
    ecma::{
        ast::Program,
        visit::{as_folder, FoldWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
pub fn next_export_optimize_images_plugin_transform(
    program: Program,
    _metadata: TransformPluginProgramMetadata,
) -> Program {
    let remote_images = Rc::new(RefCell::new(Vec::<String>::new()));

    let mut result: Option<Program> = Option::None;
    {
        let mut visitor = next_image_identifiers::IdentifierVisitor::new(Rc::clone(&remote_images));
        result.replace(program.fold_with(&mut as_folder(&mut visitor)));
    }

    let remote_images_unwrapped = Rc::try_unwrap(remote_images)
        .unwrap_or_default()
        .into_inner();

    if remote_images_unwrapped.is_empty() {
        return result.unwrap();
    }

    // Returns the current directory as a `PathBuf`.
    let mut path = env::current_dir().expect("Failed to get current directory");
    path.push("next-export-optimize-images-remote-list.nd.json");

    // Write to file
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&path)
        .unwrap();

    // For each string in remote_images, convert to JSON and write to file
    for image in remote_images_unwrapped {
        // Convert the data to JSON
        let data = json!({ "url": image });

        writeln!(file, "{}", data).expect("Unable to write data");
    }

    return result.unwrap();
}
