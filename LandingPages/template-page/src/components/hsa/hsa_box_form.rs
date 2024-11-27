use leptos::*;
// use leptos_router::*;

#[component]
pub fn HsaBoxForm() -> impl IntoView {
    view! {
        <div class="py-6 sm:py-8">
            <div class="relative mx-auto max-w-7xl px-6 py-8 sm:py-12 lg:px-8">
                // Background border effect
                <div class="absolute inset-px rounded-lg"></div>
                // Main FAQ container with shadow, border, and padding
                <div class="relative rounded-lg shadow ring-1 ring-black/5 p-6 sm:p-8 bg-white">
                    <div class="mx-auto max-w-2xl text-center">
                        <h2 class="text-4xl font-semibold tracking-tight text-gray-900 sm:text-5xl">"HSA Forms"</h2>
                        <p class="mt-6 text-base/7 text-gray-600">"Choose the appropriate HSA form from the list below:"</p>
                    </div>
                    <div class="mt-20">
                        <dl class="space-y-16 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-16 sm:space-y-0 lg:gap-x-10">
                            <div>
                                <dt class="text-base/7 font-semibold text-gray-900">"HSA form link here."</dt>
                                <dd class="mt-2 text-base/7 text-gray-600">"Short description of form here."</dd>
                            </div>
                        </dl>
                    </div>
                    <div class="mt-20">
                        <dl class="space-y-16 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-16 sm:space-y-0 lg:gap-x-10">
                            <div>
                                <dt class="text-base/7 font-semibold text-gray-900">"HSA form link here."</dt>
                                <dd class="mt-2 text-base/7 text-gray-600">"Short description of form here."</dd>
                            </div>
                        </dl>
                    </div>
                    <div class="mt-20">
                        <dl class="space-y-16 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-16 sm:space-y-0 lg:gap-x-10">
                            <div>
                                <dt class="text-base/7 font-semibold text-gray-900">"HSA form link here."</dt>
                                <dd class="mt-2 text-base/7 text-gray-600">"Short description of form here."</dd>
                            </div>
                        </dl>
                    </div>
                </div>
            </div>
        </div>
    }
}