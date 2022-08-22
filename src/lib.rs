use swc_core::{
    ast::{ImportDecl, JSXElement, Program},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
    visit::VisitMut,
};

pub struct TransformVisitor {
    pub is_next_image_import: bool,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_import_decl(&mut self, importdecl: &mut ImportDecl) {
        if let Some(v) = &importdecl.src.raw {
            if v == "\"next/image\"" {
                self.is_next_image_import = true;
            }
        }
    }
    fn visit_mut_jsx_element(&mut self, _: &mut JSXElement) {
        println!("{}", self.is_next_image_import);
    }
}

#[plugin_transform]
pub fn next_export_optimize_images_plugin(
    program: Program,
    _: TransformPluginProgramMetadata,
) -> Program {
    program
}
