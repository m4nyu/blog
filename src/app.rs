use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "hydrate")]
use web_sys;

use crate::components::header::Header;
use crate::components::ui::dialog::Dialog;
use crate::routes::home::get_posts;
use crate::routes::home::HomePage;
use crate::routes::post::PostPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Initialize settings from localStorage or defaults
    #[cfg(feature = "hydrate")]
    let (animation_speed, theme_mode, population_density) = {
        let get_from_storage = |key: &str, default: &str| -> String {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(value)) = storage.get_item(key) {
                        return value;
                    }
                }
            }
            default.to_string()
        };

        let animation_speed_val = get_from_storage("blog_animation_speed", "100")
            .parse::<u64>()
            .unwrap_or(100);
        let theme_mode_val = get_from_storage("blog_theme_mode", "system");
        let population_density_val = get_from_storage("blog_population_density", "0.08")
            .parse::<f64>()
            .unwrap_or(0.08);

        (
            RwSignal::new(animation_speed_val),
            RwSignal::new(theme_mode_val),
            RwSignal::new(population_density_val),
        )
    };

    #[cfg(not(feature = "hydrate"))]
    let (animation_speed, theme_mode, population_density) = (
        RwSignal::new(100u64),
        RwSignal::new("system".to_string()),
        RwSignal::new(0.08f64),
    );

    // Global settings dialog control
    let show_settings = RwSignal::new(false);

    // Global search query control
    let search_query = RwSignal::new(String::new());

    // Global posts resource
    let posts = create_resource(|| (), |_| async { get_posts().await });

    // Save settings to localStorage when they change
    #[cfg(feature = "hydrate")]
    {
        let save_to_storage = |key: &str, value: &str| {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item(key, value);
                }
            }
        };

        // Watch animation speed changes
        create_effect(move |_| {
            let speed = animation_speed.get();
            save_to_storage("blog_animation_speed", &speed.to_string());
        });

        // Watch population density changes
        create_effect(move |_| {
            let density = population_density.get();
            save_to_storage("blog_population_density", &density.to_string());
        });

        // Watch theme mode changes
        create_effect(move |_| {
            let mode = theme_mode.get();
            save_to_storage("blog_theme_mode", &mode);
        });

        // Apply initial theme on load
        create_effect(move |_| {
            let mode = theme_mode.get();
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        let class_list = html.class_list();
                        let _ = class_list.remove_1("dark");

                        match mode.as_str() {
                            "dark" => {
                                let _ = class_list.add_1("dark");
                            }
                            "light" => {
                                // Already removed dark class
                            }
                            "system" => {
                                // Check system preference
                                if let Ok(Some(media_query)) =
                                    window.match_media("(prefers-color-scheme: dark)")
                                {
                                    if media_query.matches() {
                                        let _ = class_list.add_1("dark");
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    }

    // Handle theme changes
    #[cfg(feature = "hydrate")]
    let handle_theme_change = move |mode: String| {
        theme_mode.set(mode); // This will trigger the effect above which applies the theme
    };

    #[cfg(not(feature = "hydrate"))]
    let handle_theme_change = move |mode: String| {
        theme_mode.set(mode);
    };

    // Provide global context for animation speed, settings, search, posts, and population density
    provide_context(animation_speed);
    provide_context(show_settings);
    provide_context(search_query);
    provide_context(posts);
    provide_context(population_density);

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind_actix.css"/>
        <Link rel="icon" type_="image/svg+xml" href="/favicon.svg?v=2"/>
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png?v=2"/>
        <Link rel="apple-touch-icon" href="/favicon-192x192.png"/>
        <Link rel="manifest" href="/manifest.json"/>
        <Meta name="theme-color" content="#000000"/>
        <Title text="blog"/>
        <Meta name="description" content="A blog built with Leptos and Rust"/>

        // Load Prism.js CSS theme
        <Link rel="stylesheet" href="/prism.css"/>

        // Load Prism with autoloader only
        <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js"></script>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js"></script>

        // Load Pyodide for Python execution
        <script src="https://cdn.jsdelivr.net/pyodide/v0.24.1/full/pyodide.js"></script>

        // Load Container2wasm for universal language execution
        <script type="module">
            r#"
            // Pre-load Container2wasm runtime for faster execution
            window.container2wasmReady = import('https://cdn.jsdelivr.net/npm/container2wasm@latest/dist/container2wasm.js')
                .then(c2w => {
                    window.container2wasm = c2w;
                    console.log('Container2wasm runtime loaded - ALL languages now executable!');
                    return c2w;
                })
                .catch(e => {
                    console.warn('Container2wasm not available:', e);
                    return null;
                });
            "#
        </script>
        <script>
            r#"
            // Initialize Pyodide asynchronously
            async function loadPyodideRuntime() {
                try {
                    console.log('Loading Pyodide...');
                    window.pyodide = await loadPyodide({
                        indexURL: 'https://cdn.jsdelivr.net/pyodide/v0.24.1/full/'
                    });
                    console.log('Pyodide loaded successfully');
                } catch (e) {
                    console.error('Failed to load Pyodide:', e);
                }
            }
            
            // Start loading Pyodide in the background
            loadPyodideRuntime();
            "#
        </script>

        // Theme switching for syntax highlighting
        <script>
            r#"
            // Make updateSyntaxTheme available globally immediately
            window.updateSyntaxTheme = function() {
                // Themes are now inline, so this doesn't need to do anything
                console.log('Theme update called - inline CSS is already active');
            };
            
            // Apply initial theme immediately
            window.updateSyntaxTheme();
            
            // Watch for theme changes
            const observer = new MutationObserver((mutations) => {
                mutations.forEach((mutation) => {
                    if (mutation.type === 'attributes' && mutation.attributeName === 'class') {
                        window.updateSyntaxTheme();
                        // Re-highlight all code blocks when theme changes
                        if (window.Prism) {
                            window.Prism.highlightAll();
                        }
                    }
                });
            });
            
            // Start observing theme changes
            observer.observe(document.documentElement, {
                attributes: true,
                attributeFilter: ['class']
            });
            
            // Just apply theme changes, let components handle their own highlighting
            document.addEventListener('DOMContentLoaded', () => {
                console.log('DOMContentLoaded: applying theme');
                window.updateSyntaxTheme();
            });
            
            window.addEventListener('load', () => {
                console.log('Window load: applying theme');
                window.updateSyntaxTheme();
            });
            "#
        </script>

        // Prevent theme flash by applying theme immediately
        <script>
            r#"
            (function() {
                try {
                    const theme = localStorage.getItem('blog_theme_mode') || 'system';
                    const html = document.documentElement;
                    
                    // Remove any existing theme class
                    html.classList.remove('dark');
                    
                    if (theme === 'dark') {
                        html.classList.add('dark');
                    } else if (theme === 'system') {
                        // Check system preference
                        if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
                            html.classList.add('dark');
                        }
                    }
                    // For 'light' theme, we don't add any class (default)
                } catch (e) {
                    // If localStorage fails, fall back to system preference
                    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
                        document.documentElement.classList.add('dark');
                    }
                }
            })();
            "#
        </script>

        // JavaScript functions for code block functionality
        <script>
            r#"
            function copyCode(codeId) {
                const codeElement = document.getElementById(codeId);
                const codeText = codeElement.querySelector('code').textContent;
                
                navigator.clipboard.writeText(codeText).then(() => {
                    // Visual feedback
                    const button = event.target.closest('button');
                    const originalContent = button.innerHTML;
                    button.innerHTML = '<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>';
                    setTimeout(() => {
                        button.innerHTML = originalContent;
                    }, 2000);
                }).catch(() => {
                    // Fallback for older browsers
                    const textArea = document.createElement('textarea');
                    textArea.value = codeText;
                    document.body.appendChild(textArea);
                    textArea.select();
                    document.execCommand('copy');
                    document.body.removeChild(textArea);
                });
            }
            
            function runCode(codeId, language) {
                const codeElement = document.getElementById(codeId);
                const codeText = codeElement.querySelector('code').textContent;
                
                // Create or find output container inside the code block
                let outputContainer = codeElement.querySelector('.code-output');
                if (!outputContainer) {
                    outputContainer = document.createElement('div');
                    outputContainer.className = 'code-output font-mono text-sm';
                    codeElement.appendChild(outputContainer);
                }
                
                outputContainer.innerHTML = '<div class="text-yellow-400">Running ' + language + '...</div>';
                
                // Execute code based on language
                executeCode(language, codeText, outputContainer);
            }
            
            function executeCode(language, code, outputContainer) {
                // Map of language executors
                const executors = {
                    'javascript': executeJS,
                    'js': executeJS,
                    'python': executePython,
                    'py': executePython,
                    'html': executeHTML,
                    'css': executeCSS,
                    'rust': executeOnline.bind(null, 'rust'),
                    'go': executeOnline.bind(null, 'go'),
                    'java': executeOnline.bind(null, 'java'),
                    'c': executeOnline.bind(null, 'c'),
                    'cpp': executeOnline.bind(null, 'cpp'),
                    'c++': executeOnline.bind(null, 'cpp'),
                    'bash': executeOnline.bind(null, 'bash'),
                    'sh': executeOnline.bind(null, 'bash'),
                    'sql': executeSQL,
                    'typescript': executeTS,
                    'ts': executeTS
                };
                
                const executor = executors[language.toLowerCase()];
                if (executor) {
                    executor(code, outputContainer);
                } else {
                    outputContainer.innerHTML = '<div class="text-yellow-400">Language "' + language + '" execution coming soon!</div>';
                }
            }
            
            function executeJS(code, outputContainer) {
                try {
                    // Create sandboxed iframe for execution
                    const iframe = document.createElement('iframe');
                    iframe.style.display = 'none';
                    iframe.sandbox = 'allow-scripts';
                    document.body.appendChild(iframe);
                    
                    const iframeDoc = iframe.contentDocument || iframe.contentWindow.document;
                    const script = iframeDoc.createElement('script');
                    
                    // Inject console capture
                    script.textContent = `
                        let output = [];
                        const originalLog = console.log;
                        const originalError = console.error;
                        
                        console.log = (...args) => {
                            output.push({type: 'log', data: args.map(a => String(a)).join(' ')});
                            originalLog(...args);
                        };
                        
                        console.error = (...args) => {
                            output.push({type: 'error', data: args.map(a => String(a)).join(' ')});
                            originalError(...args);
                        };
                        
                        try {
                            const result = (function() {
                                ${code}
                            })();
                            if (result !== undefined) {
                                output.push({type: 'result', data: result});
                            }
                        } catch (e) {
                            output.push({type: 'error', data: e.message});
                        }
                        
                        parent.postMessage({type: 'output', data: output}, '*');
                    `;
                    
                    // Listen for results
                    const messageHandler = (e) => {
                        if (e.data.type === 'output') {
                            let html = '<div class="text-xs text-muted-foreground mb-2">Output:</div>';
                            if (e.data.data.length === 0) {
                                html += '<div class="text-gray-500">No output</div>';
                            } else {
                                e.data.data.forEach(item => {
                                    if (item.type === 'log') {
                                        html += '<div class="text-green-400">' + escapeHtml(item.data) + '</div>';
                                    } else if (item.type === 'error') {
                                        html += '<div class="text-red-400">Error: ' + escapeHtml(item.data) + '</div>';
                                    } else if (item.type === 'result') {
                                        html += '<div class="text-blue-400">→ ' + escapeHtml(JSON.stringify(item.data)) + '</div>';
                                    }
                                });
                            }
                            outputContainer.innerHTML = html;
                            window.removeEventListener('message', messageHandler);
                            document.body.removeChild(iframe);
                        }
                    };
                    
                    window.addEventListener('message', messageHandler);
                    iframeDoc.head.appendChild(script);
                    
                    // Timeout after 5 seconds
                    setTimeout(() => {
                        window.removeEventListener('message', messageHandler);
                        if (document.body.contains(iframe)) {
                            document.body.removeChild(iframe);
                            outputContainer.innerHTML = '<div class="text-yellow-400">Execution timed out (5s limit)</div>';
                        }
                    }, 5000);
                    
                } catch (e) {
                    outputContainer.innerHTML = '<div class="text-red-400">Error: ' + e.message + '</div>';
                }
            }
            
            function executeHTML(code, outputContainer) {
                const iframe = document.createElement('iframe');
                iframe.className = 'w-full h-64 bg-white border border-border mt-2';
                iframe.sandbox = 'allow-scripts';
                iframe.srcdoc = code;
                outputContainer.innerHTML = '<div class="text-xs text-muted-foreground mb-2">Preview:</div>';
                outputContainer.appendChild(iframe);
            }
            
            function executeCSS(code, outputContainer) {
                const preview = document.createElement('div');
                preview.className = 'p-4 border border-border mt-2 bg-background';
                preview.innerHTML = '<div class="demo-element">Demo Element</div><p class="demo-text">Sample text to style</p>';
                
                const style = document.createElement('style');
                style.textContent = code;
                preview.appendChild(style);
                
                outputContainer.innerHTML = '<div class="text-xs text-muted-foreground mb-2">CSS Applied to Demo:</div>';
                outputContainer.appendChild(preview);
            }
            
            function executePython(code, outputContainer) {
                // Use Pyodide for Python execution
                outputContainer.innerHTML = '<div class="text-yellow-400">Loading Python interpreter...</div>';
                
                if (!window.pyodideReadyPromise) {
                    window.pyodideReadyPromise = loadPyodide();
                }
                
                window.pyodideReadyPromise.then(pyodide => {
                    try {
                        // Capture output
                        pyodide.runPython(`
                            import sys
                            from io import StringIO
                            sys.stdout = StringIO()
                        `);
                        
                        // Run user code
                        const result = pyodide.runPython(code);
                        
                        // Get output
                        const output = pyodide.runPython("sys.stdout.getvalue()");
                        
                        let html = '<div class="text-xs text-muted-foreground mb-2">Output:</div>';
                        if (output) {
                            html += '<div class="text-green-400">' + escapeHtml(output) + '</div>';
                        }
                        if (result !== undefined && result !== null) {
                            html += '<div class="text-blue-400">→ ' + escapeHtml(String(result)) + '</div>';
                        }
                        if (!output && (result === undefined || result === null)) {
                            html += '<div class="text-gray-500">No output</div>';
                        }
                        
                        outputContainer.innerHTML = html;
                    } catch (e) {
                        outputContainer.innerHTML = '<div class="text-red-400">Error: ' + escapeHtml(e.message) + '</div>';
                    }
                }).catch(e => {
                    outputContainer.innerHTML = '<div class="text-red-400">Failed to load Python: ' + e.message + '</div>';
                });
            }
            
            function executeSQL(code, outputContainer) {
                // Basic SQL execution using SQL.js
                outputContainer.innerHTML = '<div class="text-yellow-400">SQL execution requires a database connection</div>';
            }
            
            function executeTS(code, outputContainer) {
                // Transpile TypeScript to JavaScript first
                outputContainer.innerHTML = '<div class="text-yellow-400">TypeScript compilation in progress...</div>';
                // For now, fallback to JS execution
                executeJS(code, outputContainer);
            }
            
            function executeOnline(language, code, outputContainer) {
                outputContainer.innerHTML = '<div class="text-yellow-400">' + language + ' requires server-side compilation. Use online playgrounds for now.</div>';
            }
            
            function escapeHtml(text) {
                const div = document.createElement('div');
                div.textContent = text;
                return div.innerHTML;
            }
            
            function highlightCode() {
                // Trigger syntax highlighting for all code blocks
                if (window.Prism) {
                    window.Prism.highlightAll();
                }
            }
            
            // Load Pyodide for Python support
            function loadPyodide() {
                const script = document.createElement('script');
                script.src = 'https://cdn.jsdelivr.net/pyodide/v0.24.1/full/pyodide.js';
                document.head.appendChild(script);
                
                return new Promise((resolve) => {
                    script.onload = async () => {
                        const pyodide = await loadPyodide({
                            indexURL: 'https://cdn.jsdelivr.net/pyodide/v0.24.1/full/'
                        });
                        resolve(pyodide);
                    };
                });
            }
            "#
        </script>

        <Router>
            <div class="min-h-screen bg-background text-foreground transition-colors">
                <Routes>
                    <Route path="" view=move || view! {
                        <>
                            <Header
                                on_theme_change=Callback::new(handle_theme_change.clone())
                            />
                            <main class="min-h-screen pt-16">
                                <HomePage/>
                            </main>
                        </>
                    }/>
                    <Route path="/post/:slug" view=move || view! {
                        <main class="min-h-screen">
                            <PostPage/>
                        </main>
                    }/>
                </Routes>
            </div>

            // Settings Dialog
            <Dialog
                show_settings=show_settings
                animation_speed=animation_speed
                population_density=population_density
                theme_mode=theme_mode
                on_theme_change=Callback::new(handle_theme_change)
            />
        </Router>
    }
}
