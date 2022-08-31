pub mod schema;

use std::{path::Path, fs::File, io::BufReader};
use serde_xml_rs::{from_reader, Error};

/// Returns an XML stream either from a file or a URL.
pub fn import_camt(source: &str) -> Result<schema::Document, Error> {
    let local_path = Path::new(source);

    let file = File::open(local_path).expect("File not found");
    let reader = BufReader::new(file);

    from_reader(Box::new(reader))
}

#[cfg(test)]
mod tests {
     use crate::{import_camt, schema::Ntry};

    #[test]
    fn test() {
        let doc = import_camt("example/example_complex.xml").unwrap();
        println!("{:?}", doc.bk_to_cstmr_stmt.stmt.iter().next().unwrap().ntry.iter().filter(|n| n.ntry_dtls.iter().any(|d| d.tx_dtls.iter().any(|t| t.rltd_pties.is_some() ))).collect::<Vec<&Ntry>>().len());
        println!("{:?}", doc.bk_to_cstmr_stmt.stmt.iter().next().unwrap().ntry.iter().next().unwrap().cdt_dbt_ind);
        assert!(doc.bk_to_cstmr_stmt.stmt.len() == 1);
    }
}
