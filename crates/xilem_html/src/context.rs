use std::{
    any::{Any, TypeId},
    rc::Rc,
};

use bitflags::bitflags;
use wasm_bindgen::JsCast;
use web_sys::Document;

use xilem_core::{Id, IdPath};

use crate::{app::AppRunner, vecmap::VecMap, Message, HTML_NS, SVG_NS};

// Note: xilem has derive Clone here. Not sure.
pub struct Cx {
    id_path: IdPath,
    pub(crate) templates: VecMap<TypeId, (web_sys::Node, Rc<dyn Any>)>,
    document: Document,
    app_ref: Option<Box<dyn AppRunner>>,
}

pub struct MessageThunk {
    id_path: IdPath,
    app_ref: Box<dyn AppRunner>,
}

bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ChangeFlags: u32 {
        const STRUCTURE = 1;
        const OTHER_CHANGE = 2;
    }
}

impl Cx {
    pub fn new() -> Self {
        Cx {
            id_path: Vec::new(),
            document: crate::document(),
            app_ref: None,
            templates: VecMap::default(),
        }
    }

    pub fn push(&mut self, id: Id) {
        self.id_path.push(id);
    }

    pub fn pop(&mut self) {
        self.id_path.pop();
    }

    #[allow(unused)]
    pub fn id_path(&self) -> &IdPath {
        &self.id_path
    }

    /// Run some logic with an id added to the id path.
    ///
    /// This is an ergonomic helper that ensures proper nesting of the id path.
    pub fn with_id<T, F: FnOnce(&mut Cx) -> T>(&mut self, id: Id, f: F) -> T {
        self.push(id);
        let result = f(self);
        self.pop();
        result
    }

    /// Allocate a new id and run logic with the new id added to the id path.
    ///
    /// Also an ergonomic helper.
    pub fn with_new_id<T, F: FnOnce(&mut Cx) -> T>(&mut self, f: F) -> (Id, T) {
        let id = Id::next();
        self.push(id);
        let result = f(self);
        self.pop();
        (id, result)
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn create_element(&self, ns: &str, name: &str) -> web_sys::Element {
        self.document
            .create_element_ns(Some(ns), name)
            .expect("could not create element")
    }

    pub fn create_html_element(&self, name: &str) -> web_sys::HtmlElement {
        self.create_element(HTML_NS, name).unchecked_into()
    }

    pub fn create_svg_element(&self, name: &str) -> web_sys::SvgElement {
        self.create_element(SVG_NS, name).unchecked_into()
    }

    pub fn message_thunk(&self) -> MessageThunk {
        MessageThunk {
            id_path: self.id_path.clone(),
            app_ref: self.app_ref.as_ref().unwrap().clone_box(),
        }
    }
    pub(crate) fn set_runner(&mut self, runner: impl AppRunner + 'static) {
        self.app_ref = Some(Box::new(runner));
    }
}

impl Default for Cx {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageThunk {
    pub fn push_message(&self, message_body: impl Any + 'static) {
        let message = Message {
            id_path: self.id_path.clone(),
            body: Box::new(message_body),
        };
        self.app_ref.handle_message(message);
    }
}

impl ChangeFlags {
    pub fn tree_structure() -> Self {
        Self::STRUCTURE
    }
}
