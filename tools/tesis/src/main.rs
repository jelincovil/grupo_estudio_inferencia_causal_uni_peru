//! tesis — CLI para crear y verificar proyectos de tesis UNI.
//!
//! Plantillas embebidas desde este mismo repositorio (include_bytes!),
//! sin descargas de red en tiempo de ejecución.

use anyhow::{bail, Context, Result};
use clap::{Arg, ArgAction, Command};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

// ---------------------------------------------------------------------------
// Plantillas embebidas
// ---------------------------------------------------------------------------
enum TemplateKind {
    Typst,
    Latex,
}

struct Asset {
    rel: &'static str,
    bytes: &'static [u8],
}

const TYPST_ASSETS: &[Asset] = &[
    Asset { rel: "tesis.typ", bytes: include_bytes!("../templates/typst/tesis.typ") },
    Asset { rel: "tesis_doctorado.typ", bytes: include_bytes!("../templates/typst/tesis_doctorado.typ") },
    Asset { rel: "plantilla.typ", bytes: include_bytes!("../templates/typst/plantilla.typ") },
    Asset { rel: "referencias.bib", bytes: include_bytes!("../templates/typst/referencias.bib") },
    Asset { rel: "logo_uni.png", bytes: include_bytes!("../templates/typst/logo_uni.png") },
    Asset { rel: "compilar.sh", bytes: include_bytes!("../templates/typst/compilar.sh") },
    Asset { rel: "README.md", bytes: include_bytes!("../templates/typst/README.md") },
    Asset { rel: "requirements.txt", bytes: include_bytes!("../templates/typst/requirements.txt") },
];

const LATEX_ASSETS: &[Asset] = &[
    Asset { rel: "tesis.tex", bytes: include_bytes!("../templates/latex/tesis.tex") },
    Asset { rel: "referencias.bib", bytes: include_bytes!("../templates/latex/referencias.bib") },
    Asset { rel: "logo_uni.png", bytes: include_bytes!("../templates/latex/logo_uni.png") },
    Asset { rel: "compilar.sh", bytes: include_bytes!("../templates/latex/compilar.sh") },
    Asset { rel: "README.md", bytes: include_bytes!("../templates/latex/README.md") },
    Asset { rel: "GUIA_COMPLETA.md", bytes: include_bytes!("../templates/latex/GUIA_COMPLETA.md") },
];

fn assets_for(kind: &TemplateKind) -> &'static [Asset] {
    match kind {
        TemplateKind::Typst => TYPST_ASSETS,
        TemplateKind::Latex => LATEX_ASSETS,
    }
}

// ---------------------------------------------------------------------------
// Metadatos a sustituir
// ---------------------------------------------------------------------------
#[derive(Default, Clone)]
struct Meta {
    titulo: Option<String>,
    autor: Option<String>,
    asesor: Option<String>,
    anio: Option<String>,
}

fn validar_valor(campo: &str, valor: &str, prohibidos: &[char]) -> Result<()> {
    for c in prohibidos {
        if valor.contains(*c) {
            bail!("el valor de --{campo} no puede contener el carácter `{}`", c);
        }
    }
    if valor.trim().is_empty() {
        bail!("el valor de --{campo} está vacío");
    }
    Ok(())
}

fn validar_meta(m: &Meta) -> Result<()> {
    let texto = [
        ("titulo", &m.titulo, &['"', '\\', '\n', '\r'][..]),
        ("autor", &m.autor, &['"', '\\', '\n', '\r'][..]),
        ("asesor", &m.asesor, &['"', '\\', '\n', '\r'][..]),
        ("anio", &m.anio, &['{', '}', '\\', '\n', '\r'][..]),
    ];
    for (campo, valor, prohibidos) in texto {
        if let Some(v) = valor {
            validar_valor(campo, v, prohibidos)?;
        }
    }
    if let Some(a) = &m.anio {
        let ok = Regex::new(r"^\d{4}$").unwrap();
        if !ok.is_match(a) {
            bail!("--anio debe ser un año de 4 dígitos, no `{a}`");
        }
    }
    Ok(())
}

fn replace_typst_key(src: &str, key: &str, value: &str) -> String {
    let pattern = format!(r#"(?m)^(\s*{}:\s*)"(?:[^"\\]|\\.)*""#, regex::escape(key));
    let re = Regex::new(&pattern).expect("regex typst");
    re.replace_all(src, |caps: &regex::Captures| {
        format!("{}\"{}\"", &caps[1], value)
    })
    .into_owned()
}

fn replace_typst_anio(src: &str, anio: &str) -> String {
    let re = Regex::new(r"(?m)^(\s*anio:\s*)\d{4}").expect("regex anio");
    re.replace_all(src, |caps: &regex::Captures| format!("{}{}", &caps[1], anio))
        .into_owned()
}

fn replace_latex_key(src: &str, key: &str, value: &str) -> String {
    // Estructura real en tesis.tex: \\newcommand{\\KEY}{VALUE}
    // (p. ej. \\newcommand{\\docauthor}{Nombre del Autor}).
    // regex: ([{]\\KEY[}][{])[^}]*([}])
    let mut pattern = String::from("([{]\\\\");
    pattern.push_str(&regex::escape(key));
    pattern.push_str("[}][{])");
    pattern.push_str("[^}]*");
    pattern.push_str("([}])");
    let re = Regex::new(&pattern).expect("regex latex");
    re.replace_all(src, |caps: &regex::Captures| format!("{}{}{}", &caps[1], value, &caps[2]))
        .into_owned()
}

fn aplicar_meta_typst(src: &str, m: &Meta) -> String {
    let mut out = src.to_string();
    if let Some(v) = &m.titulo {
        out = replace_typst_key(&out, "titulo", v);
    }
    if let Some(v) = &m.autor {
        out = replace_typst_key(&out, "autor", v);
    }
    if let Some(v) = &m.asesor {
        out = replace_typst_key(&out, "asesor", v);
    }
    if let Some(v) = &m.anio {
        out = replace_typst_anio(&out, v);
    }
    out
}

fn aplicar_meta_latex(src: &str, m: &Meta) -> String {
    let mut out = src.to_string();
    if let Some(v) = &m.titulo {
        out = replace_latex_key(&out, "doctitle", v);
    }
    if let Some(v) = &m.autor {
        out = replace_latex_key(&out, "docauthor", v);
    }
    if let Some(v) = &m.asesor {
        out = replace_latex_key(&out, "docadvisor", v);
    }
    if let Some(v) = &m.anio {
        out = replace_latex_key(&out, "docyear", v);
    }
    out
}

// ---------------------------------------------------------------------------
// Copia de plantillas
// ---------------------------------------------------------------------------
fn tipo_desde_str(s: &str) -> Result<TemplateKind> {
    match s {
        "typst" => Ok(TemplateKind::Typst),
        "latex" => Ok(TemplateKind::Latex),
        otro => bail!("plantilla desconocida `{otro}` (use typst o latex)"),
    }
}

fn copiar_plantilla(kind: &TemplateKind, destino: &Path, meta: &Meta, force: bool) -> Result<()> {
    if destino.exists() {
        if !destino.is_dir() {
            bail!("{} ya existe y no es un directorio", destino.display());
        }
        let vacio = fs::read_dir(destino)
            .with_context(|| format!("no se puede leer {}", destino.display()))?
            .next()
            .is_none();
        if !vacio && !force {
            bail!(
                "{} no está vacío; use --force para sobrescribir los archivos de la plantilla \
                 (los demás archivos se conservan; nada se borra)",
                destino.display()
            );
        }
    } else {
        fs::create_dir_all(destino)
            .with_context(|| format!("no se puede crear {}", destino.display()))?;
    }

    let assets = assets_for(kind);
    for asset in assets {
        let ruta = destino.join(asset.rel);
        if let Some(parent) = ruta.parent() {
            fs::create_dir_all(parent)?;
        }
        let contenido = asset.bytes.to_vec();
        let path_str = asset.rel;
        let es_meta = matches!(path_str, "tesis.typ" | "tesis.tex" | "tesis_doctorado.typ");
        if es_meta {
            let texto = String::from_utf8(contenido)
                .context("la plantilla embebida no es UTF-8 válido")?;
            let texto = match kind {
                TemplateKind::Typst => aplicar_meta_typst(&texto, meta),
                TemplateKind::Latex => aplicar_meta_latex(&texto, meta),
            };
            fs::write(&ruta, texto)?;
        } else {
            fs::write(&ruta, contenido)?;
        }
    }
    // compilar.sh debe conservar el bit de ejecutable
    let compilar = destino.join("compilar.sh");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&compilar)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&compilar, perms)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// tesis check
// ---------------------------------------------------------------------------
struct ReporteCheck {
    problemas: Vec<String>,
    notas: Vec<String>,
}

fn scan_lineas_placeholder(nombre: &str, contenido: &str) -> Vec<String> {
    // Subcadenas literales (siempre son placeholder)
    let literales = [
        "Nombre del Autor",
        "Nombre del Asesor",
        "Título de la Tesis Doctoral",
        "palabra clave 1",
        "sustituya por",
    ];
    // Marcas de palabra completa (TODO aparece dentro de METODOLOGÍA, etc.)
    let palabra = [r"\bTODO\b", r"\bXXX\b"];
    let palabra_re: Vec<Regex> = palabra.iter().map(|p| Regex::new(p).expect("regex marca")).collect();

    let mut out = Vec::new();
    for (i, linea) in contenido.lines().enumerate() {
        // Las definiciones de macro (\newcommand{\TODO}…) no son usos pendientes
        if linea.contains("\\newcommand{\\TODO}") {
            continue;
        }
        for marca in literales {
            if linea.contains(marca) {
                out.push(format!("{}:{}: placeholder `{}` → {}", nombre, i + 1, marca, linea.trim()));
            }
        }
        for (marca, re) in ["TODO", "XXX"].iter().zip(&palabra_re) {
            if re.is_match(linea) {
                out.push(format!("{}:{}: placeholder `{}` → {}", nombre, i + 1, marca, linea.trim()));
            }
        }
    }
    out
}

fn ejecutar_check(dir: &Path) -> ReporteCheck {
    let mut problemas = Vec::new();
    let mut notas = Vec::new();

    if !dir.is_dir() {
        problemas.push(format!("{} no es un directorio", dir.display()));
        return ReporteCheck { problemas, notas };
    }

    let typst = dir.join("tesis.typ");
    let latex = dir.join("tesis.tex");
    let es_typst = typst.is_file();
    let es_latex = latex.is_file();

    if !es_typst && !es_latex {
        problemas.push("no se encontró `tesis.typ` ni `tesis.tex` en el directorio".into());
        return ReporteCheck { problemas, notas };
    }

    let principales: Vec<PathBuf> = if es_typst {
        let mut v = vec![typst];
        let doctorado = dir.join("tesis_doctorado.typ");
        if doctorado.is_file() {
            v.push(doctorado);
        }
        v
    } else {
        vec![latex]
    };

    for principal in &principales {
        let nombre = principal.file_name().unwrap().to_string_lossy().into_owned();
        let contenido = match fs::read_to_string(principal) {
            Ok(c) => c,
            Err(e) => {
                problemas.push(format!("{}: no se puede leer ({})", nombre, e));
                continue;
            }
        };
        problemas.extend(scan_lineas_placeholder(&nombre, &contenido));
    }

    // Archivos críticos que la plantilla asume presentes
    let requeridos: &[&str] = if es_typst {
        &["plantilla.typ", "referencias.bib", "logo_uni.png"]
    } else {
        &["referencias.bib", "logo_uni.png"]
    };
    for req in requeridos {
        if !dir.join(req).is_file() {
            problemas.push(format!("falta el archivo requerido `{req}`"));
        }
    }

    if es_typst {
        let refs = fs::read_to_string(dir.join("referencias.bib")).unwrap_or_default();
        if refs.matches('@').count() == 0 {
            notas.push("`referencias.bib` no contiene entradas `@…`".to_string());
        }
    }

    ReporteCheck { problemas, notas }
}

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------
fn cmd() -> Command {
    Command::new("tesis")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Crea y verifica proyectos de tesis UNI (plantillas Typst y LaTeX)")
        .subcommand_required(true)
        .subcommand(
            Command::new("new")
                .about("Crea un proyecto nuevo a partir de la plantilla")
                .arg(Arg::new("destino").required(true).help("Directorio de destino"))
                .arg(Arg::new("plantilla").long("plantilla").value_parser(["typst", "latex"]).default_value("typst"))
                .arg(Arg::new("autor").long("autor").help("Nombre del autor"))
                .arg(Arg::new("titulo").long("titulo").help("Título de la tesis"))
                .arg(Arg::new("asesor").long("asesor").help("Nombre y grado del asesor"))
                .arg(Arg::new("anio").long("anio").help("Año (4 dígitos)"))
                .arg(Arg::new("force").long("force").action(ArgAction::SetTrue).help("Sobrescribe los archivos de la plantilla en un directorio existente")),
        )
        .subcommand(
            Command::new("check")
                .about("Verifica placeholders sin sustituir y archivos críticos")
                .arg(Arg::new("directorio").default_value(".").help("Directorio del proyecto")),
        )
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<ExitCode> {
    let matches = cmd().get_matches();
    match matches.subcommand() {
        Some(("new", m)) => {
            let destino = PathBuf::from(m.get_one::<String>("destino").unwrap());
            let kind = tipo_desde_str(m.get_one::<String>("plantilla").unwrap())?;
            let meta = Meta {
                titulo: m.get_one::<String>("titulo").cloned(),
                autor: m.get_one::<String>("autor").cloned(),
                asesor: m.get_one::<String>("asesor").cloned(),
                anio: m.get_one::<String>("anio").cloned(),
            };
            validar_meta(&meta)?;
            copiar_plantilla(&kind, &destino, &meta, m.get_flag("force"))?;
            println!("✔ Proyecto creado en {}", destino.display());
            match kind {
                TemplateKind::Typst => {
                    println!("  siguiente paso: edite {}/tesis.typ (bloque `meta`) y ejecute:", destino.display());
                    println!("    cd {} && ./compilar.sh", destino.display());
                }
                TemplateKind::Latex => {
                    println!("  siguiente paso: edite {}/tesis.tex (bloque \\newcommand de metadatos) y ejecute:", destino.display());
                    println!("    cd {} && ./compilar.sh", destino.display());
                }
            }
            Ok(ExitCode::SUCCESS)
        }
        Some(("check", m)) => {
            let dir = PathBuf::from(m.get_one::<String>("directorio").unwrap());
            let reporte = ejecutar_check(&dir);
            if reporte.problemas.is_empty() {
                println!("✔ sin placeholders sin sustituir y archivos críticos presentes");
            } else {
                for p in &reporte.problemas {
                    eprintln!("✗ {p}");
                }
            }
            for n in &reporte.notas {
                eprintln!("· nota: {n}");
            }
            Ok(if reporte.problemas.is_empty() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            })
        }
        _ => unreachable!("subcommand_required garantiza uno de los anteriores"),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    fn meta() -> Meta {
        Meta {
            titulo: Some("Mi tesis de prueba".into()),
            autor: Some("María Prueba".into()),
            asesor: Some("Dra. Asesora".into()),
            anio: Some("2027".into()),
        }
    }

    #[test]
    fn typst_sustituye_solo_claves_dadas() {
        let src = "#let meta = (\n  titulo: \"Viejo\",\n  autor: \"Nombre del Autor\",\n  asesor: \"Dr. Nombre del Asesor\",\n  anio: 2026,\n)\n";
        let out = aplicar_meta_typst(src, &meta());
        assert!(out.contains("titulo: \"Mi tesis de prueba\""));
        assert!(out.contains("autor: \"María Prueba\""));
        assert!(out.contains("asesor: \"Dra. Asesora\""));
        assert!(out.contains("anio: 2027"));
    }

    #[test]
    fn typst_no_toca_lineas_sin_clave() {
        let src = "texto \r\n  autor: \"Nombre del Autor\",\notro autor distinto\n";
        let out = aplicar_meta_typst(src, &Meta { autor: Some("Nuevo".into()), ..Meta::default() });
        assert_eq!(out.lines().count(), 3);
        assert!(out.contains("autor: \"Nuevo\""));
        assert!(out.contains("otro autor distinto"));
    }

    #[test]
    fn latex_sustituye_claves() {
        let src = "\\newcommand{\\doctitle}{Título de la Tesis Doctoral}\n\\newcommand{\\docauthor}{Nombre del Autor}\n\\newcommand{\\docadvisor}{Dr. Nombre del Asesor}\n\\newcommand{\\docyear}{2026}\n";
        let out = aplicar_meta_latex(src, &meta());
        assert!(out.contains("\\newcommand{\\doctitle}{Mi tesis de prueba}"));
        assert!(out.contains("\\newcommand{\\docauthor}{María Prueba}"));
        assert!(out.contains("\\newcommand{\\docadvisor}{Dra. Asesora}"));
        assert!(out.contains("\\newcommand{\\docyear}{2027}"));
    }

    #[test]
    fn check_detecta_placeholders() {
        let fixture = "#let meta = (\n  titulo: \"Viejo\",\n  autor: \"Nombre del Autor\",\n  asesor: \"Dr. Nombre del Asesor\",\n  anio: 2026,\n)\n";
        let hits = scan_lineas_placeholder("tesis.typ", fixture);
        assert_eq!(hits.len(), 2);
        let limpio = aplicar_meta_typst(fixture, &meta());
        assert!(scan_lineas_placeholder("tesis.typ", &limpio).is_empty());
    }

    #[test]
    fn check_en_directorio_temporal() {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("tesis-cli-test-{ts}"));
        fs::create_dir_all(&dir).unwrap();
        // plantilla typst mínima
        fs::write(dir.join("tesis.typ"), "#let meta = (\n  autor: \"Nombre del Autor\",\n)\n").unwrap();
        fs::write(dir.join("plantilla.typ"), "// plantilla").unwrap();
        fs::write(dir.join("referencias.bib"), "@book{x, title={T}}\n").unwrap();
        fs::write(dir.join("logo_uni.png"), b"PNG").unwrap();

        let antes = ejecutar_check(&dir);
        assert!(antes.problemas.iter().any(|p| p.contains("Nombre del Autor")));

        // corregir el placeholder
        fs::write(dir.join("tesis.typ"), "#let meta = (\n  autor: \"María Prueba\",\n)\n").unwrap();
        let despues = ejecutar_check(&dir);
        assert!(despues.problemas.is_empty(), "problemas: {:?}", despues.problemas);

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn check_archivo_critico_faltante() {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("tesis-cli-test-missing-{ts}"));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("tesis.typ"), "autor: \"OK\"").unwrap();
        let reporte = ejecutar_check(&dir);
        assert!(reporte.problemas.iter().any(|p| p.contains("plantilla.typ")));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn validacion_rechaza_anio_no_numerico() {
        let m = Meta { anio: Some("veinte".into()), ..Meta::default() };
        assert!(validar_meta(&m).is_err());
    }

    #[test]
    fn validacion_rechaza_comillas_en_valores() {
        let m = Meta { autor: Some("valor con \"comilla\"".into()), ..Meta::default() };
        assert!(validar_meta(&m).is_err());
    }

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn plantillas_embebidas_coinciden_con_el_repo() {
        // En el crate publicable (cargo package) el repo no existe: se omite.
        let root = repo_root();
        if !root.join("plantilla_tesis_uni_typst").is_dir() {
            eprintln!("skipped: standalone crate (repo not present)");
            return;
        }
        for asset in TYPST_ASSETS {
            let real = root.join("plantilla_tesis_uni_typst").join(asset.rel);
            assert!(real.is_file(), "falta {}", real.display());
            assert_eq!(asset.bytes, fs::read(&real).unwrap().as_slice(),
                       "desync en typst/{} — sincroniza tools/tesis/templates", asset.rel);
        }
        for asset in LATEX_ASSETS {
            let real = root.join("plantilla_tesis_uni_latex").join(asset.rel);
            assert!(real.is_file(), "falta {}", real.display());
            assert_eq!(asset.bytes, fs::read(&real).unwrap().as_slice(),
                       "desync en latex/{} — sincroniza tools/tesis/templates", asset.rel);
        }
    }
}
