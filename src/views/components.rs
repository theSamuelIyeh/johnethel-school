use super::svgs;
use html_to_string_macro::html;

pub fn theme_toggle() -> String {
    html! {
      <template x-if="theme == 'light'">
            <button type="button" class="group flex items-center text-neutral-800 hover:text-blue-600 font-medium" x-on:click="theme = 'dark'; localStorage.setItem('theme', theme);">
      <svg class="flex-shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>
      </svg>
    </button>
    </template>
    <template x-if="theme == 'dark'">
    <button type="button" class="group flex items-center text-neutral-800 hover:text-blue-600 font-medium" x-on:click="theme = 'light'; localStorage.setItem('theme', theme);">
      <svg class="flex-shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="4"></circle>
        <path d="M12 2v2"></path>
        <path d="M12 20v2"></path>
        <path d="m4.93 4.93 1.41 1.41"></path>
        <path d="m17.66 17.66 1.41 1.41"></path>
        <path d="M2 12h2"></path>
        <path d="M20 12h2"></path>
        <path d="m6.34 17.66-1.41 1.41"></path>
        <path d="m19.07 4.93-1.41 1.41"></path>
      </svg>
    </button>
    </template>
        }
}

pub fn navbar(page: String) -> String {
    html! {
      // Header
    <header class="flex flex-wrap md:justify-start md:flex-nowrap z-50 w-full py-7 bg-lime-400">
      <nav class="relative max-w-7xl w-full flex flex-wrap md:grid md:grid-cols-12 basis-full items-center px-4 md:px-6 mx-auto" aria-label="Global">
        <div class="md:col-span-3 flex items-center">
        // Logo
          <a class="flex-none rounded-xl text-xl inline-block font-semibold focus:outline-none focus:opacity-80" href="/" aria-label="Johnethel School" hx-boost="true">
          {svgs::logo()}
          </a>
          // End Logo
        </div>

        // Button Group
        <div class="flex items-center gap-x-2 ms-auto py-1 md:ps-6 md:order-3 md:col-span-3">
        <a class="group inline-block rounded-full bg-gradient-to-r from-pink-500 via-red-500 to-yellow-500 p-[2px] dark:text-white text-black hover:text-white focus:outline-none focus:ring active:text-opacity-75 mr-3" href="/login" hx-boost="true">
        <span class="block rounded-full dark:bg-neutral-900 bg-white px-6 py-3 mr-3 text-sm font-medium group-hover:bg-transparent transition-all">
          "Portal"
        </span>
        </a>
        {theme_toggle()}

          <div class="md:hidden">
            <button type="button" class="hs-collapse-toggle size-[38px] flex justify-center items-center text-sm font-semibold rounded-xl border border-gray-200 text-black hover:bg-gray-100 disabled:opacity-50 disabled:pointer-events-none dark:text-white dark:border-neutral-700 dark:hover:bg-neutral-700" data-hs-collapse="#navbar-collapse-with-animation" aria-controls="navbar-collapse-with-animation" aria-label="Toggle navigation">
              <svg class="hs-collapse-open:hidden flex-shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="3" x2="21" y1="6" y2="6"/><line x1="3" x2="21" y1="12" y2="12"/><line x1="3" x2="21" y1="18" y2="18"/></svg>
              <svg class="hs-collapse-open:block hidden flex-shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
            </button>
          </div>
        </div>
        // End Button Group

        // Collapse
        <div id="navbar-collapse-with-animation" class="hs-collapse hidden overflow-hidden transition-all duration-300 basis-full grow md:block md:w-auto md:basis-auto md:order-2 md:col-span-6">
          <div class="flex flex-col gap-y-4 gap-x-0 mt-5 md:flex-row md:justify-center md:items-center md:gap-y-0 md:gap-x-7 md:mt-0">
            <div>
              <a class={"relative inline-block text-neutral-900 hover:text-blue-600 transition-all".to_owned() + if page == "home" { " before:absolute before:bottom-0.5 before:start-0 before:-z-[1] before:w-full before:h-1 before:bg-gradient-to-r before:from-pink-500 before:via-red-500 before:to-yellow-500" } else { "" }} {if page == "home" {r#"aria-current="page""#} else {""}} href="/" hx-boost="true">"Home"</a>
            </div>
            <div>
              <a class={"relative inline-block text-neutral-900 hover:text-blue-600 transition-all".to_owned() + if page == "about" { " before:absolute before:bottom-0.5 before:start-0 before:-z-[1] before:w-full before:h-1 before:bg-gradient-to-r before:from-pink-500 before:via-red-500 before:to-yellow-500" } else { "" }} {if page == "about" {r#"aria-current="page""#} else {""}} href="/about" hx-boost="true">"About"</a>
            </div>
            <div>
              <a class={"relative inline-block text-neutral-900 hover:text-blue-600 transition-all".to_owned() + if page == "gallery" { " before:absolute before:bottom-0.5 before:start-0 before:-z-[1] before:w-full before:h-1 before:bg-gradient-to-r before:from-pink-500 before:via-red-500 before:to-yellow-500" } else { "" }} {if page == "gallery" {r#"aria-current="page""#} else {""}} href="/gallery" hx-boost="true">"Gallery"</a>
            </div>
            <div>
              <a class={"relative inline-block text-neutral-900 hover:text-blue-600 transition-all".to_owned() + if page == "contact" { " before:absolute before:bottom-0.5 before:start-0 before:-z-[1] before:w-full before:h-1 before:bg-gradient-to-r before:from-pink-500 before:via-red-500 before:to-yellow-500" } else { "" }} {if page == "contact" {r#"aria-current="page""#} else {""}} href="/contact" hx-boost="true">"Contact"</a>
            </div>
            <div>
              <a class={"relative inline-block text-neutral-900 hover:text-blue-600 transition-all".to_owned() + if page == "blog" { " before:absolute before:bottom-0.5 before:start-0 before:-z-[1] before:w-full before:h-1 before:bg-gradient-to-r before:from-pink-500 before:via-red-500 before:to-yellow-500" } else { "" }} {if page == "blog" {r#"aria-current="page""#} else {""}} href="/blog" hx-boost="true">"Blog"</a>
            </div>
          </div>
        </div>
        // End Collapse
      </nav>
    </header>
    // End Header
        }
}

pub fn hero() -> String {
    let json_string = r##"data-hs-carousel='{
        "loadingClasses": "opacity-0",
        "isAutoPlay": true
    }'"##
        .to_string();
    html! {
        // Slider
    <div {json_string} class="relative flex-grow">
      <div class="hs-carousel relative overflow-hidden w-full h-full bg-white rounded-lg">
        <div class="hs-carousel-body absolute top-0 bottom-0 start-0 flex flex-nowrap transition-transform duration-700 opacity-0">
          <div class="hs-carousel-slide">
            <div class="flex justify-center h-full bg-gray-100 p-6 dark:bg-neutral-900">
              <span class="self-center text-4xl text-gray-800 transition duration-700 dark:text-white">"First slide"</span>
            </div>
          </div>
          <div class="hs-carousel-slide">
            <div class="flex justify-center h-full bg-gray-200 p-6 dark:bg-neutral-800">
              <span class="self-center text-4xl text-gray-800 transition duration-700 dark:text-white">"Second slide"</span>
            </div>
          </div>
          <div class="hs-carousel-slide">
            <div class="flex justify-center h-full bg-gray-300 p-6 dark:bg-neutral-700">
              <span class="self-center text-4xl text-gray-800 transition duration-700 dark:text-white">"Third slide"</span>
            </div>
          </div>
        </div>
      </div>

      <button type="button" class="hs-carousel-prev hs-carousel:disabled:opacity-50 disabled:pointer-events-none absolute inset-y-0 start-0 inline-flex justify-center items-center w-[46px] h-full text-gray-800 hover:bg-gray-800/10 rounded-s-lg dark:text-white dark:hover:bg-white/10">
        <span class="text-2xl" aria-hidden="true">
          <svg class="flex-shrink-0 size-5" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m15 18-6-6 6-6"></path>
          </svg>
        </span>
        <span class="sr-only">"Previous"</span>
      </button>
      <button type="button" class="hs-carousel-next hs-carousel:disabled:opacity-50 disabled:pointer-events-none absolute inset-y-0 end-0 inline-flex justify-center items-center w-[46px] h-full text-gray-800 hover:bg-gray-800/10 rounded-e-lg dark:text-white dark:hover:bg-white/10">
        <span class="sr-only">"Next"</span>
        <span class="text-2xl" aria-hidden="true">
          <svg class="flex-shrink-0 size-5" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m9 18 6-6-6-6"></path>
          </svg>
        </span>
      </button>

      <div class="hs-carousel-pagination flex justify-center absolute bottom-3 start-0 end-0 space-x-2">
        <span class="hs-carousel-active:bg-blue-700 hs-carousel-active:border-blue-700 size-3 border border-gray-400 rounded-full cursor-pointer dark:border-neutral-600 dark:hs-carousel-active:bg-blue-500 dark:hs-carousel-active:border-blue-500"></span>
        <span class="hs-carousel-active:bg-blue-700 hs-carousel-active:border-blue-700 size-3 border border-gray-400 rounded-full cursor-pointer dark:border-neutral-600 dark:hs-carousel-active:bg-blue-500 dark:hs-carousel-active:border-blue-500"></span>
        <span class="hs-carousel-active:bg-blue-700 hs-carousel-active:border-blue-700 size-3 border border-gray-400 rounded-full cursor-pointer dark:border-neutral-600 dark:hs-carousel-active:bg-blue-500 dark:hs-carousel-active:border-blue-500"></span>
      </div>
    </div>
    // End slider
      }
}

pub fn contact() -> String {
    html! {
        // Contact
    <div class="bg-neutral-900">
      <div class="max-w-5xl px-4 xl:px-0 py-10 lg:py-20 mx-auto">
        // Title
        <div class="max-w-3xl mb-10 lg:mb-14">
          <h2 class="text-white font-semibold text-2xl md:text-4xl md:leading-tight">"Contact us"</h2>
          <p class="mt-1 text-neutral-400">"Whatever your goal - we will get you there."</p>
        </div>
        // End Title

        // Grid
        <div class="grid grid-cols-1 md:grid-cols-2 gap-x-10 lg:gap-x-16">
          <div class="md:order-2 border-b border-neutral-800 pb-10 mb-10 md:border-b-0 md:pb-0 md:mb-0">
            <form>
              <div class="space-y-4">
                // Input
                <div class="relative">
                  <input type="text" id="hs-tac-input-name" class="peer p-4 block w-full bg-neutral-800 border-transparent rounded-lg text-sm text-white placeholder:text-transparent focus:outline-none focus:ring-0 focus:border-transparent disabled:opacity-50 disabled:pointer-events-none
              focus:pt-6
              focus:pb-2
              [&:not(:placeholder-shown)]:pt-6
              [&:not(:placeholder-shown)]:pb-2
              autofill:pt-6
              autofill:pb-2" placeholder="Name" />
                  <label for="hs-tac-input-name" class="absolute top-0 start-0 p-4 h-full text-neutral-400 text-sm truncate pointer-events-none transition ease-in-out duration-100 border border-transparent peer-disabled:opacity-50 peer-disabled:pointer-events-none
                peer-focus:text-xs
                peer-focus:-translate-y-1.5
                peer-focus:text-neutral-400
                peer-[:not(:placeholder-shown)]:text-xs
                peer-[:not(:placeholder-shown)]:-translate-y-1.5
                peer-[:not(:placeholder-shown)]:text-neutral-400">"Name"</label>
                </div>
                // End input

                // Input
                <div class="relative">
                  <input type="email" id="hs-tac-input-email" class="peer p-4 block w-full bg-neutral-800 border-transparent rounded-lg text-sm text-white placeholder:text-transparent focus:outline-none focus:ring-0 focus:border-transparent disabled:opacity-50 disabled:pointer-events-none
              focus:pt-6
              focus:pb-2
              [&:not(:placeholder-shown)]:pt-6
              [&:not(:placeholder-shown)]:pb-2
              autofill:pt-6
              autofill:pb-2" placeholder="Email" />
                  <label for="hs-tac-input-email" class="absolute top-0 start-0 p-4 h-full text-neutral-400 text-sm truncate pointer-events-none transition ease-in-out duration-100 border border-transparent peer-disabled:opacity-50 peer-disabled:pointer-events-none
                peer-focus:text-xs
                peer-focus:-translate-y-1.5
                peer-focus:text-neutral-400
                peer-[:not(:placeholder-shown)]:text-xs
                peer-[:not(:placeholder-shown)]:-translate-y-1.5
                peer-[:not(:placeholder-shown)]:text-neutral-400">"Email"</label>
                </div>
                // End Input

                // Input
                <div class="relative">
                  <input type="text" id="hs-tac-input-company" class="peer p-4 block w-full bg-neutral-800 border-transparent rounded-lg text-sm text-white placeholder:text-transparent focus:outline-none focus:ring-0 focus:border-transparent disabled:opacity-50 disabled:pointer-events-none
              focus:pt-6
              focus:pb-2
              [&:not(:placeholder-shown)]:pt-6
              [&:not(:placeholder-shown)]:pb-2
              autofill:pt-6
              autofill:pb-2" placeholder="Company" />
                  <label for="hs-tac-input-company" class="absolute top-0 start-0 p-4 h-full text-neutral-400 text-sm truncate pointer-events-none transition ease-in-out duration-100 border border-transparent peer-disabled:opacity-50 peer-disabled:pointer-events-none
                peer-focus:text-xs
                peer-focus:-translate-y-1.5
                peer-focus:text-neutral-400
                peer-[:not(:placeholder-shown)]:text-xs
                peer-[:not(:placeholder-shown)]:-translate-y-1.5
                peer-[:not(:placeholder-shown)]:text-neutral-400">"Company"</label>
                </div>
                // End Input

                // Input
                <div class="relative">
                  <input type="text" id="hs-tac-input-phone" class="peer p-4 block w-full bg-neutral-800 border-transparent rounded-lg text-sm text-white placeholder:text-transparent focus:outline-none focus:ring-0 focus:border-transparent disabled:opacity-50 disabled:pointer-events-none
              focus:pt-6
              focus:pb-2
              [&:not(:placeholder-shown)]:pt-6
              [&:not(:placeholder-shown)]:pb-2
              autofill:pt-6
              autofill:pb-2" placeholder="Phone" />
                  <label for="hs-tac-input-phone" class="absolute top-0 start-0 p-4 h-full text-neutral-400 text-sm truncate pointer-events-none transition ease-in-out duration-100 border border-transparent peer-disabled:opacity-50 peer-disabled:pointer-events-none
                peer-focus:text-xs
                peer-focus:-translate-y-1.5
                peer-focus:text-neutral-400
                peer-[:not(:placeholder-shown)]:text-xs
                peer-[:not(:placeholder-shown)]:-translate-y-1.5
                peer-[:not(:placeholder-shown)]:text-neutral-400">"Phone"</label>
                </div>
                // End Input

                // Textarea
                <div class="relative">
                  <textarea id="hs-tac-message" class="peer p-4 block w-full bg-neutral-800 border-transparent rounded-lg text-sm text-white placeholder:text-transparent focus:outline-none focus:ring-0 focus:border-transparent disabled:opacity-50 disabled:pointer-events-none
              focus:pt-6
              focus:pb-2
              [&:not(:placeholder-shown)]:pt-6
              [&:not(:placeholder-shown)]:pb-2
              autofill:pt-6
              autofill:pb-2" placeholder="This is a textarea placeholder"></textarea>
                  <label for="hs-tac-message" class="absolute top-0 start-0 p-4 h-full text-neutral-400 text-sm truncate pointer-events-none transition ease-in-out duration-100 border border-transparent peer-disabled:opacity-50 peer-disabled:pointer-events-none
                peer-focus:text-xs
                peer-focus:-translate-y-1.5
                peer-focus:text-neutral-400
                peer-[:not(:placeholder-shown)]:text-xs
                peer-[:not(:placeholder-shown)]:-translate-y-1.5
                peer-[:not(:placeholder-shown)]:text-neutral-400">"Tell us about your project"</label>
                </div>
                // End textarea
              </div>

              <div class="mt-2">
                <p class="text-xs text-neutral-500">
                  "All fields are required"
                </p>

                <p class="mt-5">
                  <a class="group inline-flex items-center gap-x-2 py-2 px-3 bg-[#ff0] font-medium text-sm text-neutral-800 rounded-full focus:outline-none" href="#">
                    "Submit"
                    <svg class="flex-shrink-0 size-4 transition group-hover:translate-x-0.5 group-hover:translate-x-0 group-focus:translate-x-0.5 group-focus:translate-x-0" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>
                  </a>
                </p>
              </div>
            </form>
          </div>
          // End col

          <div class="space-y-14">
            // Item
            <div class="flex gap-x-5">
              <svg class="flex-shrink-0 size-6 text-neutral-500" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z"/><circle cx="12" cy="10" r="3"/></svg>
              <div class="grow">
                <h4 class="text-white font-semibold">"Our address:"</h4>

                <address class="mt-1 text-neutral-400 text-sm not-italic">
                  "300 Bath Street, Tay House"<br/>
                  "Glasgow G2 4JR, United Kingdom"
                </address>
              </div>
            </div>
            // End Item

            // Item
            <div class="flex gap-x-5">
              <svg class="flex-shrink-0 size-6 text-neutral-500" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21.2 8.4c.5.38.8.97.8 1.6v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V10a2 2 0 0 1 .8-1.6l8-6a2 2 0 0 1 2.4 0l8 6Z"/><path d="m22 10-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 10"/></svg>
              <div class="grow">
                <h4 class="text-white font-semibold">"Email us:"</h4>

                <a class="mt-1 text-neutral-400 text-sm" href="#mailto:example@site.co" target="_blank">
                  "hello@example.so"
                </a>
              </div>
            </div>
            // End Item

            // Item
            <div class="flex gap-x-5">
              <svg class="flex-shrink-0 size-6 text-neutral-500" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="m3 11 18-5v12L3 14v-3z"/><path d="M11.6 16.8a3 3 0 1 1-5.8-1.6"/></svg>
              <div class="grow">
                <h4 class="text-white font-semibold">"We're hiring"</h4>
                <p class="mt-1 text-neutral-400">"We're thrilled to announce that we're expanding our team and looking for talented individuals like you to join us."</p>
                <p class="mt-2">
                  <a class="group inline-flex items-center gap-x-2 font-medium text-sm text-[#ff0] decoration-2 hover:underline focus:outline-none focus:underline" href="#">
                    "Job openings"
                    <svg class="flex-shrink-0 size-4 transition group-hover:translate-x-0.5 group-hover:translate-x-0 group-focus:translate-x-0.5 group-focus:translate-x-0" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>
                  </a>
                </p>
              </div>
            </div>
            // End item
          </div>
          // End col
        </div>
        // End grid
      </div>
    </div>
    // End contact
      }
}
