use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RmtInf {
    pub ustrd: String
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
pub struct DbtrCdtr {
    pub nm: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RltdPties {
    dbtr: Option<DbtrCdtr>,
    dbtr_acct: Option<Acct>,
    cdtr: Option<DbtrCdtr>,
    cdtr_acct: Option<Acct>     
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
    pub dt: NaiveDateTime,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Ntry {
    pub ntry_ref: Option<String>,
    pub amt: f32,
    pub cdt_dbt_ind: String,
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
pub struct Document {
    pub xmlns: String,
    #[serde(rename = "BkToCstmrStmt")]
    pub bk_to_cstmr_stmt: BkToCstmrStmt
}