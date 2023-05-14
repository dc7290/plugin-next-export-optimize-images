use std::cell::RefCell;
use std::rc::Rc;
use swc_core::ecma::{
    ast::{
        ImportDecl, ImportSpecifier, JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXElementName,
        JSXOpeningElement, Lit,
    },
    visit::VisitMut,
};

#[derive(Debug)]
pub struct IdentifierVisitor {
    next_image_import_name: Option<String>,
    next_image_import_name_legacy: Option<String>,
    remote_images: Rc<RefCell<Vec<String>>>,
}

impl IdentifierVisitor {
    pub fn new(remote_images: Rc<RefCell<Vec<String>>>) -> Self {
        Self {
            next_image_import_name: None,
            next_image_import_name_legacy: None,
            remote_images,
        }
    }
}

const NEXT_IMAGE_IMPORT_NAME: &str = "next/image";
const NEXT_IMAGE_IMPORT_NAME_LEGACY: &str = "next/legacy/image";

impl<'a> VisitMut for IdentifierVisitor {
    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        let src = &n.src.value;
        let is_next_image_import = NEXT_IMAGE_IMPORT_NAME == src;
        let is_next_image_import_legacy = NEXT_IMAGE_IMPORT_NAME_LEGACY == src;
        if is_next_image_import || is_next_image_import_legacy {
            for specifier in &n.specifiers {
                match specifier {
                    ImportSpecifier::Default(default_specifier) => {
                        if is_next_image_import {
                            self.next_image_import_name =
                                Some(default_specifier.local.sym.to_string())
                        } else if is_next_image_import_legacy {
                            self.next_image_import_name_legacy =
                                Some(default_specifier.local.sym.to_string())
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
        if self.next_image_import_name.is_none() && self.next_image_import_name_legacy.is_none() {
            return;
        }

        let element_name = &n.name;

        if let JSXElementName::Ident(ident) = element_name {
            if let Some(ref name) = self.next_image_import_name {
                if &ident.sym.to_string() != name {
                    return;
                }

                for attr in &mut n.attrs {
                    if let JSXAttrOrSpread::JSXAttr(attr) = attr {
                        if let JSXAttrName::Ident(ident) = &attr.name {
                            if &ident.sym.to_string() == "src" {
                                if let Some(JSXAttrValue::Lit(Lit::Str(str))) = &attr.value {
                                    if str.value.starts_with("http") {
                                        self.remote_images.borrow_mut().push(str.value.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if let Some(ref name) = self.next_image_import_name_legacy {
                if &ident.sym.to_string() != name {
                    return;
                }

                for attr in &mut n.attrs {
                    if let JSXAttrOrSpread::JSXAttr(attr) = attr {
                        if let JSXAttrName::Ident(ident) = &attr.name {
                            if &ident.sym.to_string() == "src" {
                                if let Some(JSXAttrValue::Lit(Lit::Str(str))) = &attr.value {
                                    if str.value.starts_with("http") {
                                        self.remote_images.borrow_mut().push(str.value.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
