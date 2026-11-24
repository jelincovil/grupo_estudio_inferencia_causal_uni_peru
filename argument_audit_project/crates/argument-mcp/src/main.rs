//! argument-mcp — Servidor MCP (stdio) para argument-audit.
//!
//! Expone dos herramientas de agentes:
//!   - argument_check   { kernel: path, require_verified?: bool }
//!   - argument_explain { id: node-id, kernel?: path, graph?: path }
//!
//! Transporte: JSON-RPC 2.0 delimitado por líneas sobre stdin/stdout (MCP).
//! Sin red, sin llamadas a modelos: las verificaciones son deterministas.

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::exit;

use argument_audit::{check, explain, AuditGraph, Decision};
use argument_document::Kernel;
use serde_json::{json, Value};

const PROTOCOL_VERSION: &str = "2024-11-05";

fn tools() -> Value {
    json!({
        "tools": [
            {
                "name": "argument_check",
                "description": "Run the deterministic administrative check on a kernel document. Returns the full report as JSON (decision, claim_supported, pending_count, diagnostics, ...). claim_supported is never promoted by this tool.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "kernel": { "type": "string", "description": "Path to the kernel JSON file" },
                        "require_verified": { "type": "boolean", "description": "If true, exit-style semantics: any pending obligation yields a non-zero verdict" }
                    },
                    "required": ["kernel"]
                }
            },
            {
                "name": "argument_explain",
                "description": "Explain one node of the deterministic context graph. Provide exactly one of kernel (recompute the graph) or graph (a saved --emit-graph file). Query id can be a typed node id (claim:, argument:, evidence_spec:, finding:) or a unique bare id.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "kernel": { "type": "string", "description": "Path to the kernel JSON file" },
                        "graph": { "type": "string", "description": "Path to a graph emitted with --emit-graph" },
                        "id": { "type": "string", "description": "Node id to explain (typed or unique bare id)" }
                    },
                    "required": ["id"]
                }
            }
        ]
    })
}

fn tool_check(args: &Value) -> Result<Value, String> {
    let path = args
        .get("kernel")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing string argument `kernel`".to_string())?;
    let require_verified = args
        .get("require_verified")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let bytes = std::fs::read(path).map_err(|e| format!("cannot read {path}: {e}"))?;
    let kernel = Kernel::load(&bytes).map_err(|e| format!("kernel rejected: {e}"))?;
    let report = check(&kernel);
    let exit_code = if report.decision == Decision::StructurallyValid
        && !(require_verified && report.pending_count > 0)
    {
        0
    } else {
        3
    };
    let mut value = serde_json::to_value(&report).map_err(|e| e.to_string())?;
    value["exit_code"] = json!(exit_code);
    Ok(value)
}

fn load_graph(args: &Value) -> Result<AuditGraph, String> {
    let kernel_path = args.get("kernel").and_then(Value::as_str);
    let graph_path = args.get("graph").and_then(Value::as_str);
    match (kernel_path, graph_path) {
        (Some(_), Some(_)) => Err("provide exactly one of `kernel` or `graph`, not both".into()),
        (Some(path), None) => {
            let bytes = std::fs::read(path).map_err(|e| format!("cannot read {path}: {e}"))?;
            let kernel = Kernel::load(&bytes).map_err(|e| format!("kernel rejected: {e}"))?;
            let report = check(&kernel);
            AuditGraph::from_report(&kernel, &report)
                .map_err(|e| format!("graph rejected: {e}"))
        }
        (None, Some(path)) => {
            let bytes = std::fs::read(path).map_err(|e| format!("cannot read {path}: {e}"))?;
            AuditGraph::from_json_slice(&bytes).map_err(|e| format!("graph rejected: {e}"))
        }
        (None, None) => Err("provide exactly one of `kernel` or `graph`".into()),
    }
}

fn tool_explain(args: &Value) -> Result<Value, String> {
    let id = args
        .get("id")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing string argument `id`".to_string())?;
    let graph = load_graph(args)?;
    let explanation = explain(&graph, id).map_err(|e| e.to_string())?;
    serde_json::to_value(&explanation).map_err(|e| e.to_string())
}

fn call_tool(name: &str, args: &Value) -> Value {
    let result = match name {
        "argument_check" => tool_check(args),
        "argument_explain" => tool_explain(args),
        other => Err(format!("unknown tool `{other}`")),
    };
    match result {
        Ok(value) => json!({
            "content": [{ "type": "text", "text": value.to_string() }],
            "isError": false
        }),
        Err(message) => json!({
            "content": [{ "type": "text", "text": message }],
            "isError": true
        }),
    }
}

fn handle(message: &Value) -> Option<Value> {
    let id = message.get("id").cloned();
    let method = message.get("method").and_then(Value::as_str).unwrap_or("");
    match method {
        "initialize" => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "argument-audit", "version": env!("CARGO_PKG_VERSION") }
            }
        })),
        "notifications/initialized" => None,
        "ping" => Some(json!({ "jsonrpc": "2.0", "id": id, "result": {} })),
        "tools/list" => Some(json!({ "jsonrpc": "2.0", "id": id, "result": tools() })),
        "tools/call" => {
            let params = message.get("params").cloned().unwrap_or(json!({}));
            let name = params.get("name").and_then(Value::as_str).unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or(json!({}));
            Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": call_tool(name, &args)
            }))
        }
        other => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": { "code": -32601, "message": format!("method not found: {other}") }
        })),
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().lock();
    for line in BufReader::new(stdin.lock()).lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }
        let message: Value = match serde_json::from_str(&line) {
            Ok(m) => m,
            Err(e) => {
                let _ = writeln!(stdout, "{}", json!({
                    "jsonrpc": "2.0",
                    "id": Value::Null,
                    "error": { "code": -32700, "message": format!("parse error: {e}") }
                }));
                let _ = stdout.flush();
                continue;
            }
        };
        if let Some(response) = handle(&message) {
            let _ = writeln!(stdout, "{}", response);
            let _ = stdout.flush();
        }
    }
    exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_example() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/structural_alias.json")
    }

    #[test]
    fn tools_list_offers_both_tools() {
        let list = tools();
        let names: Vec<&str> = list["tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, ["argument_check", "argument_explain"]);
    }

    #[test]
    fn argument_check_on_repo_example() {
        let args = json!({ "kernel": repo_example().to_string_lossy() });
        let out = tool_check(&args).expect("check");
        assert!(out["exit_code"].is_number());
        assert_eq!(out["claim_supported"], json!(false));
    }

    #[test]
    fn argument_check_missing_file_is_error() {
        let args = json!({ "kernel": "/no/such/file.json" });
        assert!(tool_check(&args).is_err());
    }

    #[test]
    fn argument_explain_requires_exactly_one_source() {
        let both = json!({ "kernel": "a.json", "graph": "b.json", "id": "C.Q" });
        assert!(tool_explain(&both).is_err());
        let neither = json!({ "id": "C.Q" });
        assert!(tool_explain(&neither).is_err());
    }

    #[test]
    fn argument_explain_on_repo_example() {
        let args = json!({ "kernel": repo_example().to_string_lossy(), "id": "A.alias" });
        let out = tool_explain(&args).expect("explain");
        assert!(out.is_object());
    }

    #[test]
    fn initialize_and_unknown_method() {
        let init = handle(&json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize" })).unwrap();
        assert_eq!(init["result"]["protocolVersion"], PROTOCOL_VERSION);
        let err = handle(&json!({ "jsonrpc": "2.0", "id": 2, "method": "nope" })).unwrap();
        assert_eq!(err["error"]["code"], -32601);
    }
}
