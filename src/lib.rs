use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use pdf_ing::*;
use pyo3::{exceptions::{PyRuntimeError, PyValueError}, prelude::*};

#[derive(FromPyObject, Clone)]
pub enum PyPdfBlock {
    Image {
        path: String,
        dpi: f32,
        pos: (f32, f32),
    },
    TextSection {
        nodes: Vec<PyTextNode>,
        pos: (f32, f32),
    },
}

impl From<PyPdfBlock> for PdfBlock<String> {
    fn from(val: PyPdfBlock) -> Self {
        match val {
            PyPdfBlock::Image { path, dpi, pos } => PdfBlock::Image { path, dpi, pos },
            PyPdfBlock::TextSection { nodes, pos } => PdfBlock::TextSection {
                nodes: nodes.iter().map(|n| TextNode {
                    content: n.content.clone(),
                    font_size: n.font_size,
                    line_height: n.line_height,
                }).collect(),
                pos,
            },
        }
    }
}

#[derive(FromPyObject, Clone)]
pub struct PyTextNode {
    pub content: String,
    pub font_size: f32,
    pub line_height: f32,
}

#[pyfunction]
pub fn gen_pdf(path: &str, width: f32, height: f32, blocks: Vec<PyPdfBlock>) -> PyResult<()> {
    let doc = Doc::new(width, height);
    let blocks: Vec<PdfBlock<String>> = blocks.iter().map(|b| PdfBlock::from(b.clone())).collect();

    generate_pdf(doc, blocks, path).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pydfing(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gen_pdf, m)?)?;
    Ok(())
}

