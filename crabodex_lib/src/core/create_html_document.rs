use chrono::Local;
use html_minifier::HTMLMinifier;

/// Create an HTML document. This function generates the HTML document using
/// the provided navigation and page body.
///
/// # Arguments
/// * `navigation` - The navigation section of the HTML document.
/// * `page_body` - The body section of the HTML document.
/// * `repo_name` - The name of the repository.
/// * `repo_description` - The description of the repository.
/// * `commit_hash` - The commit hash of the repository.
/// * `repo_url` - The URL of the repository.
///
/// # Returns
/// A string containing the HTML document.
///
///  # Panics
/// If the provided HTML is invalid.
///
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn create_html_document(
    navigation: &str,
    page_body: &str,
    repo_name: &str,
    repo_description: &str,
    commit_hash: &str,
    repo_url: &str,
) -> String {
    let mut html_minifier: HTMLMinifier = HTMLMinifier::new();
    let generation_date: String = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let body: String = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{repo_name}</title>
    <style>
        *,
        *::before,
        *::after {{
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            font: inherit;
        }}

        html {{
            font-size: 62.5%;
        }}

        :root {{
            --bg-color: #ffffff;
            --text-color: #333333;
            --nav-bg-color: #f4f4f4;
            --pre-bg-color: #f4f4f4;
            --primary: #18181b;
            --muted-foreground: #71717a;
        }}

        .dark-mode {{
            --bg-color: #2d2d2d;
            --text-color: #ffffff;
            --nav-bg-color: #222222;
            --pre-bg-color: #444444;
            --primary: #fafafa;
            --muted-foreground: #d4d4d8;
        }}

        body {{
            line-height: 1.75;
            color: #333;
            height: 100%;
            -webkit-font-smoothing: antialiased;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol";
            font-size: 1.6rem;
            color: var(--text-color);
            background-color: var(--bg-color);
        }}

        a {{
            color: var(--muted-foreground);
        }}

        a:hover {{
            color: var(--primary);
        }}

        .container {{
            display: flex;
            height: 100%;
        }}

        button {{
            cursor: pointer;
        }}

        nav {{
            width: 250px;
            height: 100vh;
            position: fixed;
            left: 0;
            top: 0;
            transition: transform 0.3s ease-in-out;
            border-right: 1px solid #ddd;
            z-index: 1000;
            background-color: var(--bg-color);
            display: flex;
            flex-direction: column;
        }}

        nav>header {{
            border-bottom: 1px solid #ddd;
            flex-grow: 0;
            flex-shrink: 0;
        }}

        nav>header>.repo-name {{
            font-family: Bahnschrift, 'DIN Alternate', 'Franklin Gothic Medium', 'Nimbus Sans Narrow', sans-serif-condensed, sans-serif;
            font-size: 2rem;
            padding: 10px 15px 0 15px;
        }}

        nav>header>.repo-description {{
            padding: 0 15px 10px 15px;
            font-size: 1.4rem;
            border-bottom: 1px solid #ddd;
            color: #999;
        }}

        nav>header>.commit-hash {{
            padding: 10px 15px 0 15px;
            font-size: 1.4rem;
        }}

        nav>header>.commit-hash>a {{
            text-decoration: underline;
        }}

        nav>header>.generation-date {{
            padding: 0 15px 10px 15px;
            font-size: 1.4rem;
        }}

        nav>header>.generation-date>span:first-of-type {{
            font-size: 1.4rem;
            color: #999;
        }}

        nav>header>.generation-date>span:last-of-type {{
            font-size: 1.4rem;
            font-weight: bold;
        }}

        nav>ul {{
            overflow-y: auto;
            font-size: 1.4rem;
            padding: 10px 15px 10px 20px;
            flex-grow: 1;
        }}

        nav a {{
            display: block;
            text-decoration: none;
            color: var(--text-color);
        }}

        nav a:hover {{
            text-decoration: underline;
        }}

        nav ul {{
            list-style-type: "→";
            margin-bottom: 10px;
        }}

        nav ul li {{
            padding-left: 5px;
        }}

        nav ul ul {{
            margin-left: 10px;
            padding: 0;
        }}

        main {{
            flex-grow: 1;
            margin-left: 250px;
            overflow-y: auto;
            position: relative;
            overflow-x: hidden;
            background-color: var(--bg-color);
        }}

        main>header {{
            border-bottom: 1px solid #ddd;
            height: 40px;
            display: flex;
            align-items: center;
            justify-content: flex-end;
            margin-bottom: 20px;
        }}

        main>footer {{
            margin-top: 30px;
            border-top: 1px solid #ddd;
        }}

        main>footer>p {{
            max-width: 800px;
            margin: 0 auto;
            padding: 10px 15px 100px 15px;
        }}

        main>div {{
            padding: 10px;
            width: 100%;
            max-width: 800px;
            margin: 0 auto;
        }}

        main p {{
            margin-bottom: 20px;
        }}

        .depth-2 {{
            padding-left: 20px;
        }}

        .depth-3 {{
            padding-left: 30px;
        }}

        .depth-4 {{
            padding-left: 40px;
        }}

        .depth-5 {{
            padding-left: 50px;
        }}

        .depth-6 {{
            padding-left: 60px;
        }}

        .depth-7 {{
            padding-left: 70px;
        }}

        .depth-8 {{
            padding-left: 80px;
        }}

        .depth-9 {{
            padding-left: 90px;
        }}

        .depth-10 {{
            padding-left: 100px;
        }}

        h1,
        h2,
        h3,
        h4,
        h5,
        h6 {{
            padding: 10px;
            font-family: Bahnschrift, 'DIN Alternate', 'Franklin Gothic Medium', 'Nimbus Sans Narrow', sans-serif-condensed, sans-serif;
            width: 100%;
            max-width: 800px;
            margin: 0 auto;
        }}

        h1>span,
        h2>span,
        h3>span,
        h4>span,
        h5>span,
        h6>span {{
            display: flex;
            align-items: center;
            justify-content: space-between;
        }}

        h1:after,
        h2:after,
        h3:after,
        h4:after,
        h5:after,
        h6:after {{
            display: block;
            width: 100%;
            height: 1px;
            background-color: #ddd;
            content: '';
        }}

        h1 {{
            font-size: 3.2rem;
        }}

        h2 {{
            font-size: 2.8rem;
            padding-left: 20px;
        }}

        h3 {{
            font-size: 2.4rem;
            padding-left: 30px;
        }}

        h4 {{
            font-size: 2.2rem;
            padding-left: 40px;
        }}

        h5 {{
            font-size: 2rem;
            padding-left: 50px;
        }}

        h6 {{
            font-size: 1.8rem;
            padding-left: 60px;
        }}

        pre {{
            background-color: #f4f4f4;
            padding: 10px;
            overflow-x: auto;
        }}

        #menu-toggle {{
            position: absolute;
            top: 0;
            right: -51px;
            z-index: 1000;
            color: #333;
            border: none;
            cursor: pointer;
            border-bottom-right-radius: 5px;
            border-right: 1px solid #ddd;
            border-bottom: 1px solid #ddd;
            background-color: #fff;
            width: 50px;
            text-align: center;
            font-size: 24px;
            height: 50px;
            display: none;
        }}

        #print-btn>svg,
        #mode-toggle>svg {{
            display: block;
            width: 18px;
            fill: var(--text-color);
        }}

        #decrease-font,
        #increase-font,
        #top-repo-link,
        #print-btn,
        #mode-toggle {{
            color: var(--text-color);
            cursor: pointer;
            background: none;
            border: none;
            font-weight: bold;
            height: 100%;
            border-left: 1px solid #ddd;
            padding: 0 8px;
            font-size: 16px;
            display: flex;
            align-items: center;
        }}

        @media (max-width:768px) {{
            nav {{
                transform: translateX(-100%);
            }}

            nav.active {{
                transform: translateX(0);
            }}

            main {{
                margin-left: 0;
            }}

            #menu-toggle {{
                display: block;
            }}
        }}

        .gh-icon {{
            mask-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath d='M12,2A10,10 0 0,0 2,12C2,16.42 4.87,20.17 8.84,21.5C9.34,21.58 9.5,21.27 9.5,21C9.5,20.77 9.5,20.14 9.5,19.31C6.73,19.91 6.14,17.97 6.14,17.97C5.68,16.81 5.03,16.5 5.03,16.5C4.12,15.88 5.1,15.9 5.1,15.9C6.1,15.97 6.63,16.93 6.63,16.93C7.5,18.45 8.97,18 9.54,17.76C9.63,17.11 9.89,16.67 10.17,16.42C7.95,16.17 5.62,15.31 5.62,11.5C5.62,10.39 6,9.5 6.65,8.79C6.55,8.54 6.2,7.5 6.75,6.15C6.75,6.15 7.59,5.88 9.5,7.17C10.29,6.95 11.15,6.84 12,6.84C12.85,6.84 13.71,6.95 14.5,7.17C16.41,5.88 17.25,6.15 17.25,6.15C17.8,7.5 17.45,8.54 17.35,8.79C18,9.5 18.38,10.39 18.38,11.5C18.38,15.32 16.04,16.16 13.81,16.41C14.17,16.72 14.5,17.33 14.5,18.26C14.5,19.6 14.5,20.68 14.5,21C14.5,21.27 14.66,21.59 15.17,21.5C19.14,20.16 22,16.42 22,12A10,10 0 0,0 12,2Z'/%3E%3C/svg%3E");
            background-color: var(--text-color);
            background-size: contain;
            width: 20px;
            height: 20px;
            display: block;
            flex-shrink: 0;
            background-repeat: no-repeat;
            background-position: center;
        }}

        code,
        pre {{
            font-family: ui-monospace, 'Cascadia Code', 'Source Code Pro', Menlo, Consolas, 'DejaVu Sans Mono', monospace;
            background-color: #f4f4f4;
            border-radius: 4px;
            color: #333;
            border: 1px solid #ddd;
        }}

        pre code {{
            background: none;
            border: none;
        }}

        code {{
            padding: 2px 4px;
            font-size: 90%;
            text-wrap: wrap;
        }}

        pre {{
            padding: 10px;
            overflow-x: auto;
        }}

        pre code {{
            background-color: transparent;
            padding: 0;
        }}

        blockquote {{
            border-left: 3px solid #999;
            padding: 5px 5px 5px 15px;
            background: #66666611;
        }}

        table {{
            border-collapse: collapse;
            width: 100%;
            margin-bottom: 10px;
        }}

        th,
        td {{
            border: 1px solid #ddd;
            padding: 8px;
            text-align: left;
        }}

        th {{
            background-color: #00000022;
            font-weight: bold;
        }}

        tr:nth-child(even) {{
            background-color: #00000011;
        }}

        ul,
        ol {{
            padding-left: 20px;
            margin-bottom: 20px;
        }}

        ul {{
            list-style-type: disc;
        }}

        ol {{
            list-style-type: decimal;
        }}

        li {{
            margin-bottom: 5px;
        }}

        hr {{
            border: none;
            border-top: 1px solid #ddd;
            margin: 20px 0;
        }}

        strong {{
            font-weight: 700;
        }}

        em {{
            font-style: italic;
        }}

        @media print {{
            main>header {{
                display: none;
            }}

            body {{
                font-size: 12pt;
            }}

            #menu-toggle{{
               display: none;
            }}

            nav {{
                width: 100%;
                height: auto;
                position: relative;
                transform: translateX(0);
                border-right: none;
                display: block;
            }}
    </style>
</head>
<body>
    <nav>
        <button id="menu-toggle">☰</button>
        <header>
            <p class="repo-name">{repo_name}</p>
            <p class="repo-description">{repo_description}</p>
            <p class="commit-hash">
                <a href="{repo_url}/commit/{commit_hash}" target="_blank">commit {commit_hash}</a>
            </p>
            <p class="generation-date">
                <span>generated at : </span>
                <span>{generation_date}</span>
            </p>
        </header>
        {navigation}
    </nav>
    <main>
        <header>
            <button id="print-btn" type="button" title="Print documentation">
            	<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                  <path d="M18 3H6v4h12m1 5a1 1 0 0 1-1-1 1 1 0 0 1 1-1 1 1 0 0 1 1 1 1 1 0 0 1-1 1m-3 7H8v-5h8m3-6H5a3 3 0 0 0-3 3v6h4v4h12v-4h4v-6a3 3 0 0 0-3-3Z"/>
                </svg>
            </button>
            <button id="decrease-font" type="button" title="Decrease font size">A-</button>
            <button id="increase-font" type="button" title="Increase font size">A+</button>
            <div id="top-repo-link" title="Go to repository">
                <a href="{repo_url}" class="gh-icon"></a>
            </div>
            <button id="mode-toggle" title="Toggle dark & light mode">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path
                        d="M7.5 2c-1.79 1.15-3 3.18-3 5.5s1.21 4.35 3.03 5.5C4.46 13 2 10.54 2 7.5A5.5 5.5 0 0 1 7.5 2m11.57 1.5 1.43 1.43L4.93 20.5 3.5 19.07 19.07 3.5m-6.18 2.43L11.41 5 9.97 6l.42-1.7L9 3.24l1.75-.12.58-1.65L12 3.1l1.73.03-1.35 1.13.51 1.67m-3.3 3.61-1.16-.73-1.12.78.34-1.32-1.09-.83 1.36-.09.45-1.29.51 1.27 1.36.03-1.05.87.4 1.31M19 13.5a5.5 5.5 0 0 1-5.5 5.5c-1.22 0-2.35-.4-3.26-1.07l7.69-7.69c.67.91 1.07 2.04 1.07 3.26m-4.4 6.58 2.77-1.15-.24 3.35-2.53-2.2m4.33-2.7 1.15-2.77 2.2 2.54-3.35.23m1.15-4.96-1.14-2.78 3.34.24-2.2 2.54M9.63 18.93l2.77 1.15-2.53 2.19-.24-3.34Z" />
                </svg>
            </button>
        </header>
        {page_body}
        <footer>
        </footer>
    </main>
    <script>
        document.addEventListener('DOMContentLoaded', () => {{

            const html = document.documentElement;

            const getCurrentFontSize = () => parseFloat(window.getComputedStyle(html).fontSize);
            const setFontSize = (newSize) => html.style.fontSize = newSize + 'px';
            document.getElementById('increase-font').addEventListener('click', () => {{
                const currentSize = getCurrentFontSize();
                setFontSize(Math.min(currentSize + 2, 24));
            }});
            document.getElementById('decrease-font').addEventListener('click', () => {{
                const currentSize = getCurrentFontSize();
                setFontSize(Math.max(currentSize - 2, 8));
            }});

            document.getElementById('menu-toggle').addEventListener('click', () => {{
                document.querySelector('nav').classList.toggle('active');
            }});
            window.addEventListener('resize', () => {{
                document.querySelector('nav').classList.remove('active');
            }});

            const printBtn = document.getElementById('print-btn');
            printBtn.addEventListener('click', () => {{
                window.print();
            }});

            const setTheme = (isDark) => {{
                document.body.classList.toggle('dark-mode', isDark);
                localStorage.setItem('storedTheme', isDark ? 'dark' : 'light')
            }};
            document.getElementById('mode-toggle').addEventListener('click', () => {{
                setTheme(!document.body.classList.contains('dark-mode'))
            }});
            const storedTheme = localStorage.getItem('storedTheme');
            if (storedTheme) {{
                setTheme(storedTheme === 'dark');
            }} else {{
                setTheme(window.matchMedia('(prefers-color-scheme: dark)').matches);
            }}
        }});
    </script>
</body>
</html>"#
    );

    html_minifier.digest(body).unwrap();
    String::from_utf8_lossy(html_minifier.get_html()).into()
}
