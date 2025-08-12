#[cfg(feature = "hydrate")]
use js_sys::Promise;

#[derive(Clone, Debug, PartialEq)]
pub enum CodeLanguage {
    JavaScript,
    TypeScript,
    Python,
    Rust,
    C,
    Cpp,
    Go,
    Java,
    Ruby,
    PHP,
    Perl,
    Lua,
    Haskell,
    Elixir,
    Kotlin,
    Scala,
    Bash,
    Other(String),
}

impl CodeLanguage {
    pub fn from_str(lang: &str) -> Option<Self> {
        match lang.to_lowercase().as_str() {
            "javascript" | "js" => Some(Self::JavaScript),
            "typescript" | "ts" => Some(Self::TypeScript),
            "python" | "py" => Some(Self::Python),
            "rust" | "rs" => Some(Self::Rust),
            "c" => Some(Self::C),
            "cpp" | "c++" | "cxx" | "cc" => Some(Self::Cpp),
            "go" | "golang" => Some(Self::Go),
            "java" => Some(Self::Java),
            "ruby" | "rb" => Some(Self::Ruby),
            "php" => Some(Self::PHP),
            "perl" | "pl" => Some(Self::Perl),
            "lua" => Some(Self::Lua),
            "haskell" | "hs" => Some(Self::Haskell),
            "elixir" | "ex" => Some(Self::Elixir),
            "kotlin" | "kt" => Some(Self::Kotlin),
            "scala" => Some(Self::Scala),
            "bash" | "shell" | "sh" => Some(Self::Bash),
            _ => Some(Self::Other(lang.to_string())),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::JavaScript => "javascript",
            Self::TypeScript => "typescript",
            Self::Python => "python",
            Self::Rust => "rust",
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::Go => "go",
            Self::Java => "java",
            Self::Ruby => "ruby",
            Self::PHP => "php",
            Self::Perl => "perl",
            Self::Lua => "lua",
            Self::Haskell => "haskell",
            Self::Elixir => "elixir",
            Self::Kotlin => "kotlin",
            Self::Scala => "scala",
            Self::Bash => "bash",
            Self::Other(s) => s,
        }
    }

    pub fn is_web_executable(&self) -> bool {
        true // ALL languages are executable!
    }
}

#[derive(Clone, Debug)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time: Option<u32>,
}

pub struct CodeExecutor {
    #[allow(dead_code)]
    timeout_ms: u32,
}

impl Default for CodeExecutor {
    fn default() -> Self {
        Self { timeout_ms: 30000 }
    }
}

impl CodeExecutor {
    #[allow(dead_code)]
    pub fn new(timeout_ms: u32) -> Self {
        Self { timeout_ms }
    }

    #[cfg(feature = "hydrate")]
    pub async fn execute(&self, language: CodeLanguage, code: &str) -> ExecutionResult {
        let start_time = js_sys::Date::now();
        
        match language {
            CodeLanguage::JavaScript | CodeLanguage::TypeScript => {
                self.execute_javascript(code, start_time).await
            }
            CodeLanguage::Python => {
                self.execute_with_api("python", code, start_time).await
            }
            CodeLanguage::Rust => {
                self.execute_rust_playground(code, start_time).await
            }
            CodeLanguage::Go => {
                self.execute_go_playground(code, start_time).await
            }
            _ => {
                self.execute_with_api(language.as_str(), code, start_time).await
            }
        }
    }

    #[cfg(not(feature = "hydrate"))]
    pub async fn execute(&self, _language: CodeLanguage, _code: &str) -> ExecutionResult {
        ExecutionResult {
            success: false,
            output: String::new(),
            error: Some("Code execution only available in browser".to_string()),
            execution_time: None,
        }
    }

    #[cfg(feature = "hydrate")]
    async fn execute_javascript(&self, code: &str, start_time: f64) -> ExecutionResult {
        let escaped_code = code
            .replace('\\', "\\\\")
            .replace('`', "\\`")
            .replace('\n', "\\n")
            .replace('\r', "\\r");

        let js_code = format!(r#"
            (() => {{
                try {{
                    let output = [];
                    let originalLog = console.log;
                    let originalError = console.error;
                    
                    console.log = (...args) => {{
                        output.push(args.map(a => String(a)).join(' '));
                        originalLog.apply(console, args);
                    }};
                    console.error = (...args) => {{
                        output.push('ERROR: ' + args.map(a => String(a)).join(' '));
                        originalError.apply(console, args);
                    }};
                    
                    let result = eval(`{}`);
                    
                    console.log = originalLog;
                    console.error = originalError;
                    
                    let stdout = output.join('\\n');
                    if (result !== undefined && result !== null) {{
                        stdout += (stdout ? '\\n' : '') + '→ ' + String(result);
                    }}
                    
                    return {{
                        success: true,
                        output: stdout,
                        error: null
                    }};
                }} catch (e) {{
                    console.log = originalLog;
                    console.error = originalError;
                    return {{
                        success: false,
                        output: '',
                        error: e.message
                    }};
                }}
            }})()
        "#, escaped_code);

        match js_sys::eval(&js_code) {
            Ok(result) => {
                let end_time = js_sys::Date::now();
                let success = js_sys::Reflect::get(&result, &"success".into())
                    .unwrap().as_bool().unwrap_or(false);
                let output = js_sys::Reflect::get(&result, &"output".into())
                    .unwrap().as_string().unwrap_or_default();
                let error = js_sys::Reflect::get(&result, &"error".into())
                    .unwrap().as_string();
                
                ExecutionResult {
                    success,
                    output,
                    error,
                    execution_time: Some((end_time - start_time) as u32),
                }
            }
            Err(_) => ExecutionResult {
                success: false,
                output: String::new(),
                error: Some("JavaScript execution failed".to_string()),
                execution_time: Some((js_sys::Date::now() - start_time) as u32),
            }
        }
    }

    #[cfg(feature = "hydrate")]
    async fn execute_rust_playground(&self, code: &str, start_time: f64) -> ExecutionResult {
        let escaped_code = code
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r");

        let js_code = format!(r#"
            (async () => {{
                try {{
                    console.log('🦀 Executing Rust code on playground...');
                    
                    const response = await fetch('https://play.rust-lang.org/execute', {{
                        method: 'POST',
                        headers: {{
                            'Content-Type': 'application/json',
                        }},
                        body: JSON.stringify({{
                            channel: 'stable',
                            mode: 'debug',
                            edition: '2021',
                            code: `{}`
                        }})
                    }});
                    
                    if (!response.ok) {{
                        throw new Error(`HTTP ${{response.status}}: ${{response.statusText}}`);
                    }}
                    
                    const result = await response.json();
                    
                    return {{
                        success: result.success,
                        output: result.stdout || result.stderr || 'No output',
                        error: result.success ? null : (result.stderr || 'Compilation failed')
                    }};
                    
                }} catch (e) {{
                    return {{
                        success: false,
                        output: '',
                        error: `Rust execution failed: ${{e.message}}`
                    }};
                }}
            }})()
        "#, escaped_code);

        self.execute_js_promise(js_code, start_time).await
    }

    #[cfg(feature = "hydrate")]
    async fn execute_go_playground(&self, code: &str, start_time: f64) -> ExecutionResult {
        let js_code = format!(r#"
            (async () => {{
                try {{
                    console.log('🐹 Executing Go code on playground...');
                    
                    const response = await fetch('https://play.golang.org/compile', {{
                        method: 'POST',
                        headers: {{
                            'Content-Type': 'application/x-www-form-urlencoded',
                        }},
                        body: 'version=2&body=' + encodeURIComponent(`{}`)
                    }});
                    
                    if (!response.ok) {{
                        throw new Error(`HTTP ${{response.status}}: ${{response.statusText}}`);
                    }}
                    
                    const result = await response.json();
                    
                    if (result.Errors) {{
                        return {{
                            success: false,
                            output: '',
                            error: result.Errors
                        }};
                    }}
                    
                    const output = result.Events ? result.Events
                        .filter(e => e.Kind === 'stdout' || e.Kind === 'stderr')
                        .map(e => e.Message)
                        .join('') : 'No output';
                    
                    return {{
                        success: true,
                        output: output,
                        error: null
                    }};
                    
                }} catch (e) {{
                    return {{
                        success: false,
                        output: '',
                        error: `Go execution failed: ${{e.message}}`
                    }};
                }}
            }})()
        "#, code);

        self.execute_js_promise(js_code, start_time).await
    }

    #[cfg(feature = "hydrate")]
    async fn execute_with_api(&self, language: &str, code: &str, start_time: f64) -> ExecutionResult {
        let escaped_code = code
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r");

        let js_code = format!(r#"
            (async () => {{
                try {{
                    console.log('⚡ Executing {} code with multiple APIs...', '{}');
                    
                    // Try Piston API (free, no auth required)
                    try {{
                        const pistonResponse = await fetch('https://emkc.org/api/v2/piston/execute', {{
                            method: 'POST',
                            headers: {{
                                'Content-Type': 'application/json',
                            }},
                            body: JSON.stringify({{
                                language: '{}',
                                version: '*',
                                files: [{{
                                    content: `{}`
                                }}]
                            }})
                        }});
                        
                        if (pistonResponse.ok) {{
                            const result = await pistonResponse.json();
                            
                            if (result.run) {{
                                const output = result.run.stdout || result.run.stderr || 'No output';
                                const success = result.run.code === 0;
                                
                                return {{
                                    success: success,
                                    output: output,
                                    error: success ? null : result.run.stderr
                                }};
                            }}
                        }}
                    }} catch (e) {{
                        console.log('Piston API failed:', e.message);
                    }}
                    
                    // Try OneCompiler API
                    try {{
                        console.log('Trying OneCompiler API...');
                        const onecompilerResponse = await fetch('https://onecompiler.com/api/code/exec', {{
                            method: 'POST',
                            headers: {{
                                'Content-Type': 'application/json',
                            }},
                            body: JSON.stringify({{
                                language: '{}',
                                code: `{}`
                            }})
                        }});
                        
                        if (onecompilerResponse.ok) {{
                            const result = await onecompilerResponse.json();
                            
                            if (result.stdout !== undefined || result.stderr !== undefined) {{
                                const output = result.stdout || result.stderr || 'No output';
                                const success = !result.stderr || result.stderr === '';
                                
                                return {{
                                    success: success,
                                    output: output,
                                    error: success ? null : result.stderr
                                }};
                            }}
                        }}
                    }} catch (e) {{
                        console.log('OneCompiler API failed:', e.message);
                    }}
                    
                    // Special handling for Python with Pyodide
                    if ('{}' === 'python') {{
                        try {{
                            console.log('🐍 Loading Pyodide for Python execution...');
                            
                            if (!window.pyodide) {{
                                const script = document.createElement('script');
                                script.src = 'https://cdn.jsdelivr.net/pyodide/v0.24.1/full/pyodide.js';
                                document.head.appendChild(script);
                                await new Promise((resolve, reject) => {{
                                    script.onload = resolve;
                                    script.onerror = reject;
                                }});
                                window.pyodide = await window.loadPyodide();
                            }}
                            
                            window.pyodide.runPython(`
import sys
from io import StringIO
sys.stdout = StringIO()
sys.stderr = StringIO()
                            `);
                            
                            window.pyodide.runPython(`{}`);
                            
                            const stdout = window.pyodide.runPython("sys.stdout.getvalue()");
                            const stderr = window.pyodide.runPython("sys.stderr.getvalue()");
                            
                            return {{
                                success: !stderr,
                                output: stdout || stderr || 'No output',
                                error: stderr ? stderr : null
                            }};
                            
                        }} catch (e) {{
                            console.log('Pyodide failed:', e.message);
                        }}
                    }}
                    
                    // If all APIs fail
                    return {{
                        success: false,
                        output: '',
                        error: `No available execution service for ${{'{}'}}. All APIs failed or timed out.`
                    }};
                    
                }} catch (e) {{
                    return {{
                        success: false,
                        output: '',
                        error: `Execution failed: ${{e.message}}`
                    }};
                }}
            }})()
        "#, 
            language,
            language,
            language,
            escaped_code,
            language,
            escaped_code,
            language,
            escaped_code,
            language
        );

        self.execute_js_promise(js_code, start_time).await
    }

    #[cfg(feature = "hydrate")]
    async fn execute_js_promise(&self, js_code: String, start_time: f64) -> ExecutionResult {
        match js_sys::eval(&js_code) {
            Ok(promise) => {
                let promise = Promise::from(promise);
                match wasm_bindgen_futures::JsFuture::from(promise).await {
                    Ok(result) => {
                        let end_time = js_sys::Date::now();
                        let success = js_sys::Reflect::get(&result, &"success".into())
                            .unwrap().as_bool().unwrap_or(false);
                        let output = js_sys::Reflect::get(&result, &"output".into())
                            .unwrap().as_string().unwrap_or_default();
                        let error = js_sys::Reflect::get(&result, &"error".into())
                            .unwrap().as_string();
                        
                        ExecutionResult {
                            success,
                            output,
                            error,
                            execution_time: Some((end_time - start_time) as u32),
                        }
                    }
                    Err(_) => ExecutionResult {
                        success: false,
                        output: String::new(),
                        error: Some("Promise execution failed".to_string()),
                        execution_time: Some((js_sys::Date::now() - start_time) as u32),
                    }
                }
            }
            Err(_) => ExecutionResult {
                success: false,
                output: String::new(),
                error: Some("Failed to initialize execution".to_string()),
                execution_time: Some((js_sys::Date::now() - start_time) as u32),
            }
        }
    }
}