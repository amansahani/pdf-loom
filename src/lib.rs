use lopdf::{dictionary, Document, Object, ObjectId};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Metadata information about a loaded PDF
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PdfInfo {
    pub page_count: u32,
    pub version: String,
    pub is_encrypted: bool,
}

/// Helper function to generate a simple valid test PDF with N pages (useful for testing & creating PDFs)
#[wasm_bindgen]
pub fn create_sample_pdf(page_count: u32, title: &str) -> Result<Vec<u8>, JsValue> {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });

    let mut page_ids = Vec::new();

    for i in 1..=page_count {
        let content = format!(
            "BT /F1 24 Tf 100 700 Td ({}: Page {}) Tj ET",
            title, i
        );
        let content_id = doc.add_object(lopdf::Stream::new(
            dictionary! {},
            content.as_bytes().to_vec(),
        ));

        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "Resources" => dictionary! {
                "Font" => dictionary! {
                    "F1" => font_id,
                },
            },
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
        });

        page_ids.push(page_id);
    }

    let kids_vec: Vec<Object> = page_ids.iter().map(|&id| Object::Reference(id)).collect();
    let pages_obj = dictionary! {
        "Type" => "Pages",
        "Count" => page_count as i64,
        "Kids" => kids_vec,
    };
    doc.set_object(pages_id, Object::Dictionary(pages_obj));

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);

    let mut output = Vec::new();
    doc.save_to(&mut output)
        .map_err(|e| JsValue::from_str(&format!("Failed to save sample PDF: {:?}", e)))?;

    Ok(output)
}

/// Get info on a PDF document
#[wasm_bindgen]
pub fn get_pdf_info(pdf_bytes: &[u8]) -> Result<JsValue, JsValue> {
    let doc = Document::load_mem(pdf_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse PDF: {:?}", e)))?;

    let info = PdfInfo {
        page_count: doc.get_pages().len() as u32,
        version: doc.version.clone(),
        is_encrypted: doc.is_encrypted(),
    };

    serde_wasm_bindgen::to_value(&info)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
}

/// Merge multiple PDF documents passed as a JS array of Uint8Arrays.
#[wasm_bindgen]
pub fn merge_pdfs(pdf_buffers: js_sys::Array) -> Result<Vec<u8>, JsValue> {
    if pdf_buffers.length() == 0 {
        return Err(JsValue::from_str("No PDFs provided for merging."));
    }

    let mut max_id = 1;
    let mut all_pages: Vec<ObjectId> = Vec::new();
    let mut documents_objects = Vec::new();
    let mut merged_doc = Document::with_version("1.5");

    for i in 0..pdf_buffers.length() {
        let uint8_array = js_sys::Uint8Array::new(&pdf_buffers.get(i));
        let bytes = uint8_array.to_vec();

        let mut doc = Document::load_mem(&bytes)
            .map_err(|e| JsValue::from_str(&format!("Error loading PDF index {}: {:?}", i, e)))?;

        doc.renumber_objects_with(max_id);
        max_id = doc.max_id + 1;

        for (_page_num, page_id) in doc.get_pages() {
            all_pages.push(page_id);
        }

        documents_objects.extend(doc.objects);
    }

    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    for (object_id, object) in documents_objects.into_iter() {
        match object.type_name().unwrap_or("") {
            "Catalog" => {
                if catalog_object.is_none() {
                    catalog_object = Some((object_id, object));
                }
            }
            "Pages" => {
                if pages_object.is_none() {
                    pages_object = Some((object_id, object));
                }
            }
            _ => {
                merged_doc.objects.insert(object_id, object);
            }
        }
    }

    let pages_object = pages_object
        .ok_or_else(|| JsValue::from_str("Pages root object not found in merged documents"))?;
    let catalog_object = catalog_object
        .ok_or_else(|| JsValue::from_str("Catalog root object not found in merged documents"))?;

    let mut kids_vec: Vec<Object> = Vec::new();

    for page_id in &all_pages {
        kids_vec.push(Object::Reference(*page_id));

        if let Some(page_obj) = merged_doc.objects.get_mut(page_id) {
            if let Ok(dict) = page_obj.as_dict_mut() {
                dict.set("Parent", Object::Reference(pages_object.0));
            }
        }
    }

    let mut pages_dict = pages_object.1.as_dict().unwrap().clone();
    pages_dict.set("Count", Object::Integer(all_pages.len() as i64));
    pages_dict.set("Kids", Object::Array(kids_vec));

    merged_doc.objects.insert(pages_object.0, Object::Dictionary(pages_dict));
    merged_doc.objects.insert(catalog_object.0, catalog_object.1);
    merged_doc.trailer.set("Root", Object::Reference(catalog_object.0));
    merged_doc.max_id = merged_doc.objects.keys().map(|k| k.0).max().unwrap_or(max_id);

    let mut output = Vec::new();
    merged_doc.save_to(&mut output)
        .map_err(|e| JsValue::from_str(&format!("Failed to save merged PDF: {:?}", e)))?;

    Ok(output)
}

/// Split / Extract selected 1-based page numbers from a PDF document.
#[wasm_bindgen]
pub fn extract_pages(pdf_bytes: &[u8], page_numbers: &[u32]) -> Result<Vec<u8>, JsValue> {
    if page_numbers.is_empty() {
        return Err(JsValue::from_str("No page numbers specified for extraction."));
    }

    let doc = Document::load_mem(pdf_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to load PDF: {:?}", e)))?;

    let total_pages = doc.get_pages();
    let mut pages_to_keep: Vec<ObjectId> = Vec::new();

    for &num in page_numbers {
        if let Some(&page_id) = total_pages.get(&num) {
            pages_to_keep.push(page_id);
        } else {
            return Err(JsValue::from_str(&format!(
                "Page {} is out of bounds (Total: {})",
                num,
                total_pages.len()
            )));
        }
    }

    let mut new_doc = Document::with_version(&doc.version);
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    for (object_id, object) in doc.objects.iter() {
        match object.type_name().unwrap_or("") {
            "Catalog" => {
                if catalog_object.is_none() {
                    catalog_object = Some((*object_id, object.clone()));
                }
            }
            "Pages" => {
                if pages_object.is_none() {
                    pages_object = Some((*object_id, object.clone()));
                }
            }
            _ => {
                new_doc.objects.insert(*object_id, object.clone());
            }
        }
    }

    let pages_object = pages_object
        .ok_or_else(|| JsValue::from_str("Pages root object not found"))?;
    let catalog_object = catalog_object
        .ok_or_else(|| JsValue::from_str("Catalog root object not found"))?;

    let mut kids_vec: Vec<Object> = Vec::new();
    for page_id in &pages_to_keep {
        kids_vec.push(Object::Reference(*page_id));
        if let Some(page_obj) = new_doc.objects.get_mut(page_id) {
            if let Ok(dict) = page_obj.as_dict_mut() {
                dict.set("Parent", Object::Reference(pages_object.0));
            }
        }
    }

    let mut pages_dict = pages_object.1.as_dict().unwrap().clone();
    pages_dict.set("Count", Object::Integer(pages_to_keep.len() as i64));
    pages_dict.set("Kids", Object::Array(kids_vec));

    new_doc.objects.insert(pages_object.0, Object::Dictionary(pages_dict));
    new_doc.objects.insert(catalog_object.0, catalog_object.1);
    new_doc.trailer.set("Root", Object::Reference(catalog_object.0));
    new_doc.max_id = new_doc.objects.keys().map(|k| k.0).max().unwrap_or(0);

    let mut output = Vec::new();
    new_doc.save_to(&mut output)
        .map_err(|e| JsValue::from_str(&format!("Failed to save extracted PDF: {:?}", e)))?;

    Ok(output)
}

/// Rotate specified pages by degrees (90, 180, 270)
#[wasm_bindgen]
pub fn rotate_pages(pdf_bytes: &[u8], page_numbers: &[u32], degrees: i32) -> Result<Vec<u8>, JsValue> {
    let mut doc = Document::load_mem(pdf_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to load PDF: {:?}", e)))?;

    let pages = doc.get_pages();
    for &page_num in page_numbers {
        if let Some(&page_id) = pages.get(&page_num) {
            if let Ok(page_obj) = doc.get_object_mut(page_id) {
                if let Ok(dict) = page_obj.as_dict_mut() {
                    let current_rotation = dict
                        .get(b"Rotate")
                        .and_then(|r| r.as_i64())
                        .unwrap_or(0);
                    let new_rotation = (current_rotation + degrees as i64) % 360;
                    dict.set("Rotate", Object::Integer(new_rotation));
                }
            }
        }
    }

    let mut output = Vec::new();
    doc.save_to(&mut output)
        .map_err(|e| JsValue::from_str(&format!("Failed to save rotated PDF: {:?}", e)))?;

    Ok(output)
}
