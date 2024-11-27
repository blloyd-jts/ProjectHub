use leptos::*;

pub fn error_fallback() -> Box<dyn Fn(RwSignal<Errors>) -> HtmlElement<html::Div>> {
    Box::new(|errors: RwSignal<Errors>| {
        view! {
            <div>
                <ul>
                    {move || {
                        errors
                            .with(|errors| {
                                errors
                                    .iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            })
                    }}
                </ul>
            </div>
        }
    })
}