use super::{CodeExecutor, CodeLanguage, ExecutionResult};
use leptos::*;

#[cfg(feature = "hydrate")]
use gloo_timers::callback::Timeout;
#[cfg(feature = "hydrate")]
use js_sys;

#[component]
pub fn CodeRunner(
    #[prop(into)] code: String,
    #[prop(into)] language: String,
    #[prop(optional)] show_copy: bool,
    #[prop(optional)] show_run: bool,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    // Clone props early to avoid move issues
    let code_clone = code.clone();
    let language_clone = language.clone();
    let show_copy = if show_copy { show_copy } else { true };
    let show_run = if show_run { show_run } else { true };

    let code_lang = CodeLanguage::from_str(&language);
    let is_executable = code_lang
        .as_ref()
        .map(|l| l.is_web_executable())
        .unwrap_or(false);

    let execution_result = create_rw_signal::<Option<ExecutionResult>>(None);
    let is_executing = create_rw_signal(false);
    let show_output = create_rw_signal(false);
    let copy_success = create_rw_signal(false);

    let code_id = format!("code-runner-{}", rand::random::<u32>());

    // Use create_effect to run highlighting after hydration
    #[cfg(feature = "hydrate")]
    {
        let highlight_id = code_id.clone();
        create_effect(move |_| {
            // Use a longer delay to ensure everything is ready
            let id_clone = highlight_id.clone();
            Timeout::new(500, move || {
                let _ = js_sys::eval(&format!(
                    "console.log('Effect highlighting: {}'); \
                     if (window.updateSyntaxTheme) {{ \
                         window.updateSyntaxTheme(); \
                     }} \
                     if (window.Prism && window.Prism.highlightElement) {{ \
                         var element = document.querySelector('#{} code'); \
                         if (element) {{ \
                             console.log('Effect found element, highlighting:', element); \
                             window.Prism.highlightElement(element); \
                         }} else {{ \
                             console.log('Effect - element not found: #{}'); \
                         }} \
                     }} else {{ \
                         console.log('Effect - Prism not available'); \
                     }}",
                    id_clone, id_clone, id_clone
                ));
            })
            .forget();
        });
    }

    let copy_code = {
        let _code_copy = code_clone.clone();
        move |_| {
            #[cfg(feature = "hydrate")]
            {
                copy_success.set(true);

                // Reset copy success state after 2 seconds
                Timeout::new(2000, move || {
                    copy_success.set(false);
                })
                .forget();

                // Fallback copy method that works in all browsers
                let _ = js_sys::eval(&format!(
                    r#"
                    try {{
                        if (navigator.clipboard && window.isSecureContext) {{
                            navigator.clipboard.writeText(`{}`).then(() => {{
                                console.log('Code copied via clipboard API');
                            }}).catch(() => {{
                                // Fallback
                                const textArea = document.createElement('textarea');
                                textArea.value = `{}`;
                                document.body.appendChild(textArea);
                                textArea.select();
                                document.execCommand('copy');
                                document.body.removeChild(textArea);
                                console.log('Code copied via execCommand fallback');
                            }});
                        }} else {{
                            // Fallback for older browsers
                            const textArea = document.createElement('textarea');
                            textArea.value = `{}`;
                            document.body.appendChild(textArea);
                            textArea.select();
                            document.execCommand('copy');
                            document.body.removeChild(textArea);
                            console.log('Code copied via execCommand');
                        }}
                    }} catch (e) {{
                        console.error('Failed to copy code:', e);
                    }}
                "#,
                    _code_copy.replace('`', "\\`").replace('\\', "\\\\"),
                    _code_copy.replace('`', "\\`").replace('\\', "\\\\"),
                    _code_copy.replace('`', "\\`").replace('\\', "\\\\")
                ));
            }
        }
    };

    let run_code = {
        let code_run = code_clone.clone();
        move |_| {
            if let Some(lang) = &code_lang {
                if lang.is_web_executable() {
                    is_executing.set(true);
                    show_output.set(true);
                    execution_result.set(None);

                    let executor = CodeExecutor::default();
                    let code_execute = code_run.clone();
                    let lang_clone = lang.clone();

                    spawn_local(async move {
                        let result = executor.execute(lang_clone, &code_execute).await;
                        execution_result.set(Some(result));
                        is_executing.set(false);
                    });
                }
            }
        }
    };

    view! {
        <div class=format!("code-runner-container relative group my-6 sm:my-7 md:my-8 {}", class.unwrap_or_default())>
            // Code block
            <pre
                class="bg-muted border-2 border-border p-3 sm:p-4 md:p-6 overflow-x-auto relative font-mono text-sm sm:text-base leading-relaxed whitespace-pre-wrap"
                id=code_id.clone()
            >
                // Action buttons only - positioned at top right, visible on hover
                <div class="absolute top-2 right-2 z-10 flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                    {if show_copy {
                        view! {
                            <button
                                class="code-btn"
                                title=move || if copy_success.get() { "Copied!" } else { "Copy code" }
                                on:click=copy_code
                            >
                                {move || if copy_success.get() {
                                    view! {
                                        <svg class="w-3 h-3 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                        </svg>
                                    }.into_view()
                                } else {
                                    view! {
                                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                                        </svg>
                                    }.into_view()
                                }}
                            </button>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }}

                    {if show_run && is_executable {
                        view! {
                            <button
                                class="code-btn"
                                title=move || {
                                    if is_executing.get() {
                                        "Executing..."
                                    } else if execution_result.get().is_some() {
                                        "Run again"
                                    } else {
                                        "Run code"
                                    }
                                }
                                on:click=run_code
                                disabled=move || is_executing.get()
                            >
                                {move || {
                                    if is_executing.get() {
                                        // Pause/loading icon during execution
                                        view! {
                                            <svg class="w-3 h-3 text-yellow-500 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                                            </svg>
                                        }.into_view()
                                    } else if let Some(_result) = execution_result.get() {
                                        // Replay icon after execution (regardless of success/failure)
                                        view! {
                                            <svg class="w-3 h-3 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                                            </svg>
                                        }.into_view()
                                    } else {
                                        // Initial play icon
                                        view! {
                                            <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 24 24">
                                                <path d="M8 5v14l11-7z"/>
                                            </svg>
                                        }.into_view()
                                    }
                                }}
                            </button>
                        }.into_view()
                    } else if show_run {
                        view! {
                            <button
                                class="code-btn opacity-50 cursor-not-allowed"
                                title="Execution not supported in browser"
                                disabled=true
                            >
                                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728L5.636 5.636m12.728 12.728L18.364 5.636M5.636 18.364l12.728-12.728"></path>
                                </svg>
                            </button>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }}
                </div>

                // Code content
                <code class=format!("language-{}", language_clone)>{code}</code>

                // Console output - minimal and clean
                {move || if show_output.get() {
                    view! {
                        <div class="border-t-2 border-border mt-4 pt-3">
                            <div class="font-mono text-xs sm:text-sm max-h-64 overflow-y-auto console-output px-3 py-2">
                                {move || match (is_executing.get(), execution_result.get()) {
                                    (true, _) => view! {
                                        <div class="text-muted-foreground">
                                            <span class="animate-pulse">"Executing..."</span>
                                        </div>
                                    }.into_view(),
                                    (false, Some(result)) => {
                                        view! {
                                            <div>
                                                {if !result.output.is_empty() {
                                                    let output_clone = result.output.clone();
                                                    let lines: Vec<String> = output_clone.split('\n').map(|s| s.to_string()).collect();
                                                    lines.into_iter().map(|line| {
                                                        if line.starts_with("ERROR:") {
                                                            let error_text = line.trim_start_matches("ERROR:").trim().to_string();
                                                            view! {
                                                                <div class="text-red-400 leading-relaxed">
                                                                    {error_text}
                                                                </div>
                                                            }.into_view()
                                                        } else if line.starts_with("→") {
                                                            let result_text = line.trim_start_matches("→").trim().to_string();
                                                            view! {
                                                                <div class="text-muted-foreground leading-relaxed opacity-75">
                                                                    <span class="mr-1">">"</span>
                                                                    {result_text}
                                                                </div>
                                                            }.into_view()
                                                        } else if !line.trim().is_empty() {
                                                            let line_clone = line.clone();
                                                            view! {
                                                                <div class="text-foreground leading-relaxed">
                                                                    {line_clone}
                                                                </div>
                                                            }.into_view()
                                                        } else {
                                                            view! { <div></div> }.into_view()
                                                        }
                                                    }).collect_view()
                                                } else {
                                                    view! { <div></div> }.into_view()
                                                }}

                                                {if let Some(error) = &result.error {
                                                    view! {
                                                        <div class="text-red-400 leading-relaxed mt-1">
                                                            {error.clone()}
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    view! { <div></div> }.into_view()
                                                }}

                                                {if result.success && result.error.is_none() && result.output.is_empty() {
                                                    view! {
                                                        <div class="text-muted-foreground opacity-50 text-xs mt-1">
                                                            "No output"
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    view! { <div></div> }.into_view()
                                                }}
                                            </div>
                                        }.into_view()
                                    },
                                    (false, None) => view! {
                                        <div class="text-muted-foreground opacity-50 text-xs">
                                            "Ready"
                                        </div>
                                    }.into_view(),
                                }}
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }}
            </pre>
        </div>
    }
}
