use leptos::*;
use leptos_router::*;
use::regex::Regex;

#[component]
pub fn ClaimsBoxForm() -> impl IntoView {

    view! {
        <div class="py-6 sm:py-8">
            <div class="relative mx-auto max-w-7xl px-6 py-8 sm:py-12 lg:px-8">
                // Background border effect
                <div class="absolute inset-px rounded-lg"></div>
                // Main FAQ container with shadow, border, and padding
                <div class="relative rounded-lg shadow ring-1 ring-black/5 p-6 sm:p-8 bg-white">
                    <div class="mx-auto max-w-2xl text-center">
                        <h2 class="text-4xl font-semibold tracking-tight text-gray-900 sm:text-5xl">"Claim Forms"</h2>
                        <p class="mt-6 text-base/7 text-gray-600">"Choose the appropriate claim form from the list below:"</p>
                    </div>
                    <div class="mt-20">
                        <dl class="space-y-16 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-16 sm:space-y-0 lg:gap-x-10">
                            <div>
                                <dt class="text-base/7 font-semibold text-gray-900">"Claims form link here."</dt>
                                <dd class="mt-2 text-base/7 text-gray-600">"Short description of form here."</dd>
                            </div>
                        </dl>
                    </div>
                    <div class="mt-20">
                        <dl class="space-y-16 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-16 sm:space-y-0 lg:gap-x-10">
                            <div>
                                <dt class="text-base/7 font-semibold text-gray-900">"Claims form link here."</dt>
                                <dd class="mt-2 text-base/7 text-gray-600">"Short description of form here."</dd>
                            </div>
                        </dl>
                    </div>
                    <div class="mt-20">
                        <dl class="space-y-16 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-16 sm:space-y-0 lg:gap-x-10">
                            <div>
                                <dt class="text-base/7 font-semibold text-gray-900">"Claims form link here."</dt>
                                <dd class="mt-2 text-base/7 text-gray-600">"Short description of form here."</dd>
                            </div>
                        </dl>
                    </div>
                </div>
            </div>
        </div>
    }
    // let (email, set_email) = create_signal(String::new());
    // let (is_invalid, set_is_invalid) = create_signal(false);

    // let validate_email = move |value: String| {
    //     let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    //     set_is_invalid(!email_regex.is_match(&value));
    //     set_email(value);
    // };

    // view! {
    //     <div class="max-w-[800px] mx-auto mb-16 relative rounded-lg shadow ring-1 ring-black/5 p-12 bg-white rounded-xl">
    //         // Form Title
    //         <div class="text-center mb-12">
    //             <h2 class="text-4xl font-semibold tracking-tight text-gray-900 sm:text-5xl">"Claims Form"</h2>
    //         </div>

    //         // Form
    //         <form>
    //             // First name and Last name (side by side)
    //             <div class="flex gap-8 mb-6">
    //                 <div class="flex-1">
    //                     <label class="block text-sm mb-2" for="first_name">"First name"</label>
    //                     <input
    //                         type="text"
    //                         id="first_name"
    //                         class="w-full px-3 py-2 border border-gray-200 rounded-md"
    //                     />
    //                 </div>
    //                 <div class="flex-1">
    //                     <label class="block text-sm mb-2" for="last_name">"Last name"</label>
    //                     <input
    //                         type="text"
    //                         id="last_name"
    //                         class="w-full px-3 py-2 border border-gray-200 rounded-md"
    //                     />
    //                 </div>
    //             </div>

    //             // Email address (full width)
    //             <div class="mb-6">
    //                 <label class="block text-sm mb-2" for="email">"Email address"</label>
    //                 <input
    //                     type="email"
    //                     id="email"
    //                     class="w-full px-3 py-2 border border-gray-200 rounded-md"
    //                     value=email
    //                     on:input=move |e| validate_email(event_target_value(&e))
    //                 />
    //             </div>

    //             // Country dropdown (full width)
    //             <div class="mb-6">
    //                 <label class="block text-sm mb-2" for="country">"Country"</label>
    //                 <select
    //                     id="country"
    //                     class="w-full px-3 py-2 border border-gray-200 rounded-md bg-white"
    //                 >
    //                     <option selected>"United States"</option>
    //                     <option>"Canada"</option>
    //                     <option>"Mexico"</option>
    //                 </select>
    //             </div>

    //             // Street address (full width)
    //             <div class="mb-6">
    //                 <label class="block text-sm mb-2" for="street">"Street address"</label>
    //                 <input
    //                     type="text"
    //                     id="street"
    //                     class="w-full px-3 py-2 border border-gray-200 rounded-md"
    //                 />
    //             </div>

    //             // City, State, ZIP (three columns)
    //             <div class="flex gap-8 mb-12">
    //                 <div class="flex-1">
    //                     <label class="block text-sm mb-2" for="city">"City"</label>
    //                     <input
    //                         type="text"
    //                         id="city"
    //                         class="w-full px-3 py-2 border border-gray-200 rounded-md"
    //                     />
    //                 </div>
    //                 <div class="flex-1">
    //                     <label class="block text-sm mb-2" for="state">"State / Province"</label>
    //                     <input
    //                         type="text"
    //                         id="state"
    //                         class="w-full px-3 py-2 border border-gray-200 rounded-md"
    //                     />
    //                 </div>
    //                 <div class="flex-1">
    //                     <label class="block text-sm mb-2" for="zip">"ZIP / Postal code"</label>
    //                     <input
    //                         type="text"
    //                         id="zip"
    //                         class="w-full px-3 py-2 border border-gray-200 rounded-md"
    //                     />
    //                 </div>
    //             </div>

    //             // Action buttons
    //             <div class="flex justify-end items-center gap-4 pt-6 border-t border-gray-200">
    //                 <button type="button" class="text-gray-600 text-sm">
    //                     "Cancel"
    //                 </button>
    //                 <button type="submit" class="bg-indigo-600 text-white px-4 py-2 rounded-lg text-sm">
    //                     "Save"
    //                 </button>
    //             </div>
    //         </form>
    //     </div>
    // }
}