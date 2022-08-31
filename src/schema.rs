use chrono::NaiveDate;
use serde::Deserialize;

mod date_format {
    use chrono::{NaiveDate};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CdtrRefInf {
    r#ref: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Strd {
    cdtr_ref_inf: Option<CdtrRefInf>
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RmtInf {
    pub ustrd: Option<String>,
    pub strd: Option<Strd>
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Id {
    pub iban: Option<String>,
    pub othr: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Acct {
    pub id: Id,
    pub ccy: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PartyInfo {
    pub nm: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RltdPties {
    pub dbtr: Option<PartyInfo>,
    pub dbtr_acct: Option<Acct>,
    pub cdtr: Option<PartyInfo>,
    pub cdtr_acct: Option<Acct>     
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TxDtls {
    pub rltd_pties: Option<RltdPties>,
    pub rmt_inf: Option<RmtInf>
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NtryDtls {
    pub tx_dtls: Vec<TxDtls>
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ValDt {
    #[serde(with = "date_format")]
    pub dt: NaiveDate,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum CdtDbtInd {
    DBIT,
    CRDT
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Ntry {
    pub ntry_ref: Option<String>,
    pub amt: f32,
    pub cdt_dbt_ind: CdtDbtInd,
    pub val_dt: ValDt,
    pub ntry_dtls: Vec<NtryDtls>
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Stmt {
    pub acct: Acct,
    #[serde(rename = "Ntry", default)]
    pub ntry: Vec<Ntry>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct BkToCstmrStmt {
    #[serde(rename = "Stmt", default)]
    pub stmt: Vec<Stmt>
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Document {
    pub bk_to_cstmr_stmt: BkToCstmrStmt
}