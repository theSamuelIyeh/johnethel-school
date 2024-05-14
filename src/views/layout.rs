use html_to_string_macro::html;

pub fn layout(title: String, page: String) -> String {
    let json_string =
        r##"x-on:htmx:after-swap.camel="console.log('i am here'); window.HSStaticMethods.autoInit();""##
            .to_string();
    html! {
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta http-equiv="X-UA-Compatible" content="IE=edge" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
        <link href="https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;700&display=swap" rel="stylesheet"/>
        <link href="/static/css/tailwind.css" rel="stylesheet" />
        <title>{title}</title>
        <meta name="description" content="Description of the page" />
        <meta name="keywords" content="keywords, for, the, page" />
        <script src="/static/js/htmx.min.js"></script>
        <script defer src="/static/js/alpine.min.js"></script>
        <script defer src="/static/js/preline.js"></script>
    </head>
    <body x-cloak x-data="{theme: '' }" class="dark:bg-neutral-900 " {json_string} {r##":class="theme""##} {r##"x-init="if (!localStorage.getItem('theme')) {
        			if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
            			theme = 'dark';
        			} else {
						theme = 'light';
        			}
                    localStorage.setItem('theme', theme);
    			} else {
                    theme = localStorage.getItem('theme');
                }; htmx.config.useTemplateFragments = true;""##}>
        <main class="dark:bg-neutral-900">
        {page}
        </main>
    </body>
    </html>
    }
    // .render()
}
