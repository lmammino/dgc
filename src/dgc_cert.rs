use crate::{Recovery, Test, Vaccination};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DgcCertName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#fn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gnt: Option<String>,
    pub fnt: String,
}

fn empty_if_null<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DgcCert {
    pub ver: String,
    pub nam: DgcCertName,
    pub dob: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "empty_if_null"
    )]
    pub t: Vec<Test>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "empty_if_null"
    )]
    pub v: Vec<Vaccination>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "empty_if_null"
    )]
    pub r: Vec<Recovery>,
}

impl DgcCert {
    pub fn expand_values(&mut self) {
        self.t.iter_mut().for_each(|t| t.expand_values());
        self.v.iter_mut().for_each(|v| v.expand_values());
        self.r.iter_mut().for_each(|r| r.expand_values());
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;

    #[test]
    fn test_json_serialization() {
        let expected_json = "{\"ver\":\"1.3.0\",\"nam\":{\"gn\":\"ALSTON\",\"fn\":\"BLAKE\",\"gnt\":\"ALSTON\",\"fnt\":\"BLAKE\"},\"dob\":\"1990-01-01\",\"t\":[{\"tg\":\"840539006\",\"tt\":\"LP6464-4\",\"sc\":\"2021-10-09T12:03:12Z\",\"tr\":\"260415000\",\"tc\":\"Alhosn One Day Surgery\",\"co\":\"AE\",\"is\":\"Ministry of Health & Prevention\",\"ci\":\"URN:UVCI:V1:AE:8KST0RH057HI8XKW3M8K2NAD06\"}]}";
        let cert = DgcCert {
            ver: String::from("1.3.0"),
            nam: DgcCertName {
                gn: Some(String::from("ALSTON")),
                r#fn: Some(String::from("BLAKE")),
                gnt: Some(String::from("ALSTON")),
                fnt: String::from("BLAKE"),
            },
            dob: String::from("1990-01-01"),
            t: vec![Test {
                tg: Cow::from("840539006"),
                tt: Cow::from("LP6464-4"),
                sc: String::from("2021-10-09T12:03:12Z"),
                tr: Cow::from("260415000"),
                tc: Some(String::from("Alhosn One Day Surgery")),
                co: Cow::from("AE"),
                is: Cow::from("Ministry of Health & Prevention"),
                ci: String::from("URN:UVCI:V1:AE:8KST0RH057HI8XKW3M8K2NAD06"),
                nm: None,
                ma: None,
                dr: None,
            }],
            v: vec![],
            r: vec![],
        };

        let serialized = serde_json::to_string(&cert).unwrap();

        assert_eq!(expected_json, serialized);
    }

    #[test]
    fn test_json_deserialization() {
        let json_data = r#"{
            "ver": "1.0.0",
            "nam": {
              "fn": "Di Caprio",
              "fnt": "DI<CAPRIO",
              "gn": "Marilù Teresa",
              "gnt": "MARILU<TERESA"
            },
            "dob": "1977-06-16",
            "t": [
              {
                "tg": "840539006",
                "tt": "LP6464-4",
                "nm": "Roche LightCycler qPCR",
                "ma": "1232",
                "sc": "2021-05-03T10:27:15Z",
                "dr": "2021-05-11T12:27:15Z",
                "tr": "260415000",
                "tc": "Policlinico Umberto I",
                "co": "IT",
                "is": "IT",
                "ci": "01IT053059F7676042D9BEE9F874C4901F9B#3"
              }
            ]
          }
"#;
        let cert: DgcCert = serde_json::from_str(json_data).unwrap();
        assert_eq!(cert.ver, String::from("1.0.0"));
        assert_eq!(cert.nam.r#fn, Some(String::from("Di Caprio")));
        assert_eq!(cert.nam.fnt, String::from("DI<CAPRIO"));
        assert_eq!(cert.nam.gn, Some(String::from("Marilù Teresa")));
        assert_eq!(cert.nam.gnt, Some(String::from("MARILU<TERESA")));
        assert_eq!(cert.dob, String::from("1977-06-16"));
        assert_eq!(cert.t[0].tg, String::from("840539006"));
        assert_eq!(cert.t[0].tt, String::from("LP6464-4"));
        assert_eq!(cert.t[0].nm, Some(String::from("Roche LightCycler qPCR")));
        assert_eq!(cert.t[0].ma, Some(Cow::from("1232")));
        assert_eq!(cert.t[0].sc, String::from("2021-05-03T10:27:15Z"));
        assert_eq!(cert.t[0].dr, Some(String::from("2021-05-11T12:27:15Z")));
        assert_eq!(cert.t[0].tr, String::from("260415000"));
        assert_eq!(cert.t[0].tc, Some(String::from("Policlinico Umberto I")));
        assert_eq!(cert.t[0].co, String::from("IT"));
        assert_eq!(cert.t[0].is, String::from("IT"));
        assert_eq!(
            cert.t[0].ci,
            String::from("01IT053059F7676042D9BEE9F874C4901F9B#3")
        );
    }

    #[test]
    fn test_json_deserialization_and_expansion() {
        let json_data = r#"{
            "ver": "1.0.0",
            "nam": {
              "fn": "Di Caprio",
              "fnt": "DI<CAPRIO",
              "gn": "Marilù Teresa",
              "gnt": "MARILU<TERESA"
            },
            "dob": "1977-06-16",
            "t": [
              {
                "tg": "840539006",
                "tt": "LP6464-4",
                "nm": "Roche LightCycler qPCR",
                "ma": "1232",
                "sc": "2021-05-03T10:27:15Z",
                "dr": "2021-05-11T12:27:15Z",
                "tr": "260415000",
                "tc": "Policlinico Umberto I",
                "co": "IT",
                "is": "IT",
                "ci": "01IT053059F7676042D9BEE9F874C4901F9B#3"
              }
            ]
          }
"#;
        let mut cert: DgcCert = serde_json::from_str(json_data).unwrap();
        cert.expand_values();
        assert_eq!(cert.ver, String::from("1.0.0"));
        assert_eq!(cert.nam.r#fn, Some(String::from("Di Caprio")));
        assert_eq!(cert.nam.fnt, String::from("DI<CAPRIO"));
        assert_eq!(cert.nam.gn, Some(String::from("Marilù Teresa")));
        assert_eq!(cert.nam.gnt, Some(String::from("MARILU<TERESA")));
        assert_eq!(cert.dob, String::from("1977-06-16"));
        assert_eq!(cert.t[0].tg, String::from("COVID-19"));
        assert_eq!(
            cert.t[0].tt,
            String::from("Nucleic acid amplification with probe detection")
        );
        assert_eq!(cert.t[0].nm, Some(String::from("Roche LightCycler qPCR")));
        assert_eq!(
            cert.t[0].ma,
            Some(Cow::from(
                "Abbott Rapid Diagnostics, Panbio COVID-19 Ag Rapid Test"
            ))
        );
        assert_eq!(cert.t[0].sc, String::from("2021-05-03T10:27:15Z"));
        assert_eq!(cert.t[0].dr, Some(String::from("2021-05-11T12:27:15Z")));
        assert_eq!(cert.t[0].tr, String::from("Not detected"));
        assert_eq!(cert.t[0].tc, Some(String::from("Policlinico Umberto I")));
        assert_eq!(cert.t[0].co, String::from("Italy"));
        assert_eq!(cert.t[0].is, String::from("Italy"));
        assert_eq!(
            cert.t[0].ci,
            String::from("01IT053059F7676042D9BEE9F874C4901F9B#3")
        );
    }
}
