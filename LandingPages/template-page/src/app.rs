use leptos::*;
use leptos_meta::*;
use leptos_router::{Router, Routes, Route};

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
                <Content/>
                <PageFooter/>
            </Container>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="bg-gray-100">
            <div class="relative px-6 pt-4 lg:px-8">
                <div class="mx-auto max-w-2xl py-8 sm:py-12 lg:py-16">
                    <div class="hidden sm:mb-8 sm:flex sm:justify-center">
                        <div class="relative rounded-full px-3 py-1 text-sm/6 text-gray-600 ring-1 ring-gray-900/10 hover:ring-gray-900/20">
                            "View the complete Benefits booklet "
                            <a href="http://booklets.ebiteam.com/books/ilyy/#p=1" class="font-semibold text-indigo-600">
                                <span class="absolute inset-0" aria-hidden="true"></span>
                                "here "<span aria-hidden="true">"→"</span>
                            </a>
                        </div>
                    </div>
                    <div class="text-center">
                        <h1 class="text-balance text-5xl font-semibold tracking-tight text-gray-900 sm:text-7xl">
                            "Maximize your benefits with 'Company Name'"
                        </h1>
                        <p class="mt-8 text-pretty text-lg font-medium text-gray-500 sm:text-xl/8">
                            "Click the link below to learn more about the claims process."
                        </p>
                        <div class="mt-10 flex items-center justify-center gap-x-6">
                            <a href="faq" class="text-sm/6 font-semibold text-gray-900">
                                "Learn more "<span aria-hidden="true">"→"</span>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
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
