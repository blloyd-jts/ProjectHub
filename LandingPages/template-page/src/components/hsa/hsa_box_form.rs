use leptos::*;
// use leptos_router::*;

#[component]
pub fn HsaBoxForm() -> impl IntoView {
    view! {
        <div class="py-6 sm:py-8">
            <div class="relative mx-auto max-w-7xl px-6 py-8 sm:py-12 lg:px-8">
            
                // Background border effect
                <div class="absolute inset-px rounded-lg"></div>

                // Main container with shadow, border, and padding
                <div class="relative rounded-lg shadow ring-1 ring-black/5 p-6 sm:p-8 bg-white">
                    <div class="mx-auto max-w-2xl text-center">

                        // Heading
                        <h2 class="text-4xl font-semibold tracking-tight text-gray-900 sm:text-5xl">"HSA Forms"</h2>

                        // Subheading
                        <p class="mt-6 text-base/7 text-gray-600">"Choose the appropriate claim form from the list below:"</p>
                    </div>

                    // Form 1 Container
                    <div class="mt-10">
                        <dl class="space-y-16 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-16 sm:space-y-0 lg:gap-x-10">
                            <div>
                                <dt class="text-base/7 font-semibold text-gray-900">
                                    <a 
                                    href="#" 
                                    target="_blank" 
                                    rel="noopener noreferrer"
                                    class="text-blue-600 hover:underline"
                                    >
                                    "HSA Form #1"
                                    </a>
                                </dt>
                                <dd class="mt-2 text-base/7 text-gray-600">"Form 1 description."</dd>
                            </div>
                        </dl>
                    </div>

                    // Form 2 Container
                    <div class="mt-10">
                        <dl class="space-y-16 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-16 sm:space-y-0 lg:gap-x-10">
                            <div>
                                <dt class="text-base/7 font-semibold text-gray-900">
                                    <a 
                                    href="#" 
                                    target="_blank" 
                                    rel="noopener noreferrer"
                                    class="text-blue-600 hover:underline"
                                    >
                                    "HSA Form #2"
                                    </a>
                                </dt>
                                <dd class="mt-2 text-base/7 text-gray-600">"Form 2 description."</dd>
                            </div>
                        </dl>
                    </div>

                    // Form 3 Container
                    <div class="mt-10">
                        <dl class="space-y-16 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-16 sm:space-y-0 lg:gap-x-10">
                            <div>
                                <dt class="text-base/7 font-semibold text-gray-900">
                                    <a 
                                    href="#" 
                                    target="_blank" 
                                    rel="noopener noreferrer"
                                    class="text-blue-600 hover:underline"
                                    >
                                    "HSA Form #3"
                                    </a>
                                </dt>
                                <dd class="mt-2 text-base/7 text-gray-600">"Form 3 description."</dd>
                            </div>
                        </dl>
                    </div>

                    // Form 4 Container
                    <div class="mt-10">
                        <dl class="space-y-16 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-16 sm:space-y-0 lg:gap-x-10">
                            <div>
                                <dt class="text-base/7 font-semibold text-gray-900">
                                    <a 
                                    href="#" 
                                    target="_blank" 
                                    rel="noopener noreferrer"
                                    class="text-blue-600 hover:underline"
                                    >
                                    "HSA Form #4"
                                    </a>
                                </dt>
                                <dd class="mt-2 text-base/7 text-gray-600">"Form 4 description."</dd>
                            </div>
                        </dl>
                    </div>
                </div>
            </div>
        </div>
    }
}