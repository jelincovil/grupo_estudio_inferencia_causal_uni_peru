use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result};
use argument_audit::{
    admit_receipt, check, explain, ssch_one_decision_exact_v1, AuditGraph, CheckReport, Decision,
    LIMITS,
};
use argument_card::{
    parse_card, project, render, review_card, review_prose, Language, Lexicon, Outcome,
    SPEAK_LIMITS,
};
use argument_document::{evidence_spec_sha256, Kernel, Receipt};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "argument-audit",
    version,
    about = "Audit typed argument documents (claims, rules, evidence). A passing audit is not a certified corpus."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show profiles, limits, and the synthetic exact instance. Prints no credentials.
    Doctor,
    /// Audit one kernel document. Exit 0 is structural only.
    Check {
        #[arg(long)]
        kernel: PathBuf,
        #[arg(long)]
        json: Option<PathBuf>,
        /// Write a typed dependency graph for deterministic agent queries.
        #[arg(long)]
        emit_graph: Option<PathBuf>,
        /// Exit 3 when any obligation remains. Does not create evidence.
        #[arg(long)]
        require_verified: bool,
        #[arg(long)]
        receipts: Option<PathBuf>,
    },
    /// Explain one node or finding without changing the audit verdict.
    Explain {
        /// Recompute the graph from a kernel.
        #[arg(long, required_unless_present = "graph", conflicts_with = "graph")]
        kernel: Option<PathBuf>,
        /// Read a graph previously written by `check --emit-graph`.
        #[arg(long, required_unless_present = "kernel", conflicts_with = "kernel")]
        graph: Option<PathBuf>,
        /// Typed node/finding id, or a unique bare claim/argument/evidence id.
        id: String,
    },
    /// Run the sealed example set. Accepts `*_pending` and rejects `invalid_*`.
    Fixtures {
        #[arg(long)]
        dir: PathBuf,
    },
    /// Render a controlled human card from a kernel. Not a certificate.
    Render {
        #[arg(long)]
        kernel: PathBuf,
        #[arg(long, default_value = "en")]
        language: String,
        #[arg(long)]
        lexicon: Option<PathBuf>,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Compare a card or free prose with the kernel. Exit 0 is not an endorsement.
    Align {
        #[arg(long)]
        kernel: PathBuf,
        #[arg(long)]
        text: PathBuf,
        #[arg(long, default_value = "en")]
        language: String,
        #[arg(long)]
        lexicon: Option<PathBuf>,
        /// Treat the text as free prose even if it has no argument keys.
        #[arg(long)]
        prose: bool,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<ExitCode> {
    let cli = Cli::parse();
    match cli.command {
        Command::Doctor => {
            doctor();
            Ok(ExitCode::SUCCESS)
        }
        Command::Check {
            kernel,
            json,
            emit_graph,
            require_verified,
            receipts,
        } => check_path(
            &kernel,
            json.as_ref(),
            emit_graph.as_ref(),
            require_verified,
            receipts.as_ref(),
        ),
        Command::Explain { kernel, graph, id } => {
            explain_path(kernel.as_ref(), graph.as_ref(), &id)
        }
        Command::Fixtures { dir } => fixtures(&dir),
        Command::Render {
            kernel,
            language,
            lexicon,
            out,
        } => render_path(&kernel, &language, lexicon.as_ref(), out.as_ref()),
        Command::Align {
            kernel,
            text,
            language,
            lexicon,
            prose,
        } => align_path(&kernel, &text, &language, lexicon.as_ref(), prose),
    }
}

fn doctor() {
    println!("argument-audit {}", env!("CARGO_PKG_VERSION"));
    println!("checker: {}", argument_audit::CHECKER_VERSION);
    println!("profiles: scientific_audit (schema 1.0 default), structural (schema 1.1 default)");
    println!("evidence classes: lean, python, registered");
    println!("registered is trackable only; it is not a proof and not a reproduction");
    println!("limits: {LIMITS}");
    println!("reading: {SPEAK_LIMITS}");
    match ssch_one_decision_exact_v1() {
        Some(instance) => println!(
            "synthetic fixture {}: risk0={}, risk1={}, bayes={}, scope={}",
            instance.experiment,
            instance.risk0,
            instance.risk1,
            instance.bayes_risk,
            instance.scope
        ),
        None => println!("synthetic fixture ssch_one_decision_exact_v1: arithmetic error"),
    }
}

fn check_path(
    path: &PathBuf,
    json: Option<&PathBuf>,
    emit_graph: Option<&PathBuf>,
    require_verified: bool,
    receipts: Option<&PathBuf>,
) -> Result<ExitCode> {
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let kernel = match Kernel::load(&bytes) {
        Ok(kernel) => kernel,
        Err(err) => {
            eprintln!("contract: {err}");
            return Ok(ExitCode::from(2));
        }
    };
    let report = check(&kernel);
    if let Some(receipts) = receipts {
        admit_file(&kernel, receipts)?;
    }
    if let Some(out) = emit_graph {
        let graph = AuditGraph::from_report(&kernel, &report)
            .map_err(|err| anyhow::anyhow!(err.to_string()))?;
        let rendered = serde_json::to_string_pretty(&graph)?;
        fs::write(out, format!("{rendered}\n"))
            .with_context(|| format!("writing {}", out.display()))?;
    }
    let rendered = serde_json::to_string_pretty(&report)?;
    if let Some(out) = json {
        fs::write(out, format!("{rendered}\n"))
            .with_context(|| format!("writing {}", out.display()))?;
    } else {
        println!("{rendered}");
    }
    Ok(exit_for(&report, require_verified))
}

fn explain_path(kernel: Option<&PathBuf>, graph: Option<&PathBuf>, id: &str) -> Result<ExitCode> {
    let graph = match (kernel, graph) {
        (Some(path), None) => {
            let kernel = load_kernel(path)?;
            let report = check(&kernel);
            AuditGraph::from_report(&kernel, &report)
                .map_err(|err| anyhow::anyhow!(err.to_string()))?
        }
        (None, Some(path)) => {
            let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
            AuditGraph::from_json_slice(&bytes).map_err(|err| anyhow::anyhow!(err.to_string()))?
        }
        _ => anyhow::bail!("provide exactly one of --kernel or --graph"),
    };
    let explanation = explain(&graph, id).map_err(|err| anyhow::anyhow!(err.to_string()))?;
    if explanation.claim_supported {
        anyhow::bail!("internal error: an explanation certified a claim");
    }
    println!("{}", serde_json::to_string_pretty(&explanation)?);
    Ok(ExitCode::SUCCESS)
}

fn admit_file(kernel: &Kernel, path: &PathBuf) -> Result<()> {
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let receipts = Receipt::load_all(&bytes).map_err(|err| anyhow::anyhow!(err.to_string()))?;
    for receipt in receipts {
        let Some(spec) = kernel
            .evidence_specs
            .iter()
            .find(|spec| spec.id == receipt.spec_id)
        else {
            println!(
                "admission {}: spec `{}` is not in this kernel; claim_supported=false",
                receipt.id, receipt.spec_id
            );
            continue;
        };
        let hash = evidence_spec_sha256(spec).map_err(|err| anyhow::anyhow!(err.to_string()))?;
        let admission = admit_receipt(&receipt, spec, &hash);
        println!("{}", serde_json::to_string(&admission)?);
        if admission.claim_supported {
            anyhow::bail!("internal error: admission set claim_supported");
        }
    }
    Ok(())
}

fn fixtures(dir: &PathBuf) -> Result<ExitCode> {
    let mut entries = fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    let mut checks = Vec::new();
    let mut failed = false;
    for entry in entries {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        let expected = !name.starts_with("invalid_");
        let bytes = fs::read(&path).with_context(|| format!("reading {}", path.display()))?;
        let accepted = match Kernel::load(&bytes) {
            Ok(kernel) => check(&kernel).decision == Decision::StructurallyValid,
            Err(_) => false,
        };
        if accepted != expected {
            failed = true;
        }
        checks.push(serde_json::json!({
            "file": name,
            "expected_accept": expected,
            "accepted": accepted,
        }));
    }
    let summary = serde_json::json!({
        "package_validation": if failed { "FAIL" } else { "PASS" },
        "fixture_count": checks.len(),
        "checks": checks,
        "limit": LIMITS,
    });
    println!("{}", serde_json::to_string_pretty(&summary)?);
    if failed {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

fn render_path(
    path: &PathBuf,
    language: &str,
    lexicon: Option<&PathBuf>,
    out: Option<&PathBuf>,
) -> Result<ExitCode> {
    let kernel = load_kernel(path)?;
    let language = Language::parse(language).map_err(|err| anyhow::anyhow!(err.to_string()))?;
    let lexicon = load_lexicon(lexicon, language)?;
    let reading =
        render(&kernel, &lexicon, language).map_err(|err| anyhow::anyhow!(err.to_string()))?;
    if let Some(path) = out {
        fs::write(path, &reading.card).with_context(|| format!("writing {}", path.display()))?;
    } else {
        print!("{}", reading.card);
    }
    eprintln!(
        "reading: claim_supported=false alignment_guaranteed=false glosses_pending={}",
        reading.glosses_pending.len()
    );
    Ok(ExitCode::SUCCESS)
}

fn align_path(
    kernel_path: &PathBuf,
    text_path: &PathBuf,
    language: &str,
    lexicon: Option<&PathBuf>,
    prose: bool,
) -> Result<ExitCode> {
    let kernel = load_kernel(kernel_path)?;
    let language = Language::parse(language).map_err(|err| anyhow::anyhow!(err.to_string()))?;
    let lexicon = load_lexicon(lexicon, language)?;
    let text = fs::read_to_string(text_path)
        .with_context(|| format!("reading {}", text_path.display()))?;
    let looks_like_card = text.lines().any(|line| line.starts_with("argument: "))
        || text.starts_with("argument-card ");
    let report = if prose || !looks_like_card {
        review_prose(&kernel, &text, &lexicon).map_err(|err| anyhow::anyhow!(err.to_string()))?
    } else {
        review_card(&kernel, &text, &lexicon, language)
            .map_err(|err| anyhow::anyhow!(err.to_string()))?
    };
    let parsed_matches = match (parse_card(&text), project(&kernel)) {
        (Ok(parsed), Ok(formal)) => parsed == formal,
        _ => false,
    };
    println!("{}", serde_json::to_string_pretty(&report)?);
    eprintln!(
        "align: outcome={} endorsement={} formal_round_trip={parsed_matches}",
        outcome_label(report.outcome),
        report.endorsement
    );
    if report.claim_supported || report.alignment_guaranteed {
        anyhow::bail!("internal error: a reading review certified alignment");
    }
    Ok(exit_for_review(report.outcome))
}

fn load_kernel(path: &PathBuf) -> Result<Kernel> {
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    Kernel::load(&bytes).map_err(|err| anyhow::anyhow!("contract: {err}"))
}

fn load_lexicon(path: Option<&PathBuf>, language: Language) -> Result<Lexicon> {
    let Some(path) = path else {
        return Ok(Lexicon::empty(language));
    };
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    Lexicon::load(&bytes).map_err(|err| anyhow::anyhow!(err.to_string()))
}

fn outcome_label(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::NoBlockingProblem => "no_blocking_problem",
        Outcome::BlockingProblem => "blocking_problem",
        Outcome::Unresolved => "unresolved",
    }
}

fn exit_for_review(outcome: Outcome) -> ExitCode {
    match outcome {
        Outcome::NoBlockingProblem | Outcome::Unresolved => ExitCode::SUCCESS,
        Outcome::BlockingProblem => ExitCode::from(3),
    }
}

fn exit_for(report: &CheckReport, require_verified: bool) -> ExitCode {
    if report.decision == Decision::Rejected {
        return ExitCode::from(3);
    }
    if require_verified
        && (report.pending_count > 0 || report.decision != Decision::StructurallyValid)
    {
        return ExitCode::from(3);
    }
    ExitCode::SUCCESS
}
