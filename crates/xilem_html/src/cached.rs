use crate::{ChangeFlags, Cx, View, ViewMarker};

pub struct Cached<D, VF, UF> {
    data: D,
    child_cb: VF,
    invalidate_cb: UF,
}

pub struct CachedState<T, A, V: View<T, A>> {
    view: V,
    view_state: V::State,
    dirty: bool,
}

impl<D, VF, UF> Cached<D, VF, UF> {
    pub fn new(data: D, invalidate_cb: UF, child_cb: VF) -> Self {
        Cached {
            data,
            invalidate_cb,
            child_cb,
        }
    }
}

impl<D, VF, UF> ViewMarker for Cached<D, VF, UF> {}

impl<T, A, D, V, VF, UF> View<T, A> for Cached<D, VF, UF>
where
    V: View<T, A>,
    VF: Fn(&D) -> V + 'static,
    UF: Fn(&D, &D) -> bool + 'static,
{
    type State = CachedState<T, A, V>;

    type Element = V::Element;

    fn build(&self, cx: &mut Cx) -> (xilem_core::Id, Self::State, Self::Element) {
        // TODO debug_assert or assert?
        debug_assert!(
            std::mem::size_of::<VF>() == 0,
            "The callback is not allowed to be a function pointer or a closure capturing context"
        );
        debug_assert!(
            std::mem::size_of::<UF>() == 0,
            "The callback is not allowed to be a function pointer or a closure capturing context"
        );
        let view = (self.child_cb)(&self.data);
        let (id, view_state, element) = view.build(cx);
        let memoize_state = CachedState {
            view,
            view_state,
            dirty: false,
        };
        (id, memoize_state, element)
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        id: &mut xilem_core::Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        if std::mem::take(&mut state.dirty) || (self.invalidate_cb)(&prev.data, &self.data) {
            let view = (self.child_cb)(&self.data);
            let changed = view.rebuild(cx, &state.view, id, &mut state.view_state, element);
            state.view = view;
            changed
        } else {
            ChangeFlags::empty()
        }
    }

    fn message(
        &self,
        id_path: &[xilem_core::Id],
        state: &mut Self::State,
        event: Box<dyn std::any::Any>,
        app_state: &mut T,
    ) -> xilem_core::MessageResult<A> {
        let r = state
            .view
            .message(id_path, &mut state.view_state, event, app_state);
        if matches!(r, crate::MessageResult::RequestRebuild) {
            state.dirty = true;
        }
        r
    }
}

pub fn cached<T, A, D, V, VF, UF>(data: D, invalidate_cb: UF, view: VF) -> Cached<D, VF, UF>
where
    V: View<T, A>,
    VF: Fn(&D) -> V + 'static,
    UF: Fn(&D, &D) -> bool + 'static,
{
    Cached::new(data, invalidate_cb, view)
}

/// Memoize the view returned by the callback, until the `data` changes (in which case `view` is called again)
pub fn memoize<T, A, D, V, VF>(
    data: D,
    view: VF,
) -> Cached<D, VF, impl Fn(&D, &D) -> bool + 'static>
where
    V: View<T, A>,
    D: PartialEq,
    VF: Fn(&D) -> V + 'static,
{
    Cached::new(data, |prev: &D, cur: &D| prev != cur, view)
}

// TODO we need TAITs for less obscure generic code in function docs...

/// A static/constant view, the callback is only run once when the view is built
pub fn s<T, A, V, VF>(
    view: VF,
) -> Cached<(), impl Fn(&()) -> V + 'static, impl Fn(&(), &()) -> bool + 'static>
where
    V: View<T, A>,
    VF: Fn() -> V + 'static,
{
    Cached::new(
        (),
        |(): &(), (): &()| false,
        move |_: &()| {
            // TODO debug_assert or assert?
            debug_assert!(
                std::mem::size_of::<VF>() == 0,
                "The callback is not allowed to be a function pointer or a closure capturing context"
            );
            view()
        },
    )
}
