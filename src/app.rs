#[leptos::component]
pub(crate) fn App() -> impl leptos::IntoView {
    leptos::view! {
        <Header />
        <main class="flex-1"></main>
        <footer class=""></footer>
    }
}

#[leptos::component]
pub(crate) fn Header() -> impl leptos::IntoView {
    leptos::view! {
        <header class="w-full h-14 flex flex-row items-center justify-center">
            <nav class="text-white flex flex-row justify-between items-center w-4/5 px-4 h-full  ">
                <a href="/">
                    <h1 class="text-2xl">CSR</h1>
                </a>

                <Search />
                <Burger />
            </nav>
        </header>
    }
}

#[leptos::component]
pub(crate) fn Search() -> impl leptos::IntoView {
    leptos::view! {
        <div class="flex relative flex-row items-center w-2/4 rounded-full">
            <input
                type="text"
                name="Search"
                id="search"
                class="rounded-full flex-1 placeholder:text-start px-2"
                placeholder="buscar..."
            />
            <button class="h-6 w-6 absolute right-0 rounded-full items-center hover:outline justify-center flex">
                <div class="flex items-center over scale-50">

                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="w-6 h-6"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z"
                        />
                    </svg>

                </div>
            </button>
        </div>
    }
}

#[leptos::component]
pub(crate) fn Burger() -> impl leptos::IntoView {
    leptos::view! {
        <input
            type="checkbox"
            name="Burger"
            id="burger"
            class="checked:scale-150 hidden peer/invisible"
        />
        <label
            for="burger"
            class="absolute top-0 left-0 w-full h-full -z-50 peer-checked:invisible"
        />
        <button class="w-6 h-6 rounded-full text-center ">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="w-6 h-6 rounded-full"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                />
            </svg>

        </button>
        <menu class="relative flex-row items-center justify-center hidden"></menu>
    }
}
