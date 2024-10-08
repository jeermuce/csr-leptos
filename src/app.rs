use leptos::{SignalGetUntracked, SignalUpdate};

#[leptos::component]
pub(crate) fn App() -> impl leptos::IntoView {
    leptos::view! {
        <Header menu_loc=true />
        <main class="flex flex-col flex-nowrap flex-1 gap-4 p-4 mx-auto bg-green-300">
            <Signals_Reactive_TrackeVSUntracked />
            <CardGrid />
        </main>
        <Footer />
    }
}

#[leptos::component]
pub(crate) fn Signals_Reactive_TrackeVSUntracked() -> impl leptos::IntoView {
    let (truth, set_truthy) = leptos::create_signal(false);
    let (count, set_count) = leptos::create_signal(0);
    leptos::view! {
        <div class="flex flex-col justify-center items-center">
            <div class="flex flex-row flex-nowrap gap-3 justify-between items-center">
                <h1 class="text-2xl font-black w-fit">sh: {truth}</h1>
                <h1 class="text-2xl font-black w-fit">re: {move || truth}</h1>
                <h1 class="text-2xl font-black w-fit">nr: {truth.get_untracked()}</h1>
            </div>
            <button on:mousedown=move |_| {
                set_truthy(!truth());
            }>swap</button>
        </div>
        <div class="flex flex-col justify-center items-center">
            <div class="flex flex-row flex-nowrap gap-3 justify-between items-center">
                <h1 class="text-2xl font-black w-fit">sh: {count}</h1>
                <h1 class="text-2xl font-black w-fit">re: {move || count}</h1>
                <h1 class="text-2xl font-black w-fit">nr: {count.get_untracked()}</h1>
            </div>
            <div class="flex flex-row gap-3">
                <button
                    class="w-16 h-fit"
                    on:mousedown=move |_| {
                        set_count.update(|count| *count += 1);
                    }
                >
                    {"+"}
                </button>
                <button
                    class="w-16 h-fit"
                    on:mousedown=move |_| {
                        set_count.update(|count| *count -= 1);
                    }
                >
                    {"-"}
                </button>
            </div>
        </div>
    }
}
#[leptos::component]
pub(crate) fn CardGrid() -> impl leptos::IntoView {
    leptos::view! {
        <div class="columns-[300px]">
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />

        </div>
    }
}

#[leptos::component]
pub(crate) fn Card() -> impl leptos::IntoView {
    leptos::view! { <img src="https://picsum.photos/200/300" alt="image" class="w-full mb-4 mirror" /> }
}

#[leptos::component]
pub(crate) fn Header(menu_loc: bool) -> impl leptos::IntoView {
    leptos::view! {
        <header class="flex flex-row justify-center items-center w-full h-14 bg-black">
            <nav class="flex flex-row justify-between items-center px-4 w-full max-w-screen-lg h-full text-white">
                <a href="/">
                    <h1 class="text-2xl">CSR</h1>
                </a>

                <Search />
                <ul class="hidden rounded-lg overflow-clip items-center justify-center md:flex flex-row ">

                    <li class="hover:bg-gray-600 cursor-pointer p-2">Home</li>
                    <li class="hover:bg-gray-600 cursor-pointer p-2">Home</li>
                    <li class="hover:bg-gray-600 cursor-pointer p-2">Home</li>
                    <li class="hover:bg-gray-600 cursor-pointer p-2">Home</li>

                </ul>
                <Burger loc=menu_loc />
            </nav>
        </header>
    }
}

#[leptos::component]
pub(crate) fn Footer() -> impl leptos::IntoView {
    leptos::view! {
        <footer class="flex flex-row justify-center items-center w-full h-auto bg-white">
            <Header menu_loc=false />
        </footer>
    }
}

#[leptos::component]
pub(crate) fn Search() -> impl leptos::IntoView {
    leptos::view! {
        <div class="flex relative flex-row items-center w-2/4 min-h-14 child:min-h-10">
            <input

                type="text"
                name="Search"
                id="search"
                required
                class="flex-1 px-2 rounded-full appearance-none outline-none placeholder:text-start"
                placeholder="buscar..."
            />

            <button class="flex border-none  hover:bg-gray-600/80 absolute right-0 justify-center items-center rounded-full search-button aspect-square">

                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="w-8 h-8"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z"
                    />
                </svg>
            </button>
        </div>
    }
}
#[leptos::component]
pub(crate) fn Burger(loc: bool) -> impl leptos::IntoView {
    let class_of_ul: &str;
    if loc {
        class_of_ul = "z-50 absolute right-0 top-full overflow-clip bg-black my-2 min-w-fit border border-gray-200 rounded-md shadow-lg";
    } else {
        class_of_ul = "z-50 absolute right-0 bottom-full overflow-clip bg-black my-2 min-w-fit border border-gray-200 rounded-md shadow-lg";
    }

    leptos::view! {
        <details class="relative md:hidden bg-red-500 rounded-full w-fit overflow-visible">
            <summary class="flex  hover:bg-gray-600/80 rounded-full items-center p-2 cursor-pointer">
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
                        d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                    />
                </svg>
            </summary>

            <ul class=class_of_ul>
                <li class="px-4 cursor-pointer py-2 hover:bg-gray-600">{loc}</li>
                <li class="px-4 cursor-pointer py-2 hover:bg-gray-600">Home</li>
                <li class="px-4 cursor-pointer py-2 hover:bg-gray-600">Home</li>
                <li class="px-4 cursor-pointer py-2 hover:bg-gray-600">Home</li>
                <li class="px-4 cursor-pointer py-2 hover:bg-gray-600">Home</li>
            </ul>
        </details>
    }
}
