// Copyright (C) 2026 Slicer Project Generators contributors
// AGPL-3.0-only. Newly authored 2026-10-08; source-informed 3MF facts are
// documented in provenance/bambu-studio/source-influence/protocol-v1.md.
use std::{
    collections::{HashMap, HashSet},
    io::{Cursor, Read},
};

use anyhow::{Context, Result, ensure};
use roxmltree::{Document, Node, ParsingOptions};
use zip::{CompressionMethod, ZipArchive};

use crate::IndexedTriangleMesh;

pub const CORE: &str = "http://schemas.microsoft.com/3dmanufacturing/core/2015/02";
pub const MAX_RAW_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_DECODED_BYTES: usize = 32 * 1024 * 1024;
pub const MAX_VERTICES: usize = 100_000;
pub const MAX_TRIANGLES: usize = 200_000;
pub const IDENTITY: [f64; 16] = [
    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
];

#[derive(Debug, thiserror::Error)]
#[error("unsupported Geometry 3MF profile")]
pub struct UnsupportedGeometry;

fn supported(condition: bool) -> Result<()> {
    if !condition {
        return Err(UnsupportedGeometry.into());
    }
    Ok(())
}

pub fn xml(bytes: &[u8]) -> Result<Document<'_>> {
    let doc = Document::parse_with_options(
        std::str::from_utf8(bytes)?,
        ParsingOptions {
            allow_dtd: false,
            nodes_limit: 1_000_000,
            ..ParsingOptions::default()
        },
    )?;
    for node in doc.descendants() {
        ensure!(node.ancestors().take(65).count() <= 64, "XML depth limit");
    }
    Ok(doc)
}

pub fn archive(bytes: &[u8]) -> Result<HashMap<String, Vec<u8>>> {
    ensure!(bytes.len() <= MAX_DECODED_BYTES, "archive byte limit");
    // Bound the central-directory allocation before handing the archive to zip.
    let eocd = bytes
        .windows(4)
        .rposition(|b| b == b"PK\x05\x06")
        .context("missing ZIP directory")?;
    let tail = bytes.get(eocd..).context("invalid ZIP directory")?;
    ensure!(tail.len() >= 22, "truncated ZIP directory");
    let word = |offset| u16::from_le_bytes([tail[offset], tail[offset + 1]]);
    ensure!(
        word(4) == 0 && word(6) == 0 && word(8) == word(10) && word(10) <= 32,
        "unsupported ZIP directory"
    );
    ensure!(
        tail.len() == 22 + usize::from(word(20)),
        "ZIP trailing data"
    );
    ensure!(
        eocd < 20 || &bytes[eocd - 20..eocd - 16] != b"PK\x06\x07",
        "ZIP64 unsupported"
    );
    // zip retries earlier EOCDs after malformed directories. Exclude embedded
    // records so no unbounded fallback directory can be allocated.
    supported(!bytes[..eocd].windows(4).any(|b| b == b"PK\x05\x06"))?;
    let dword = |offset| u32::from_le_bytes(tail[offset..offset + 4].try_into().unwrap()) as usize;
    ensure!(
        dword(16).checked_add(dword(12)) == Some(eocd),
        "invalid ZIP directory bounds"
    );
    let mut zip = ZipArchive::new(Cursor::new(bytes))?;
    ensure!(
        zip.len() == usize::from(word(10)),
        "inconsistent ZIP directory"
    );
    let mut files = HashMap::new();
    let mut names = HashSet::new();
    let mut decoded = 0usize;
    for index in 0..zip.len() {
        let file = zip.by_index(index)?;
        let name = file.name().to_owned();
        ensure!(names.insert(name.clone()), "duplicate ZIP member");
        ensure!(
            !name.starts_with('/') && !name.contains('\\') && name.is_ascii() && name.len() <= 1024,
            "unsafe ZIP member"
        );
        ensure!(
            name.trim_end_matches('/')
                .split('/')
                .all(|s| !s.is_empty() && s != "." && s != ".."),
            "unsafe ZIP member"
        );
        ensure!(
            file.unix_mode().is_none_or(|mode| mode & 0o170000 == 0
                || mode & 0o170000 == if file.is_dir() { 0o040000 } else { 0o100000 }),
            "nonregular ZIP member"
        );
        ensure!(
            !file.encrypted()
                && matches!(
                    file.compression(),
                    CompressionMethod::Stored | CompressionMethod::Deflated
                ),
            "unsupported ZIP encoding"
        );
        let size = usize::try_from(file.size())?;
        decoded = decoded.checked_add(size).context("ZIP size overflow")?;
        ensure!(decoded <= MAX_DECODED_BYTES, "ZIP decoded byte limit");
        let mut contents = Vec::new();
        file.take((size + 1) as u64).read_to_end(&mut contents)?;
        ensure!(contents.len() == size, "ZIP member size mismatch");
        if name.ends_with('/') {
            ensure!(contents.is_empty(), "nonempty ZIP directory");
        } else {
            files.insert(name, contents);
        }
    }
    Ok(files)
}

pub fn children<'a, 'input>(
    node: Node<'a, 'input>,
    names: &[&str],
    namespace: &str,
) -> Result<Vec<Node<'a, 'input>>> {
    let mut result = Vec::new();
    for child in node.children() {
        if child.is_element() {
            supported(
                child.tag_name().namespace() == Some(namespace)
                    && names.contains(&child.tag_name().name()),
            )?;
            result.push(child);
        } else if child.is_text() {
            ensure!(
                child.text().unwrap_or("").trim().is_empty(),
                "unexpected XML text"
            );
        }
    }
    Ok(result)
}

pub fn attrs(node: Node<'_, '_>, names: &[&str]) -> Result<()> {
    for attr in node.attributes() {
        if attr.namespace() == Some("http://www.w3.org/XML/1998/namespace") && attr.name() == "lang"
        {
            continue;
        }
        supported(attr.namespace().is_none() && names.contains(&attr.name()))?;
    }
    Ok(())
}

pub fn required<'a>(node: Node<'a, '_>, name: &str) -> Result<&'a str> {
    node.attribute(name).context("missing XML attribute")
}

pub fn transform(value: Option<&str>) -> Result<[f64; 16]> {
    let Some(value) = value else {
        return Ok(IDENTITY);
    };
    let numbers = value
        .split_whitespace()
        .map(str::parse::<f64>)
        .collect::<std::result::Result<Vec<_>, _>>()?;
    ensure!(
        numbers.len() == 12 && numbers.iter().all(|v| v.is_finite()),
        "invalid 3MF transform"
    );
    let mut matrix = IDENTITY;
    for column in 0..4 {
        for row in 0..3 {
            matrix[row * 4 + column] = numbers[column * 3 + row];
        }
    }
    ensure!(
        determinant(&matrix).is_finite() && determinant(&matrix) != 0.,
        "singular 3MF transform"
    );
    Ok(matrix)
}

pub fn determinant(m: &[f64; 16]) -> f64 {
    m[0] * (m[5] * m[10] - m[6] * m[9]) - m[1] * (m[4] * m[10] - m[6] * m[8])
        + m[2] * (m[4] * m[9] - m[5] * m[8])
}

fn multiply(a: &[f64; 16], b: &[f64; 16]) -> Result<[f64; 16]> {
    let mut result = [0.; 16];
    for row in 0..4 {
        for col in 0..4 {
            for k in 0..4 {
                result[row * 4 + col] += a[row * 4 + k] * b[k * 4 + col];
            }
        }
    }
    ensure!(result.iter().all(|v| v.is_finite()), "transform overflow");
    ensure!(
        determinant(&result).is_finite() && determinant(&result) != 0.,
        "composed transform out of range"
    );
    Ok(result)
}

pub fn apply(m: &[f64; 16], v: [f64; 3]) -> Result<[f64; 3]> {
    let result = std::array::from_fn(|r| {
        m[r * 4] * v[0] + m[r * 4 + 1] * v[1] + m[r * 4 + 2] * v[2] + m[r * 4 + 3]
    });
    ensure!(result.iter().all(|v| v.is_finite()), "coordinate overflow");
    Ok(result)
}

pub fn parse_mesh(node: Node<'_, '_>) -> Result<IndexedTriangleMesh> {
    attrs(node, &[])?;
    let lists = children(node, &["vertices", "triangles"], CORE)?;
    ensure!(
        lists.len() == 2
            && lists[0].tag_name().name() == "vertices"
            && lists[1].tag_name().name() == "triangles",
        "invalid mesh structure"
    );
    let mut mesh = IndexedTriangleMesh {
        vertices: Vec::new(),
        triangles: Vec::new(),
    };
    attrs(lists[0], &[])?;
    attrs(lists[1], &[])?;
    for v in children(lists[0], &["vertex"], CORE)? {
        attrs(v, &["x", "y", "z"])?;
        ensure!(children(v, &[], CORE)?.is_empty(), "vertex children");
        let point = [
            required(v, "x")?.parse::<f64>()?,
            required(v, "y")?.parse::<f64>()?,
            required(v, "z")?.parse::<f64>()?,
        ];
        ensure!(point.iter().all(|v| v.is_finite()), "nonfinite vertex");
        mesh.vertices.push(point);
        ensure!(mesh.vertices.len() <= MAX_VERTICES, "vertex limit");
    }
    for t in children(lists[1], &["triangle"], CORE)? {
        attrs(t, &["v1", "v2", "v3"])?;
        ensure!(children(t, &[], CORE)?.is_empty(), "triangle children");
        let triangle = [
            required(t, "v1")?.parse::<usize>()?,
            required(t, "v2")?.parse::<usize>()?,
            required(t, "v3")?.parse::<usize>()?,
        ];
        ensure!(
            triangle.iter().all(|&v| v < mesh.vertices.len()),
            "triangle index out of bounds"
        );
        mesh.triangles.push(triangle);
        ensure!(mesh.triangles.len() <= MAX_TRIANGLES, "triangle limit");
    }
    ensure!(
        !mesh.vertices.is_empty() && !mesh.triangles.is_empty(),
        "empty mesh"
    );
    Ok(mesh)
}

enum Resource {
    Mesh(IndexedTriangleMesh),
    Components(Vec<(u32, [f64; 16])>),
}

pub fn realize(bytes: &[u8], placement: &[f64; 16]) -> Result<IndexedTriangleMesh> {
    let mut budget = MAX_DECODED_BYTES;
    realize_with_budget(bytes, placement, &mut budget)
}

pub fn realize_with_budget(
    bytes: &[u8],
    placement: &[f64; 16],
    decoded_budget: &mut usize,
) -> Result<IndexedTriangleMesh> {
    ensure!(bytes.len() <= MAX_RAW_BYTES, "raw input byte limit");
    let files = archive(bytes)?;
    let decoded: usize = files.values().map(Vec::len).sum();
    *decoded_budget = decoded_budget
        .checked_sub(decoded)
        .context("invocation decoded byte limit")?;
    let path = primary_model(&files)?;
    supported(files.len() == 3)?;
    let doc = xml(files.get(&path).context("missing primary model")?)?;
    realize_model(&doc, placement)
}

pub fn primary_model(files: &HashMap<String, Vec<u8>>) -> Result<String> {
    let rels = xml(files.get("_rels/.rels").context("missing relationships")?)?;
    let root = rels.root_element();
    ensure!(
        root.tag_name().name() == "Relationships"
            && root.tag_name().namespace()
                == Some("http://schemas.openxmlformats.org/package/2006/relationships"),
        "invalid relationships"
    );
    attrs(root, &[])?;
    let relations = children(
        root,
        &["Relationship"],
        "http://schemas.openxmlformats.org/package/2006/relationships",
    )?;
    supported(relations.len() == 1)?;
    let relation = relations[0];
    attrs(relation, &["Target", "Id", "Type"])?;
    supported(
        required(relation, "Type")?
            == "http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel",
    )?;
    ensure!(
        !required(relation, "Id")?.is_empty()
            && children(
                relation,
                &[],
                "http://schemas.openxmlformats.org/package/2006/relationships"
            )?
            .is_empty(),
        "invalid relationship"
    );
    let path = required(relation, "Target")?
        .strip_prefix('/')
        .unwrap_or(required(relation, "Target")?);
    supported(path.ends_with(".model"))?;
    ensure!(files.contains_key(path), "missing primary model");
    validate_content_types(
        files
            .get("[Content_Types].xml")
            .context("missing content types")?,
        path,
    )?;
    Ok(path.into())
}

fn realize_model(doc: &Document<'_>, placement: &[f64; 16]) -> Result<IndexedTriangleMesh> {
    let root = doc.root_element();
    ensure!(
        root.tag_name().namespace() == Some(CORE) && root.tag_name().name() == "model",
        "invalid model root"
    );
    attrs(root, &["unit", "requiredextensions"])?;
    supported(
        root.attribute("requiredextensions")
            .is_none_or(str::is_empty),
    )?;
    let meters = match root.attribute("unit").unwrap_or("millimeter") {
        "micron" => 0.000001,
        "millimeter" => 0.001,
        "centimeter" => 0.01,
        "inch" => 0.0254,
        "foot" => 0.3048,
        "meter" => 1.,
        _ => return Err(UnsupportedGeometry.into()),
    };
    let sections = children(root, &["resources", "build"], CORE)?;
    ensure!(
        sections.len() == 2
            && sections[0].tag_name().name() == "resources"
            && sections[1].tag_name().name() == "build",
        "invalid model structure"
    );
    attrs(sections[0], &[])?;
    attrs(sections[1], &[])?;
    let mut resources = HashMap::new();
    let mut stored_vertices = 0;
    let mut stored_triangles = 0;
    for object in children(sections[0], &["object"], CORE)? {
        attrs(object, &["id", "type", "name", "partnumber"])?;
        supported(object.attribute("type").is_none_or(|t| t == "model"))?;
        let id = required(object, "id")?.parse::<u32>()?;
        ensure!(
            id > 0 && resources.len() < 256,
            "invalid resource ID or count"
        );
        let content = children(object, &["mesh", "components"], CORE)?;
        ensure!(content.len() == 1, "invalid object structure");
        let resource = if content[0].tag_name().name() == "mesh" {
            let mesh = parse_mesh(content[0])?;
            stored_vertices += mesh.vertices.len();
            stored_triangles += mesh.triangles.len();
            ensure!(
                stored_vertices <= MAX_VERTICES && stored_triangles <= MAX_TRIANGLES,
                "stored geometry limit"
            );
            Resource::Mesh(mesh)
        } else {
            attrs(content[0], &[])?;
            let mut components = Vec::new();
            for component in children(content[0], &["component"], CORE)? {
                attrs(component, &["objectid", "transform"])?;
                ensure!(
                    children(component, &[], CORE)?.is_empty(),
                    "component children"
                );
                components.push((
                    required(component, "objectid")?.parse()?,
                    transform(component.attribute("transform"))?,
                ));
                ensure!(components.len() <= 256, "component limit");
            }
            ensure!(!components.is_empty(), "empty components");
            Resource::Components(components)
        };
        ensure!(
            resources.insert(id, resource).is_none(),
            "duplicate resource ID"
        );
    }
    let mut mesh = IndexedTriangleMesh {
        vertices: Vec::new(),
        triangles: Vec::new(),
    };
    let mut visited = HashSet::new();
    let mut work = 0usize;
    let items = children(sections[1], &["item"], CORE)?;
    ensure!(
        !items.is_empty() && items.len() <= 256,
        "invalid build cardinality"
    );
    for item in items {
        attrs(item, &["objectid", "transform", "printable"])?;
        ensure!(children(item, &[], CORE)?.is_empty(), "build item children");
        supported(
            item.attribute("printable")
                .is_none_or(|p| p == "1" || p == "true"),
        )?;
        expand(
            required(item, "objectid")?.parse()?,
            &transform(item.attribute("transform"))?,
            &resources,
            &mut Vec::new(),
            &mut visited,
            &mut work,
            &mut mesh,
        )?;
    }
    ensure!(
        visited.len() == resources.len(),
        "unreferenced geometry resource"
    );
    ensure!(
        determinant(placement).is_finite() && determinant(placement) != 0.,
        "singular placement"
    );
    for vertex in &mut mesh.vertices {
        *vertex = apply(placement, vertex.map(|v| v * meters))?.map(|v| v * 1000.);
        ensure!(vertex.iter().all(|v| v.is_finite()), "placement overflow");
    }
    if determinant(placement) < 0. {
        for triangle in &mut mesh.triangles {
            triangle.swap(1, 2);
        }
    }
    Ok(mesh)
}

#[allow(clippy::too_many_arguments)]
fn expand(
    id: u32,
    matrix: &[f64; 16],
    resources: &HashMap<u32, Resource>,
    stack: &mut Vec<u32>,
    visited: &mut HashSet<u32>,
    work: &mut usize,
    output: &mut IndexedTriangleMesh,
) -> Result<()> {
    *work += 1;
    ensure!(
        *work <= 4096 && stack.len() < 32 && !stack.contains(&id),
        "component expansion limit or cycle"
    );
    visited.insert(id);
    match resources.get(&id).context("missing resource reference")? {
        Resource::Mesh(mesh) => {
            let offset = output.vertices.len();
            ensure!(
                offset + mesh.vertices.len() <= MAX_VERTICES
                    && output.triangles.len() + mesh.triangles.len() <= MAX_TRIANGLES,
                "expanded geometry limit"
            );
            for vertex in &mesh.vertices {
                output.vertices.push(apply(matrix, *vertex)?);
            }
            for triangle in &mesh.triangles {
                let mut t = triangle.map(|v| v + offset);
                if determinant(matrix) < 0. {
                    t.swap(1, 2);
                }
                output.triangles.push(t);
            }
        }
        Resource::Components(components) => {
            stack.push(id);
            for (child, transform) in components {
                expand(
                    *child,
                    &multiply(matrix, transform)?,
                    resources,
                    stack,
                    visited,
                    work,
                    output,
                )?;
            }
            stack.pop();
        }
    }
    Ok(())
}

fn validate_content_types(bytes: &[u8], model: &str) -> Result<()> {
    const NS: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
    let doc = xml(bytes)?;
    let root = doc.root_element();
    ensure!(
        root.tag_name().namespace() == Some(NS) && root.tag_name().name() == "Types",
        "invalid content types"
    );
    attrs(root, &[])?;
    let mut types = HashMap::new();
    for node in children(root, &["Default", "Override"], NS)? {
        attrs(
            node,
            if node.tag_name().name() == "Default" {
                &["Extension", "ContentType"]
            } else {
                &["PartName", "ContentType"]
            },
        )?;
        children(node, &[], NS)?;
        let key = if node.tag_name().name() == "Default" {
            format!("ext:{}", required(node, "Extension")?)
        } else {
            format!("part:{}", required(node, "PartName")?)
        };
        ensure!(
            types.insert(key, required(node, "ContentType")?).is_none(),
            "duplicate content type"
        );
    }
    let model_type = types
        .get(&format!("part:/{model}"))
        .or_else(|| types.get("ext:model"));
    ensure!(
        model_type == Some(&"application/vnd.ms-package.3dmanufacturing-3dmodel+xml")
            && types.get("ext:rels")
                == Some(&"application/vnd.openxmlformats-package.relationships+xml"),
        "missing content type"
    );
    Ok(())
}
