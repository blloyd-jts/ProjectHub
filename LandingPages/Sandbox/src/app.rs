use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::parent_child::{
    parent_child_home::ParentChildHome, parent_child_write_signal::Parent as ParentWriteSignal,
    parent_child_callback::Parent as ParentCallBack,
};
use crate::components::color_tool::color_home::ColorHome;
use crate::components::faq::two_column_centered_intro::FaqTwoColumnCenteredIntro;
use crate::components::claims::claims_box_form::ClaimsBoxForm;
use crate::components::hsa::hsa_box_form::HsaBoxForm;

#[component]
pub fn Container(children: Children) -> impl IntoView {
    view! { <div class="w-full min-h-screen bg-gray-100 flex flex-col">{children()}</div> }
}

#[component]
pub fn PageHeader() -> impl IntoView {
    view! {
        <header id="page-header">
            <div class="flex items-center gap-x-6 bg-customOrange px-6 py-4 sm:px-3.5 sm:before:flex-1">
                <p class="text-lg/6 text-white">
                    <a href="#">
                        <strong class="font-semibold">"Open Enrollment • June 8 - 9"</strong>
                    </a>
                </p>
                <div class="flex flex-1 justify-end">
                    <button type="button" class="-m-3 p-3 focus-visible:outline-offset-[-4px]">
                        <span class="sr-only">"Dismiss"</span>
                    </button>
                </div>
            </div>
        </header>
    }
}

// #[component]
// pub fn BentoGrid() -> impl IntoView {
//     view! {
//         <div class="py-6 sm:py-8">
//             <div class="mx-auto max-w-2xl px-6 lg:max-w-7xl lg:px-8">
//                 // <h2 class="text-base/7 font-semibold text-indigo-600">Deploy faster</h2>
//                 // <p class="mt-2 max-w-lg text-pretty text-4xl font-semibold tracking-tight text-gray-950 sm:text-5xl">Everything you need to deploy your app</p>
//                 <div class="mt-10 grid grid-cols-1 gap-2 sm:mt-16 lg:grid-cols-6 lg:grid-rows-1">
//                 <div class="relative lg:col-span-3">
//                     <div class="absolute inset-px rounded-lg bg-white max-lg:rounded-t-[2rem] lg:rounded-tl-[2rem]"></div>
//                     <div class="relative flex h-full flex-col overflow-hidden rounded-[calc(theme(borderRadius.lg)+1px)] max-lg:rounded-t-[calc(2rem+1px)] lg:rounded-tl-[calc(2rem+1px)]">
//                     // <img class="h-80 object-cover object-left" src="https://tailwindui.com/plus/img/component-images/bento-01-performance.png" alt=""/>
//                     <div class="p-10 pt-4">
//                         <p class="mt-2 text-lg font-medium tracking-tight text-gray-950">Claims Box</p>
//                         <p class="mt-2 max-w-lg text-sm/6 text-gray-600">Lorem ipsum dolor sit amet, consectetur adipiscing elit. In gravida justo et nulla efficitur, maximus egestas sem pellentesque.</p>
//                     </div>
//                 </div>
//                     <div class="pointer-events-none absolute inset-px rounded-lg shadow ring-1 ring-black/5 max-lg:rounded-t-[2rem] lg:rounded-tl-[2rem]"></div>
//                 </div>
//                 <div class="relative lg:col-span-3">
//                     <div class="absolute inset-px rounded-lg bg-white lg:rounded-tr-[2rem]"></div>
//                     <div class="relative flex h-full flex-col overflow-hidden rounded-[calc(theme(borderRadius.lg)+1px)] lg:rounded-tr-[calc(2rem+1px)]">
//                     // <img class="h-80 object-cover object-left lg:object-right" src="https://tailwindui.com/plus/img/component-images/bento-01-releases.png" alt=""/>
//                     <div class="p-10 pt-4">
//                         <p class="mt-2 text-lg font-medium tracking-tight text-gray-950">HSA Box</p>
//                         <p class="mt-2 max-w-lg text-sm/6 text-gray-600">Curabitur auctor, ex quis auctor venenatis, eros arcu rhoncus massa, laoreet dapibus ex elit vitae odio.</p>
//                     </div>
//                     </div>
//                     <div class="pointer-events-none absolute inset-px rounded-lg shadow ring-1 ring-black/5 lg:rounded-tr-[2rem]"></div>
//                 </div>
//                 // <div class="relative lg:col-span-2">
//                 //     <div class="absolute inset-px rounded-lg bg-white lg:rounded-bl-[2rem]"></div>
//                 //     <div class="relative flex h-full flex-col overflow-hidden rounded-[calc(theme(borderRadius.lg)+1px)] lg:rounded-bl-[calc(2rem+1px)]">
//                 //     // <img class="h-80 object-cover object-left" src="https://tailwindui.com/plus/img/component-images/bento-01-speed.png" alt=""/>
//                 //     <div class="p-10 pt-4">
//                 //         <h3 class="text-sm/4 font-semibold text-indigo-600">Speed</h3>
//                 //         <p class="mt-2 text-lg font-medium tracking-tight text-gray-950">Built for power users</p>
//                 //         <p class="mt-2 max-w-lg text-sm/6 text-gray-600">Sed congue eros non finibus molestie. Vestibulum euismod augue.</p>
//                 //     </div>
//                 //     </div>
//                 //     <div class="pointer-events-none absolute inset-px rounded-lg shadow ring-1 ring-black/5 lg:rounded-bl-[2rem]"></div>
//                 // </div>
//                 // <div class="relative lg:col-span-2">
//                 //     <div class="absolute inset-px rounded-lg bg-white"></div>
//                 //     <div class="relative flex h-full flex-col overflow-hidden rounded-[calc(theme(borderRadius.lg)+1px)]">
//                 //     // <img class="h-80 object-cover" src="https://tailwindui.com/plus/img/component-images/bento-01-integrations.png" alt=""/>
//                 //     <div class="p-10 pt-4">
//                 //         <h3 class="text-sm/4 font-semibold text-indigo-600">Integrations</h3>
//                 //         <p class="mt-2 text-lg font-medium tracking-tight text-gray-950">Connect your favorite tools</p>
//                 //         <p class="mt-2 max-w-lg text-sm/6 text-gray-600">Maecenas at augue sed elit dictum vulputate, in nisi aliquam maximus arcu.</p>
//                 //     </div>
//                 //     </div>
//                 //     <div class="pointer-events-none absolute inset-px rounded-lg shadow ring-1 ring-black/5"></div>
//                 // </div>
//                 // <div class="relative lg:col-span-2">
//                 //     <div class="absolute inset-px rounded-lg bg-white max-lg:rounded-b-[2rem] lg:rounded-br-[2rem]"></div>
//                 //     <div class="relative flex h-full flex-col overflow-hidden rounded-[calc(theme(borderRadius.lg)+1px)] max-lg:rounded-b-[calc(2rem+1px)] lg:rounded-br-[calc(2rem+1px)]">
//                 //     // <img class="h-80 object-cover" src="https://tailwindui.com/plus/img/component-images/bento-01-network.png" alt=""/>
//                 //     <div class="p-10 pt-4">
//                 //         <h3 class="text-sm/4 font-semibold text-indigo-600">Network</h3>
//                 //         <p class="mt-2 text-lg font-medium tracking-tight text-gray-950">Globally distributed CDN</p>
//                 //         <p class="mt-2 max-w-lg text-sm/6 text-gray-600">Aenean vulputate justo commodo auctor vehicula in malesuada semper.</p>
//                 //     </div>
//                 //     </div>
//                 //     <div class="pointer-events-none absolute inset-px rounded-lg shadow ring-1 ring-black/5 max-lg:rounded-b-[2rem] lg:rounded-br-[2rem]"></div>
//                 // </div>
//                 </div>
//             </div>
//         </div>
//     }
// }

#[component]
pub fn PageFooter() -> impl IntoView {
    view! {
        <footer id="page-footer" class="bg-customBlue">
            <div class="mx-auto max-w-7xl overflow-hidden px-6 py-2 sm:py-4 lg:px-8">
                <nav class="-mb-6 flex flex-wrap justify-center gap-x-12 gap-y-3 text-sm/6" aria-label="Footer">
                  <a href="#" class="text-gray-400 hover:text-white">"About"</a>
                  <a href="#" class="text-gray-400 hover:text-white">"Blog"</a>
                  <a href="#" class="text-gray-400 hover:text-white">"Jobs"</a>
                  <a href="#" class="text-gray-400 hover:text-white">"Press"</a>
                  <a href="#" class="text-gray-400 hover:text-white">"Accessibility"</a>
                  <a href="#" class="text-gray-400 hover:text-white">"Partners"</a>
                </nav>
                <div class="mt-16 flex justify-center gap-x-10">
                    <a href="https://facebook.com" class="text-gray-400 hover:text-gray-300">
                        <span class="sr-only">"Facebook"</span>
                        <svg class="size-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path fill-rule="evenodd" d="M22 12c0-5.523-4.477-10-10-10S2 6.477 2 12c0 4.991 3.657 9.128 8.438 9.878v-6.987h-2.54V12h2.54V9.797c0-2.506 1.492-3.89 3.777-3.89 1.094 0 2.238.195 2.238.195v2.46h-1.26c-1.243 0-1.63.771-1.63 1.562V12h2.773l-.443 2.89h-2.33v6.988C18.343 21.128 22 16.991 22 12z" clip-rule="evenodd" />
                        </svg>
                    </a>
                    <a href="https://instagram.com" class="text-gray-400 hover:text-gray-300">
                        <span class="sr-only">"Instagram"</span>
                            <svg class="size-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path fill-rule="evenodd" d="M12.315 2c2.43 0 2.784.013 3.808.06 1.064.049 1.791.218 2.427.465a4.902 4.902 0 011.772 1.153 4.902 4.902 0 011.153 1.772c.247.636.416 1.363.465 2.427.048 1.067.06 1.407.06 4.123v.08c0 2.643-.012 2.987-.06 4.043-.049 1.064-.218 1.791-.465 2.427a4.902 4.902 0 01-1.153 1.772 4.902 4.902 0 01-1.772 1.153c-.636.247-1.363.416-2.427.465-1.067.048-1.407.06-4.123.06h-.08c-2.643 0-2.987-.012-4.043-.06-1.064-.049-1.791-.218-2.427-.465a4.902 4.902 0 01-1.772-1.153 4.902 4.902 0 01-1.153-1.772c-.247-.636-.416-1.363-.465-2.427-.047-1.024-.06-1.379-.06-3.808v-.63c0-2.43.013-2.784.06-3.808.049-1.064.218-1.791.465-2.427a4.902 4.902 0 011.153-1.772A4.902 4.902 0 015.45 2.525c.636-.247 1.363-.416 2.427-.465C8.901 2.013 9.256 2 11.685 2h.63zm-.081 1.802h-.468c-2.456 0-2.784.011-3.807.058-.975.045-1.504.207-1.857.344-.467.182-.8.398-1.15.748-.35.35-.566.683-.748 1.15-.137.353-.3.882-.344 1.857-.047 1.023-.058 1.351-.058 3.807v.468c0 2.456.011 2.784.058 3.807.045.975.207 1.504.344 1.857.182.466.399.8.748 1.15.35.35.683.566 1.15.748.353.137.882.3 1.857.344 1.054.048 1.37.058 4.041.058h.08c2.597 0 2.917-.01 3.96-.058.976-.045 1.505-.207 1.858-.344.466-.182.8-.398 1.15-.748.35-.35.566-.683.748-1.15.137-.353.3-.882.344-1.857.048-1.055.058-1.37.058-4.041v-.08c0-2.597-.01-2.917-.058-3.96-.045-.976-.207-1.505-.344-1.858a3.097 3.097 0 00-.748-1.15 3.098 3.098 0 00-1.15-.748c-.353-.137-.882-.3-1.857-.344-1.023-.047-1.351-.058-3.807-.058zM12 6.865a5.135 5.135 0 110 10.27 5.135 5.135 0 010-10.27zm0 1.802a3.333 3.333 0 100 6.666 3.333 3.333 0 000-6.666zm5.338-3.205a1.2 1.2 0 110 2.4 1.2 1.2 0 010-2.4z" clip-rule="evenodd" />
                        </svg>
                    </a>
                    <a href="https://x.com" class="text-gray-400 hover:text-gray-300">
                        <span class="sr-only">"X"</span>
                            <svg class="size-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path d="M13.6823 10.6218L20.2391 3H18.6854L12.9921 9.61788L8.44486 3H3.2002L10.0765 13.0074L3.2002 21H4.75404L10.7663 14.0113L15.5685 21H20.8131L13.6819 10.6218H13.6823ZM11.5541 13.0956L10.8574 12.0991L5.31391 4.16971H7.70053L12.1742 10.5689L12.8709 11.5655L18.6861 19.8835H16.2995L11.5541 13.096V13.0956Z" />
                            </svg>
                    </a>
                    <a href="https://github.com" class="text-gray-400 hover:text-gray-300">
                        <span class="sr-only">"GitHub"</span>
                            <svg class="size-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd" />
                            </svg>
                    </a>
                    <a href="https://youtube.com" class="text-gray-400 hover:text-gray-300">
                        <span class="sr-only">"YouTube"</span>
                        <svg class="size-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                        <path fill-rule="evenodd" d="M19.812 5.418c.861.23 1.538.907 1.768 1.768C21.998 8.746 22 12 22 12s0 3.255-.418 4.814a2.504 2.504 0 0 1-1.768 1.768c-1.56.419-7.814.419-7.814.419s-6.255 0-7.814-.419a2.505 2.505 0 0 1-1.768-1.768C2 15.255 2 12 2 12s0-3.255.417-4.814a2.507 2.507 0 0 1 1.768-1.768C5.744 5 11.998 5 11.998 5s6.255 0 7.814.418ZM15.194 12 10 15V9l5.194 3Z" clip-rule="evenodd" />
                        </svg>
                    </a>
                </div>
                <p class="mt-10 text-center text-sm/6 text-gray-400">"JTS Financial Services, LLC"</p>
            </div>
        </footer>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <header>
            <nav class="mx-auto flex max-w-7xl items-center justify-between p-6 lg:px-8" aria-label="Global">
                // <!-- Left-side links -->
                <div class="flex flex-1 justify-start lg:gap-x-12">
                    <div class="hidden lg:flex lg:gap-x-6">
                        <div class="relative rounded-lg shadow ring-1 ring-black/5 hover:bg-gray-300 flex items-center h-8 px-4 transition-color duration-500">
                            <a href="/" class="text-sm/6 font-semibold text-gray-900">Home</a>
                        </div>
                        <div class="relative rounded-lg shadow ring-1 ring-black/5 hover:bg-gray-300 flex items-center h-8 px-4 transition-color duration-500">
                            <a href="claims" class="text-sm/6 font-semibold text-gray-900">Claims</a>
                        </div>
                        <div class="relative rounded-lg shadow ring-1 ring-black/5 hover:bg-gray-300 flex items-center h-8 px-4 transition-color duration-500">
                            <a href="hsa" class="text-sm/6 font-semibold text-gray-900">HSA</a>
                        </div>
                        <div class="relative rounded-lg shadow ring-1 ring-black/5 hover:bg-gray-300 flex items-center h-8 px-4 transition-color duration-500">
                            <a href="faq" class="text-sm/6 font-semibold text-gray-900">FAQ</a>
                        </div>
                        <div class="relative rounded-lg shadow ring-1 ring-black/5 hover:bg-gray-300 flex items-center h-8 px-4 transition-color duration-500">
                            <a href="http://booklets.ebiteam.com/books/ilyy/#p=1" class="text-sm/6 font-semibold text-gray-900">Booklet</a>
                        </div>
                    </div>
                    <div class="flex lg:hidden">
                        <button type="button" class="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-gray-700">
                            <span class="sr-only">Open main menu</span>
                            <svg class="size-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                            </svg>
                        </button>
                    </div>
                </div>

                // <!-- Centered logo -->
                <div class="flex justify-center">
                    <a href="https://jtsfs.com" class="-m-1.5 p-1.5">
                        <span class="sr-only">JTS Financial Services</span>
                        <img class="h-16 w-auto" src="/Users/blloyd/Developer Projects/Git Repositories/ProjectHub/LandingPages/Sandbox/assets/jts-logo.png" alt="Company Logo" />
                    </a>
                </div>

                // <!-- Empty space for alignment -->
                <div class="flex flex-1 justify-end"></div>
            </nav>

            // <!-- Mobile menu -->
            <div class="lg:hidden" role="dialog" aria-modal="true">
                // <!-- Background backdrop -->
                <div class="fixed inset-0 z-10"></div>
                <div class="fixed inset-y-0 left-0 z-10 w-full overflow-y-auto bg-white px-6 py-6">
                    <div class="flex items-center justify-between">
                        <div class="flex flex-1">
                            <button type="button" class="-m-2.5 rounded-md p-2.5 text-gray-700">
                                <span class="sr-only">Close menu</span>
                                <svg class="size-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                                </svg>
                            </button>
                        </div>
                        <a href="https://jtsfs.com" class="-m-1.5 p-1.5">
                            <span class="sr-only">JTS Financial Services</span>
                            <img class="h-16 w-auto" src="/Users/blloyd/Developer Projects/Git Repositories/ProjectHub/LandingPages/Sandbox/assets/jts-logo.png" alt="Company Logo" />
                        </a>
                    </div>
                    <div class="mt-6 space-y-2">
                        <div class="relative rounded-lg shadow ring-1 ring-black/5 hover:bg-gray-300 p-6 sm:p-8">
                            <a href="/" class="text-sm/6 font-semibold text-gray-900">Home</a>
                        </div>
                        <div class="relative rounded-lg shadow ring-1 ring-black/5 hover:bg-gray-300 p-6 sm:p-8">
                            <a href="claims" class="text-sm/6 font-semibold text-gray-900">Claims</a>
                        </div>
                        <div class="relative rounded-lg shadow ring-1 ring-black/5 hover:bg-gray-300 p-6 sm:p-8">
                            <a href="hsa" class="text-sm/6 font-semibold text-gray-900">HSA</a>
                        </div>
                        <div class="relative rounded-lg shadow ring-1 ring-black/5 hover:bg-gray-300 p-6 sm:p-8">
                            <a href="faq" class="-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-gray-900 hover:bg-gray-50">FAQ</a>
                        </div>
                        <div class="relative rounded-lg shadow ring-1 ring-black/5 hover:bg-gray-300 p-6 sm:p-8">
                            <a href="http://booklets.ebiteam.com/books/ilyy/#p=1" class="-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-gray-900 hover:bg-gray-50">Booklet</a>
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}

#[component]
pub fn Content() -> impl IntoView {
    view! {
            <Router>
                    <main id="content" class="flex-grow">
                        <Routes>
                            <Route path="" view=HomePage/>
                            <Route path="/parent-child" view=ParentChildHome>
                                <Route path="write-signal" view=ParentWriteSignal/>
                                <Route path="callback" view=ParentCallBack/>
                                <Route path="" view=|| view! {
                                    <p>"Click an example link."</p>
                                } />
                            </Route>
                            <Route path="/color-tool" view=ColorHome />
                            <Route path="/claims" view=ClaimsBoxForm />
                            <Route path="/faq" view=FaqTwoColumnCenteredIntro />
                            <Route path="/hsa" view=HsaBoxForm />
                            <Route path="/*any" view=NotFound/>
                        </Routes>
                    </main>
            </Router>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // content for this welcome page 
            <Container>
                // injects a stylesheet into the document <head>
                // id=leptos means cargo-leptos will hot-reload this stylesheet
                <Stylesheet id="leptos" href="/pkg/leptos-crm.css"/>
                // sets the document title
                <Title text="JTS Landing Page"/>
                <PageHeader/>
                <NavBar/>
                // <BentoGrid/>
                <Content/>
                <PageFooter/>
            </Container>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-3xl mt-16 mb-16 px-4 sm:px-6 lg:max-w-7xl">
            // Profile Section
            <div class="mb-16 divide-y divide-gray-900/10">
                <div class="grid grid-cols-1 gap-x-8 gap-y-8 md:grid-cols-[200px,minmax(0,800px),1fr]">
                    <div class="px-4 sm:px-0">
                        <h2 class="text-base/7 font-semibold text-gray-900">"Profile"</h2>
                        <p class="mt-1 text-sm/6 text-gray-600">
                            "This information will be displayed publicly so be careful what you share."
                        </p>
                    </div>

                    <form class="bg-white shadow-sm ring-1 ring-gray-900/5 sm:rounded-xl col-start-2">
                        <div class="px-4 py-6 sm:p-8">
                            <div class="grid max-w-2xl grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
                                <div class="sm:col-span-4">
                                    <label for="website" class="block text-sm/6 font-medium text-gray-900">"Website"</label>
                                    <div class="mt-2">
                                        <div class="flex rounded-md shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-inset focus-within:ring-indigo-600 sm:max-w-md">
                                            <span class="flex select-none items-center pl-3 text-gray-500 sm:text-sm">"http://"</span>
                                            <input type="text" name="website" id="website"
                                                class="block flex-1 border-0 bg-transparent py-1.5 pl-1 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm/6"
                                                placeholder="www.example.com" />
                                        </div>
                                    </div>
                                </div>

                                <div class="col-span-full">
                                    <label for="about" class="block text-sm/6 font-medium text-gray-900">"About"</label>
                                    <div class="mt-2">
                                        <textarea id="about" name="about" rows="3"
                                            class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6" />
                                    </div>
                                    <p class="mt-3 text-sm/6 text-gray-600">"Write a few sentences about yourself."</p>
                                </div>

                                <div class="col-span-full">
                                    <label for="photo" class="block text-sm/6 font-medium text-gray-900">"Photo"</label>
                                    <div class="mt-2 flex items-center gap-x-3">
                                        <svg class="size-12 text-gray-300" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                                            <path fill-rule="evenodd" d="M18.685 19.097A9.723 9.723 0 0 0 21.75 12c0-5.385-4.365-9.75-9.75-9.75S2.25 6.615 2.25 12a9.723 9.723 0 0 0 3.065 7.097A9.716 9.716 0 0 0 12 21.75a9.716 9.716 0 0 0 6.685-2.653Zm-12.54-1.285A7.486 7.486 0 0 1 12 15a7.486 7.486 0 0 1 5.855 2.812A8.224 8.224 0 0 1 12 20.25a8.224 8.224 0 0 1-5.855-2.438ZM15.75 9a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" clip-rule="evenodd" />
                                        </svg>
                                        <button type="button"
                                            class="rounded-md bg-white px-2.5 py-1.5 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50">
                                            "Change"
                                        </button>
                                    </div>
                                </div>

                                <div class="col-span-full">
                                    <label for="cover-photo" class="block text-sm/6 font-medium text-gray-900">"Cover photo"</label>
                                    <div class="mt-2 flex justify-center rounded-lg border border-dashed border-gray-900/25 px-6 py-10">
                                        <div class="text-center">
                                            <svg class="mx-auto size-12 text-gray-300" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                                                <path fill-rule="evenodd" d="M1.5 6a2.25 2.25 0 0 1 2.25-2.25h16.5A2.25 2.25 0 0 1 22.5 6v12a2.25 2.25 0 0 1-2.25 2.25H3.75A2.25 2.25 0 0 1 1.5 18V6ZM3 16.06V18c0 .414.336.75.75.75h16.5A.75.75 0 0 0 21 18v-1.94l-2.69-2.689a1.5 1.5 0 0 0-2.12 0l-.88.879.97.97a.75.75 0 1 1-1.06 1.06l-5.16-5.159a1.5 1.5 0 0 0-2.12 0L3 16.061Zm10.125-7.81a1.125 1.125 0 1 1 2.25 0 1.125 1.125 0 0 1-2.25 0Z" clip-rule="evenodd" />
                                            </svg>
                                            <div class="mt-4 flex text-sm/6 text-gray-600">
                                                <label for="file-upload"
                                                    class="relative cursor-pointer rounded-md bg-white font-semibold text-indigo-600 focus-within:outline-none focus-within:ring-2 focus-within:ring-indigo-600 focus-within:ring-offset-2 hover:text-indigo-500">
                                                    <span>"Upload a file"</span>
                                                    <input id="file-upload" name="file-upload" type="file" class="sr-only" />
                                                </label>
                                                <p class="pl-1">"or drag and drop"</p>
                                            </div>
                                            <p class="text-xs/5 text-gray-600">"PNG, JPG, GIF up to 10MB"</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="flex items-center justify-end gap-x-6 border-t border-gray-900/10 px-4 py-4 sm:px-8">
                            <button type="button" class="text-sm/6 font-semibold text-gray-900">"Cancel"</button>
                            <button type="submit"
                                class="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                                "Save"
                            </button>
                        </div>
                    </form>
                </div>
            </div>

            // Personal Information Section
            <div class="grid grid-cols-1 gap-x-8 gap-y-8 mb-16 md:grid-cols-[200px,minmax(0,800px),1fr]">
                <div class="px-4 sm:px-0">
                    <h2 class="text-base/7 font-semibold text-gray-900">"Personal Information"</h2>
                    <p class="mt-1 text-sm/6 text-gray-600">"Use a permanent address where you can receive mail."</p>
                </div>

                <form class="bg-white shadow-sm ring-1 ring-gray-900/5 sm:rounded-xl col-start-2">
                    <div class="px-4 py-6 sm:p-8">
                        <div class="grid max-w-2xl grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
                            <div class="sm:col-span-3">
                                <label for="first-name" class="block text-sm/6 font-medium text-gray-900">"First name"</label>
                                <div class="mt-2">
                                    <input type="text" name="first-name" id="first-name" autocomplete="given-name" 
                                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6" />
                                </div>
                            </div>

                            <div class="sm:col-span-3">
                                <label for="last-name" class="block text-sm/6 font-medium text-gray-900">"Last name"</label>
                                <div class="mt-2">
                                    <input type="text" name="last-name" id="last-name" autocomplete="family-name" 
                                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6" />
                                </div>
                            </div>

                            <div class="sm:col-span-4">
                                <label for="email" class="block text-sm/6 font-medium text-gray-900">"Email address"</label>
                                <div class="mt-2">
                                    <input id="email" name="email" type="email" autocomplete="email" 
                                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6" />
                                </div>
                            </div>

                            <div class="sm:col-span-4">
                                <label for="country" class="block text-sm/6 font-medium text-gray-900">"Country"</label>
                                <div class="mt-2">
                                    <select id="country" name="country" autocomplete="country-name" 
                                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:max-w-xs sm:text-sm/6">
                                        <option>"United States"</option>
                                        <option>"Canada"</option>
                                        <option>"Mexico"</option>
                                    </select>
                                </div>
                            </div>

                            <div class="col-span-full">
                                <label for="street-address" class="block text-sm/6 font-medium text-gray-900">"Street address"</label>
                                <div class="mt-2">
                                    <input type="text" name="street-address" id="street-address" autocomplete="street-address" 
                                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6" />
                                </div>
                            </div>

                            <div class="sm:col-span-2 sm:col-start-1">
                                <label for="city" class="block text-sm/6 font-medium text-gray-900">"City"</label>
                                <div class="mt-2">
                                    <input type="text" name="city" id="city" autocomplete="address-level2" 
                                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6" />
                                </div>
                            </div>

                            <div class="sm:col-span-2">
                                <label for="region" class="block text-sm/6 font-medium text-gray-900">"State / Province"</label>
                                <div class="mt-2">
                                    <input type="text" name="region" id="region" autocomplete="address-level1" 
                                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6" />
                                </div>
                            </div>

                            <div class="sm:col-span-2">
                                <label for="postal-code" class="block text-sm/6 font-medium text-gray-900">"ZIP / Postal code"</label>
                                <div class="mt-2">
                                    <input type="text" name="postal-code" id="postal-code" autocomplete="postal-code" 
                                        class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6" />
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="flex items-center justify-end gap-x-6 border-t border-gray-900/10 px-4 py-4 sm:px-8">
                        <button type="button" class="text-sm/6 font-semibold text-gray-900">"Cancel"</button>
                        <button type="submit" 
                            class="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                            "Save"
                        </button>
                    </div>
                </form>
            </div>

            // Notification Section
            <div class="grid grid-cols-1 gap-x-8 gap-y-8 md:grid-cols-[200px,minmax(0,800px),1fr]">
                <div class="px-4 sm:px-0">
                    <h2 class="text-base/7 font-semibold text-gray-900">Notifications</h2>
                    <p class="mt-1 text-sm/6 text-gray-600">"We'll always let you know about important changes, but you pick what else you want to hear about."</p>
                </div>

                <form class="bg-white shadow-sm ring-1 ring-gray-900/5 sm:rounded-xl col-start-2">
                    <div class="px-4 py-6 sm:p-8">
                        <div class="max-w-2xl space-y-10">
                            <fieldset>
                                <legend class="text-sm/6 font-semibold text-gray-900">By Email</legend>
                                <div class="mt-6 space-y-6">
                                    <div class="relative flex gap-x-3">
                                        <div class="flex h-6 items-center">
                                            <input id="comments" name="comments" type="checkbox" class="size-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"/>
                                        </div>
                                        <div class="text-sm/6">
                                            <label for="comments" class="font-medium text-gray-900">Comments</label>
                                            <p class="text-gray-500">Get notified when someones posts a comment on a posting.</p>
                                        </div>
                                    </div>
                                    <div class="relative flex gap-x-3">
                                        <div class="flex h-6 items-center">
                                            <input id="candidates" name="candidates" type="checkbox" class="size-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"/>
                                        </div>
                                        <div class="text-sm/6">
                                            <label for="candidates" class="font-medium text-gray-900">Candidates</label>
                                            <p class="text-gray-500">Get notified when a candidate applies for a job.</p>
                                        </div>
                                    </div>
                                    <div class="relative flex gap-x-3">
                                        <div class="flex h-6 items-center">
                                            <input id="offers" name="offers" type="checkbox" class="size-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"/>
                                        </div>
                                        <div class="text-sm/6">
                                            <label for="offers" class="font-medium text-gray-900">Offers</label>
                                            <p class="text-gray-500">Get notified when a candidate accepts or rejects an offer.</p>
                                        </div>
                                    </div>
                                </div>
                            </fieldset>
                            <fieldset>
                                <legend class="text-sm/6 font-semibold text-gray-900">Push Notifications</legend>
                                <p class="mt-1 text-sm/6 text-gray-600">These are delivered via SMS to your mobile phone.</p>
                                <div class="mt-6 space-y-6">
                                    <div class="flex items-center gap-x-3">
                                        <input id="push-everything" name="push-notifications" type="radio" class="size-4 border-gray-300 text-indigo-600 focus:ring-indigo-600"/>
                                        <label for="push-everything" class="block text-sm/6 font-medium text-gray-900">Everything</label>
                                    </div>
                                    <div class="flex items-center gap-x-3">
                                        <input id="push-email" name="push-notifications" type="radio" class="size-4 border-gray-300 text-indigo-600 focus:ring-indigo-600"/>
                                        <label for="push-email" class="block text-sm/6 font-medium text-gray-900">Same as email</label>
                                    </div>
                                    <div class="flex items-center gap-x-3">
                                        <input id="push-nothing" name="push-notifications" type="radio" class="size-4 border-gray-300 text-indigo-600 focus:ring-indigo-600"/>
                                        <label for="push-nothing" class="block text-sm/6 font-medium text-gray-900">No push notifications</label>
                                    </div>
                                </div>
                            </fieldset>
                        </div>
                    </div>
                    <div class="flex items-center justify-end gap-x-6 border-t border-gray-900/10 px-4 py-4 sm:px-8">
                        <button type="button" class="text-sm/6 font-semibold text-gray-900">Cancel</button>
                        <button type="submit" class="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Save</button>
                    </div>
                </form>
            </div>
        </div>
        // <h1>"Welcome to Leptos!"</h1>
        // <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
