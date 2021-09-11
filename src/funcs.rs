use crate::deserialize::*;
use crate::errors::*;

use std::collections::HashMap;
use std::path::Path;

fn sort_elements(par_elm: &xml::Element) -> xml::Element {
    let mut elems = HashMap::<&str, Vec<xml::Xml>>::new();
    for c in &par_elm.children {
        if let xml::Xml::ElementNode(ref xml_elm) = *c {
            let sorted_xml_elm = sort_elements(&xml_elm);
            elems
                .entry(&xml_elm.name)
                .or_insert_with(Vec::new)
                .push(xml::Xml::ElementNode(sorted_xml_elm));
        } else {
            elems
                .entry("__value")
                .or_insert_with(Vec::new)
                .push(c.clone());
        }
    }
    let mut new_par_elm = par_elm.clone();
    new_par_elm.children = elems
        .into_iter()
        .flat_map(|(_s, v)| v.into_iter())
        .collect::<Vec<xml::Xml>>();
    new_par_elm
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<Model>> {
    read_from_string(&std::fs::read_to_string(path)?)
}

pub fn read_from_string(string: &str) -> Result<Vec<Model>> {
    let e: xml::Element = string.parse().map_err(SdfError::new)?;
    let mut models = Vec::new();
    for c in &e.children {
        if let xml::Xml::ElementNode(ref xml_elm) = *c {
            if xml_elm.name == "model" {
                let src_xml: xml::Element =
                    format!("{}", xml_elm).parse().map_err(SdfError::new)?;
                let sorted_xml = sort_elements(&src_xml);
                let model =
                    serde_xml_rs::from_str(&format!("{}", sorted_xml)).map_err(SdfError::new)?;
                models.push(model);
            }
        };
    }
    Ok(models)
}
