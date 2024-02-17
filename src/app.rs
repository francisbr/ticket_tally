use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ticket_tally.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        <Router>
            <div class="flex flex-col min-h-screen">
                <NavBar/>

                <main class="flex flex-1 flex-col gap-4 p-4 md:gap-8 md:p-10">
                    <Routes>
                        <Route path="" view=Home/>
                    </Routes>
                </main>

                <Footer/>
            </div>
        </Router>
    }
}

#[component]
fn nav_bar() -> impl IntoView {
    view! {
        <header class="flex items-center h-16 px-4 border-b shrink-0 md:px-6">
            <nav class="items-center gap-2 text-lg font-semibold lg:gap-4">
                <a class="flex items-center gap-2 text-lg font-semibold" href="#" rel="ugc">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="w-4 h-4"
                    >
                        <path d="M3 9h18v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V9Z"></path>
                        <path d="m3 9 2.45-4.9A2 2 0 0 1 7.24 3h9.52a2 2 0 0 1 1.8 1.1L21 9"></path>
                        <path d="M12 3v6"></path>
                    </svg>
                    <span class="sr-only">Acme Inc</span>
                </a>
            </nav>
            <div class="flex items-center w-full gap-4 md:ml-auto md:gap-2 lg:gap-4">
                <a class="text-sm font-medium" href="#" rel="ugc">
                    "Ticket Tally"
                </a>
                <a class="text-sm font-medium" href="#" rel="ugc">
                    Tickets
                </a>
                <a class="text-sm font-medium" href="#" rel="ugc">
                    Team Members
                </a>
                <a class="text-sm font-medium" href="#" rel="ugc">
                    Reports
                </a>
                <button class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 w-10 rounded-full">
                    <img
                        src="/placeholder.svg"
                        width="32"
                        height="32"
                        class="rounded-full"
                        alt="Avatar"
                        style="aspect-ratio:32/32;object-fit:cover"
                    />
                    <span class="sr-only">Toggle user menu</span>
                </button>
            </div>
        </header>
    }
}

#[component]
fn home() -> impl IntoView {
    view! {
        <div class="grid gap-4 md:grid-cols-2">
            <div
                class="rounded-lg bg-card text-card-foreground border-0 shadow-none"
                data-v0-t="card"
            >
                <div class="p-6 flex flex-row items-center space-y-0">
                    <h3 class="whitespace-nowrap tracking-tight text-base font-semibold">
                        Ongoing Sessions
                    </h3>
                    <button class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 w-10 ml-auto rounded-full">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="w-4 h-4"
                        >
                            <path d="M5 12h14"></path>
                            <path d="M12 5v14"></path>
                        </svg>
                        <span class="sr-only">New Session</span>
                    </button>
                </div>
                <div class="p-0">
                    <div class="flex flex-col gap-2">
                        <div
                            class="rounded-lg border bg-card text-card-foreground shadow-sm divide-y"
                            data-v0-t="card"
                        >
                            <div class="flex items-center justify-between p-4">
                                <div class="space-y-1">
                                    <h3 class="whitespace-nowrap tracking-tight text-sm font-medium">
                                        Sprint Planning
                                    </h3>
                                    <p class="text-muted-foreground text-sm font-normal">
                                        Estimating user stories for the upcoming sprint
                                    </p>
                                </div>
                                <button class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-9 rounded-md px-3">
                                    View
                                </button>
                            </div>
                        </div>
                        <div
                            class="rounded-lg border bg-card text-card-foreground shadow-sm divide-y"
                            data-v0-t="card"
                        >
                            <div class="flex items-center justify-between p-4">
                                <div class="space-y-1">
                                    <h3 class="whitespace-nowrap tracking-tight text-sm font-medium">
                                        Backlog Refinement
                                    </h3>
                                    <p class="text-muted-foreground text-sm font-normal">
                                        Estimating backlog items for the next release
                                    </p>
                                </div>
                                <button class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-9 rounded-md px-3">
                                    View
                                </button>
                            </div>
                        </div>
                        <div
                            class="rounded-lg border bg-card text-card-foreground shadow-sm divide-y"
                            data-v0-t="card"
                        >
                            <div class="flex items-center justify-between p-4">
                                <div class="space-y-1">
                                    <h3 class="whitespace-nowrap tracking-tight text-sm font-medium">
                                        Bug Triage
                                    </h3>
                                    <p class="text-muted-foreground text-sm font-normal">
                                        Estimating reported bugs for the current sprint
                                    </p>
                                </div>
                                <button class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-9 rounded-md px-3">
                                    View
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div
                class="rounded-lg bg-card text-card-foreground border-0 shadow-none"
                data-v0-t="card"
            >
                <div class="p-6 flex flex-row items-center space-y-0">
                    <h3 class="whitespace-nowrap tracking-tight text-base font-semibold">
                        Latest Tickets
                    </h3>
                    <form class="flex ml-auto sm:flex-initial">
                        <div class="relative">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="absolute left-2.5 top-2.5 h-4 w-4 text-gray-500 dark:text-gray-400"
                            >
                                <circle cx="11" cy="11" r="8"></circle>
                                <path d="m21 21-4.3-4.3"></path>
                            </svg>
                            <input
                                type="search"
                                class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 pl-8 sm:w-[300px] md:w-[200px] lg:w-[300px]"
                                placeholder="Search tickets..."
                            />
                        </div>
                    </form>
                </div>
                <div class="p-0">
                    <div class="flex flex-col gap-2">
                        <div
                            class="rounded-lg border bg-card text-card-foreground shadow-sm divide-y"
                            data-v0-t="card"
                        >
                            <div class="flex items-center justify-between p-4">
                                <div class="space-y-1">
                                    <h3 class="whitespace-nowrap tracking-tight text-sm font-medium">
                                        Add user authentication
                                    </h3>
                                    <p class="text-muted-foreground text-sm font-normal">
                                        We need to add user authentication to our app to restrict access to certain features.
                                    </p>
                                </div>
                                <button class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-9 rounded-md px-3">
                                    View
                                </button>
                            </div>
                        </div>
                        <div
                            class="rounded-lg border bg-card text-card-foreground shadow-sm divide-y"
                            data-v0-t="card"
                        >
                            <div class="flex items-center justify-between p-4">
                                <div class="space-y-1">
                                    <h3 class="whitespace-nowrap tracking-tight text-sm font-medium">
                                        Update homepage styling
                                    </h3>
                                    <p class="text-muted-foreground text-sm font-normal">
                                        The homepage of our app needs some visual improvements.
                                    </p>
                                </div>
                                <button class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-9 rounded-md px-3">
                                    View
                                </button>
                            </div>
                        </div>
                        <div
                            class="rounded-lg border bg-card text-card-foreground shadow-sm divide-y"
                            data-v0-t="card"
                        >
                            <div class="flex items-center justify-between p-4">
                                <div class="space-y-1">
                                    <h3 class="whitespace-nowrap tracking-tight text-sm font-medium">
                                        Fix database connection issue
                                    </h3>
                                    <p class="text-muted-foreground text-sm font-normal">
                                        Users are experiencing problems accessing our app due to a database connection issue.
                                    </p>
                                </div>
                                <button class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-9 rounded-md px-3">
                                    View
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn footer() -> impl IntoView {
    view! {
        <footer class="flex items-center h-16 border-t px-4 shrink-0 md:px-6">
            <div class="flex items-center gap-2 text-sm text-gray-500 dark:text-gray-400">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="h-4 w-4"
                >
                    <path d="M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z"></path>
                </svg>
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="h-4 w-4"
                >
                    <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"></path>
                    <path d="M9 18c-4.51 2-5-2-7-2"></path>
                </svg>
            </div>
            <div class="ml-auto text-sm text-gray-500 dark:text-gray-400">
                "© 2023 Acme Inc. All rights reserved."
            </div>
        </footer>
    }
}
