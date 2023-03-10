use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,
        // reproduce the error:
        <AsChildren>
            <p>"Children passed as children"</p>
        </AsChildren>
        // an attempted work-around:
        <AsAttr>
            <p>"Children passed as attribute"</p>
        </AsAttr>
    }
}

#[component]
fn AsAttr(cx: Scope, children: Children) -> impl IntoView {
    view! {
        cx,
        // Attempting to pass children as an attribute directly instead of the normal way gives a
        // mismatched type error:
        <Suspense
            fallback={move || view!{cx, <p>"Loading..."</p>}}
            children={children} // error[E0308]: mismatched types
                                // ...
                                //    = note: expected struct `Box<(dyn std::ops::Fn(leptos::Scope) -> Fragment + 'static)>`
                                //               found struct `Box<(dyn FnOnce(leptos::Scope) -> Fragment + 'static)>`
        />
    }
}

#[component]
fn AsChildren(cx: Scope, children: Children) -> impl IntoView {
    view! {
        cx,
        // OR attempting to use children in the standard way gives a move error:
        <Suspense fallback={move || view!{cx, <p>"Loading..."</p>}}>
            {children(cx)} // FIXME: errors here with:
                           // error[E0507]: cannot move out of `children`, a captured variable in an `Fn` closure
        </Suspense>
    }
}
