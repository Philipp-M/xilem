// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use std::{cell::RefCell, rc::Rc};

use crate::{
    attribute::{AttributeScratch, Attributes},
    class::{ClassScratch, Classes},
    document,
    style::{StyleScratch, Styles},
    AnyPod, Pod, ViewCtx,
};
#[cfg(feature = "hydration")]
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

// Lazy access to attributes etc. to avoid allocating unnecessary memory when it isn't needed
// Benchmarks have shown, that this can significantly increase performance and reduce memory usage...
/// This holds all the state for a DOM [`Element`](`crate::interfaces::Element`), it is used for [`DomView::Props`](`crate::DomView::Props`)
pub struct ElementProps {
    pub(crate) attributes: Option<Box<Attributes>>,
    pub(crate) classes: Option<Box<Classes>>,
    pub(crate) styles: Option<Box<Styles>>,
    pub(crate) children: Vec<AnyPod>,
    pub(crate) scratch: Rc<RefCell<ElementScratch>>,
}

impl ElementProps {
    pub fn new(children: Vec<AnyPod>, ctx: &mut ViewCtx) -> Self {
        let attr_size_hint = ctx.modifier_size_hint::<Attributes>();
        let attributes = if attr_size_hint > 0 {
            Some(Box::new(Attributes::new(
                ctx.element_scratch.clone(),
                attr_size_hint,
            )))
        } else {
            None
        };
        let style_size_hint = ctx.modifier_size_hint::<Styles>();
        let styles = if style_size_hint > 0 {
            Some(Box::new(Styles::new(
                ctx.element_scratch.clone(),
                style_size_hint,
            )))
        } else {
            None
        };
        let class_size_hint = ctx.modifier_size_hint::<Classes>();
        let classes = if class_size_hint > 0 {
            Some(Box::new(Classes::new(
                ctx.element_scratch.clone(),
                class_size_hint,
            )))
        } else {
            None
        };
        Self {
            attributes,
            classes,
            styles,
            children,
            scratch: ctx.element_scratch.clone(),
        }
    }
}

impl ElementProps {
    // All of this is slightly more complicated than it should be,
    // because we want to minimize DOM traffic as much as possible (that's basically the bottleneck)
    pub fn update_element(&mut self, element: &web_sys::Element) {
        if let Some(attributes) = &mut self.attributes {
            attributes.apply_attribute_changes(element);
        }
        if let Some(classes) = &mut self.classes {
            classes.apply_class_changes(element);
        }
        if let Some(styles) = &mut self.styles {
            styles.apply_style_changes(element);
        }
    }

    pub fn attributes(&mut self) -> &mut Attributes {
        self.attributes
            .get_or_insert_with(|| Box::new(Attributes::new(self.scratch.clone(), 0)))
    }

    pub fn styles(&mut self) -> &mut Styles {
        self.styles
            .get_or_insert_with(|| Box::new(Styles::new(self.scratch.clone(), 0)))
    }

    pub fn classes(&mut self) -> &mut Classes {
        self.classes
            .get_or_insert_with(|| Box::new(Classes::new(self.scratch.clone(), 0)))
    }
}

impl Pod<web_sys::Element, ElementProps> {
    /// Creates a new Pod with [`web_sys::Element`] as element and `ElementProps` as its [`DomView::Props`](`crate::DomView::Props`)
    pub fn new_element(
        children: Vec<AnyPod>,
        ns: &str,
        elem_name: &str,
        ctx: &mut ViewCtx,
    ) -> Self {
        let element = document()
            .create_element_ns(Some(ns), elem_name)
            .unwrap_throw();

        for child in children.iter() {
            let _ = element.append_child(child.node.as_ref());
        }

        ctx.element_scratch.borrow_mut().was_created = true;

        Self {
            node: element,
            props: ElementProps::new(children, ctx),
        }
    }

    #[cfg(feature = "hydration")]
    pub fn hydrate_element(
        children: Vec<AnyPod>,
        element: web_sys::Node,
        ctx: &mut ViewCtx,
    ) -> Self {
        Self {
            node: element.dyn_into().unwrap_throw(),
            props: ElementProps::new(children, ctx),
        }
    }
}

#[derive(Debug, Default)]
pub struct ElementScratch {
    pub(crate) attributes: AttributeScratch,
    pub(crate) styles: StyleScratch,
    pub(crate) class: ClassScratch,
    #[cfg(feature = "hydration")]
    pub(crate) in_hydration: bool,
    pub(crate) was_created: bool,
}

impl ElementScratch {
    pub fn was_initialized(&self) -> bool {
        let mut was_initialized = self.was_created;
        #[cfg(feature = "hydration")]
        {
            was_initialized |= self.in_hydration;
        }
        was_initialized
    }
}
