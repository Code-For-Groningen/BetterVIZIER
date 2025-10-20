use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::io::{BufRead, BufReader, Write};

fn get_editor_line_number(editor_text: &str, fitch_line: u32) -> usize {
    let search_str = fitch_line.to_string();

    editor_text
        .lines()
        .position(|line| line.starts_with(&search_str))
        .map(|pos| pos + 1)
        .unwrap_or(0)
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    result: Option<Value>,
}

#[derive(Debug, Clone)]
struct Document {
    uri: String,
    text: String,
    version: i64,
}

struct LanguageServer {
    documents: std::collections::HashMap<String, Document>,
}

impl LanguageServer {
    fn new() -> Self {
        Self {
            documents: std::collections::HashMap::new(),
        }
    }

    fn handle_message(&mut self, msg: Message) -> Option<Message> {
        match msg.method.as_deref() {
            Some("initialize") => self.handle_initialize(msg.id),
            Some("initialized") => None,
            Some("textDocument/didOpen") => {
                self.handle_did_open(msg.params?);
                None
            }
            Some("textDocument/didChange") => {
                self.handle_did_change(msg.params?);
                None
            }
            Some("textDocument/formatting") => self.handle_formatting(msg.id, msg.params?),
            Some("textDocument/semanticTokens/full") => {
                self.handle_semantic_tokens(msg.id, msg.params?)
            }
            Some("textDocument/completion") => self.handle_completion(msg.id, msg.params?),
            Some("shutdown") => Some(Message {
                jsonrpc: "2.0".to_string(),
                id: msg.id,
                method: None,
                params: None,
                result: Some(Value::Null),
            }),
            Some("exit") => std::process::exit(0),
            _ => None,
        }
    }

    fn handle_initialize(&self, id: Option<i64>) -> Option<Message> {
        let capabilities = serde_json::json!({
            "capabilities": {
                "textDocumentSync": {
                    "openClose": true,
                    "change": 1, // Full sync
                },
                "documentFormattingProvider": true,
                "semanticTokensProvider": {
                    "legend": {
                        "tokenTypes": ["keyword", "variable", "string", "number", "comment"],
                        "tokenModifiers": []
                    },
                    "full": true
                },
                "completionProvider": {
                    "resolveProvider": false,
                    "triggerCharacters": []
                }
            }
        });

        Some(Message {
            jsonrpc: "2.0".to_string(),
            id,
            method: None,
            params: None,
            result: Some(capabilities),
        })
    }

    fn handle_did_open(&mut self, params: Value) {
        if let Some(text_doc) = params.get("textDocument") {
            let uri = text_doc["uri"].as_str().unwrap_or("").to_string();
            let text = text_doc["text"].as_str().unwrap_or("").to_string();
            let version = text_doc["version"].as_i64().unwrap_or(0);

            self.documents.insert(
                uri.clone(),
                Document {
                    uri: uri.clone(),
                    text: text.clone(),
                    version,
                },
            );

            // Send diagnostics
            self.send_diagnostics(&uri, &text);
        }
    }

    fn handle_did_change(&mut self, params: Value) {
        if let Some(text_doc) = params.get("textDocument") {
            let uri = text_doc["uri"].as_str().unwrap_or("").to_string();
            let version = text_doc["version"].as_i64().unwrap_or(0);

            if let Some(changes) = params["contentChanges"].as_array() {
                if let Some(change) = changes.first() {
                    let text = change["text"].as_str().unwrap_or("").to_string();

                    self.documents.insert(
                        uri.clone(),
                        Document {
                            uri: uri.clone(),
                            text: text.clone(),
                            version,
                        },
                    );

                    self.send_diagnostics(&uri, &text);
                }
            }
        }
    }

    fn handle_formatting(&self, id: Option<i64>, params: Value) -> Option<Message> {
        let uri = params["textDocument"]["uri"].as_str()?;
        let doc = self.documents.get(uri)?;

        // let result: String = fitch_proof::check_proof_with_template(&proof, template, &variables);

        let formatted: String = fitch_proof::format_proof(&doc.text);
        if formatted == "invalid" {
            return None;
        }

        let result = serde_json::json!([{
            "range": {
                "start": { "line": 0, "character": 0 },
                "end": {
                    "line": doc.text.lines().count(),
                    "character": 0
                }
            },
            "newText": formatted
        }]);

        Some(Message {
            jsonrpc: "2.0".to_string(),
            id,
            method: None,
            params: None,
            result: Some(result),
        })
    }

    fn handle_semantic_tokens(&self, id: Option<i64>, params: Value) -> Option<Message> {
        eprintln!("Semantic tokens requested");
        let uri = params["textDocument"]["uri"].as_str()?;
        let doc = self.documents.get(uri)?;

        // Simple tokenization example
        let mut tokens: Vec<u32> = Vec::new();
        let keywords = ["∀", "∃"];

        for (line_idx, line) in doc.text.lines().enumerate() {
            let mut char_idx = 0;
            for word in line.split_whitespace() {
                if let Some(pos) = line[char_idx..].find(word) {
                    char_idx += pos;
                    let token_type = if keywords.contains(&word) {
                        0 // keyword
                    } else if word.starts_with('"') {
                        2 // string
                    } else if word.parse::<f64>().is_ok() {
                        3 // number
                    } else {
                        1 // variable
                    };

                    tokens.extend_from_slice(&[
                        line_idx as u32,
                        char_idx as u32,
                        word.len() as u32,
                        token_type,
                        0,
                    ]);

                    char_idx += word.len();
                }
            }
        }

        let result = serde_json::json!({ "data": tokens });
        eprintln!("{}", result);

        Some(Message {
            jsonrpc: "2.0".to_string(),
            id,
            method: None,
            params: None,
            result: Some(result),
        })
    }

    fn handle_completion(&self, id: Option<i64>, params: Value) -> Option<Message> {
        eprintln!("Completion requested");

        let completions = vec![
            serde_json::json!({
                "label": "∧",
                "kind": 15,
                "insertTextFormat": 2,
                "insertText": "∧",
                "documentation": "Conjunction",
                "filterText": "conjunction and"
            }),
            serde_json::json!({
                "label": "∨",
                "kind": 15,
                "insertTextFormat": 2,
                "insertText": "∨",
                "documentation": "Disjunction",
                "filterText": "disjunction or"
            }),
            serde_json::json!({
                "label": "fa",
                "kind": 15,
                "insertTextFormat": 2,
                "insertText": "∀",
                "documentation": "For all (universal quantifier)",
                "filterText": "fa forall"
            }),
            serde_json::json!({
                "label": "ex",
                "kind": 15,
                "insertTextFormat": 2,
                "insertText": "∃",
                "documentation": "There exists (existential quantifier)",
                "filterText": "ex exists"
            }),
            serde_json::json!({
                "label": "→",
                "kind": 15,
                "insertTextFormat": 2,
                "insertText": "→",
                "documentation": "Implies",
                "filterText": "implies implication if"
            }),
            serde_json::json!({
                "label": "↔",
                "kind": 15,
                "insertTextFormat": 2,
                "insertText": "↔",
                "documentation": "Doube implication",
                "filterText": "bic double"
            }),
            serde_json::json!({
                "label": "⊥",
                "kind": 15,
                "insertTextFormat": 2,
                "insertText": "⊥",
                "documentation": "Bottom",
                "filterText": "bottom false contradiction"
            }),
            serde_json::json!({
                "label": "¬",
                "kind": 15,
                "insertTextFormat": 2,
                "insertText": "¬",
                "documentation": "Negation",
                "filterText": "not neg !"
            }),
        ];

        let result = serde_json::json!({
            "isIncomplete": false,
            "items": completions
        });

        Some(Message {
            jsonrpc: "2.0".to_string(),
            id,
            method: None,
            params: None,
            result: Some(result),
        })
    }

    fn send_diagnostics(&self, uri: &str, text: &str) {
        let mut diagnostics = Vec::new();

        let errorstring = fitch_proof::check_proof(text, "x,y,z,u,v,w");

        // Example: Check for lines longer than 100 characters
        for (i, err_line) in errorstring.lines().enumerate() {
            if err_line.trim() == "" || err_line.contains("correct!") {
                continue;
            }

            let re = Regex::new(r"(?:line\s+)(\d+)").unwrap();
            let fitch_line_num = re
                .captures(err_line)
                .and_then(|caps| caps.get(1))
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .unwrap_or(1);

            let editor_line_num = get_editor_line_number(text, fitch_line_num);
            diagnostics.push(serde_json::json!({
                "range": {
                    "start": { "line": editor_line_num, "character": 0 },
                    "end": { "line": editor_line_num, "character": 100 }
                },
                "severity": 1, // Error
                "source": "fitch_lsp",
                "message": err_line
            }));
        }

        let notification = Message {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: Some("textDocument/publishDiagnostics".to_string()),
            params: Some(serde_json::json!({
                "uri": uri,
                "diagnostics": diagnostics
            })),
            result: None,
        };

        Self::send_message(&notification);
    }

    fn send_message(msg: &Message) {
        let content = serde_json::to_string(msg).unwrap();
        let header = format!("Content-Length: {}\r\n\r\n", content.len());
        print!("{}{}", header, content);
        std::io::stdout().flush().unwrap();
    }
}

fn read_message<R: BufRead>(reader: &mut R) -> Result<Message, Box<dyn Error>> {
    let mut content_length = 0;

    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;

        if line.trim().is_empty() {
            break;
        }

        if line.starts_with("Content-Length:") {
            content_length = line[15..].trim().parse()?;
        }
    }

    let mut content = vec![0u8; content_length];
    reader.read_exact(&mut content)?;

    let msg: Message = serde_json::from_slice(&content)?;
    Ok(msg)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut server = LanguageServer::new();

    loop {
        let msg = read_message(&mut reader)?;

        if let Some(response) = server.handle_message(msg) {
            LanguageServer::send_message(&response);
        }
    }
}
