use std::collections::HashMap;

lazy_static! {
    /// Populated from https://github.com/ehn-dcc-development/ehn-dcc-schema/tree/release/1.3.0/valuesets
    /// List generated with the following Node.js snippet (for every valueset file):
    /// > for (const [key, val] of Object.entries(fileData.valueSetValues)) {
    /// >
    /// >   console.log(\`m.insert("${key}", "${val.display}");\`)
    /// >
    /// > }
    pub static ref VALUESET: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("840539006", "COVID-19");
        m.insert("308", "PCL Inc, PCL COVID19 Ag Rapid FIA");
        m.insert("344", "SD BIOSENSOR Inc, STANDARD F COVID-19 Ag FIA");
        m.insert("345", "SD BIOSENSOR Inc, STANDARD Q COVID-19 Ag Test");
        m.insert("768", "ArcDia International Ltd, mariPOC SARS-CoV-2");
        m.insert("1097", "Quidel Corporation, Sofia SARS Antigen FIA");
        m.insert("1114", "Sugentech, Inc, SGTi-flex COVID-19 Ag");
        m.insert("1144", "Green Cross Medical Science Corp., GENEDIA W COVID-19 Ag");
        m.insert("1162", "Nal von minden GmbH, NADAL COVID-19 Ag Test");
        m.insert("1173", "CerTest Biotec, CerTest SARS-CoV-2 Card test");
        m.insert("1180", "MEDsan GmbH, MEDsan SARS-CoV-2 Antigen Rapid Test");
        m.insert("1190", "möLab, COVID-19 Rapid Antigen Test");
        m.insert("1199", "Oncosem Onkolojik Sistemler San. ve Tic. A.S., CAT");
        m.insert("1215", "Hangzhou Laihe Biotech Co., Ltd, LYHER Novel Coronavirus (COVID-19) Antigen Test Kit(Colloidal Gold)");
        m.insert("1218", "Siemens Healthineers, CLINITEST Rapid Covid-19 Antigen Test");
        m.insert("1223", "BIOSYNEX S.A., BIOSYNEX COVID-19 Ag BSS");
        m.insert("1225", "DDS DIAGNOSTIC, Test Rapid Covid-19 Antigen (tampon nazofaringian)");
        m.insert("1232", "Abbott Rapid Diagnostics, Panbio COVID-19 Ag Rapid Test");
        m.insert("1236", "BTNX Inc, Rapid Response COVID-19 Antigen Rapid Test");
        m.insert("1244", "GenBody, Inc, Genbody COVID-19 Ag Test");
        m.insert("1246", "VivaChek Biotech (Hangzhou) Co., Ltd, Vivadiag SARS CoV 2 Ag Rapid Test");
        m.insert("1253", "GenSure Biotech Inc, GenSure COVID-19 Antigen Rapid Kit (REF: P2004)");
        m.insert("1256", "Hangzhou AllTest Biotech Co., Ltd, COVID-19 and Influenza A+B Antigen Combo Rapid Test");
        m.insert("1263", "Humasis, Humasis COVID-19 Ag Test");
        m.insert("1266", "Labnovation Technologies Inc, SARS-CoV-2 Antigen Rapid Test Kit");
        m.insert("1267", "LumiQuick Diagnostics Inc, QuickProfile COVID-19 Antigen Test");
        m.insert("1268", "LumiraDX, LumiraDx SARS-CoV-2 Ag Test");
        m.insert("1271", "Precision Biosensor, Inc, Exdia COVID-19 Ag");
        m.insert("1278", "Xiamen Boson Biotech Co. Ltd, Rapid SARS-CoV-2 Antigen Test Card");
        m.insert("1295", "Zhejiang Anji Saianfu Biotech Co., Ltd, reOpenTest COVID-19 Antigen Rapid Test");
        m.insert("1296", "Zhejiang Anji Saianfu Biotech Co., Ltd, AndLucky COVID-19 Antigen Rapid Test");
        m.insert("1304", "AMEDA Labordiagnostik GmbH, AMP Rapid Test SARS-CoV-2 Ag");
        m.insert("1319", "SGA Medikal, V-Chek SARS-CoV-2 Ag Rapid Test Kit (Colloidal Gold)");
        m.insert("1331", "Beijing Lepu Medical Technology Co., Ltd, SARS-CoV-2 Antigen Rapid Test Kit");
        m.insert("1333", "Joinstar Biomedical Technology Co., Ltd, COVID-19 Rapid Antigen Test (Colloidal Gold)");
        m.insert("1341", "Qingdao Hightop Biotech Co., Ltd, SARS-CoV-2 Antigen Rapid Test (Immunochromatography)");
        m.insert("1343", "Zhezhiang Orient Gene Biotech Co., Ltd, Coronavirus Ag Rapid Test Cassette (Swab)");
        m.insert("1360", "Guangdong Wesail Biotech Co., Ltd, COVID-19 Ag Test Kit");
        m.insert("1363", "Hangzhou Clongene Biotech Co., Ltd, Covid-19 Antigen Rapid Test Kit");
        m.insert("1365", "Hangzhou Clongene Biotech Co., Ltd, COVID-19/Influenza A+B Antigen Combo Rapid Test");
        m.insert("1375", "DIALAB GmbH, DIAQUICK COVID-19 Ag Cassette");
        m.insert("1392", "Hangzhou Testsea Biotechnology Co., Ltd, COVID-19 Antigen Test Cassette");
        m.insert("1420", "NanoEntek, FREND COVID-19 Ag");
        m.insert("1437", "Guangzhou Wondfo Biotech Co., Ltd, Wondfo 2019-nCoV Antigen Test (Lateral Flow Method)");
        m.insert("1443", "Vitrosens Biotechnology Co., Ltd, RapidFor SARS-CoV-2 Rapid Ag Test");
        m.insert("1456", "Xiamen Wiz Biotech Co., Ltd, SARS-CoV-2 Antigen Rapid Test");
        m.insert("1466", "TODA PHARMA, TODA CORONADIAG Ag");
        m.insert("1468", "ACON Laboratories, Inc, Flowflex SARS-CoV-2 Antigen rapid test");
        m.insert("1481", "MP Biomedicals, Rapid SARS-CoV-2 Antigen Test Card");
        m.insert("1484", "Beijing Wantai Biological Pharmacy Enterprise Co., Ltd, Wantai SARS-CoV-2 Ag Rapid Test (FIA)");
        m.insert("1489", "Safecare Biotech (Hangzhou) Co. Ltd, COVID-19 Antigen Rapid Test Kit (Swab)");
        m.insert("1490", "Safecare Biotech (Hangzhou) Co. Ltd, Multi-Respiratory Virus Antigen Test Kit(Swab)  (Influenza A+B/ COVID-19)");
        m.insert("1574", "Shenzhen Zhenrui Biotechnology Co., Ltd, Zhenrui ®COVID-19 Antigen Test Cassette");
        m.insert("1604", "Roche (SD BIOSENSOR), SARS-CoV-2 Antigen Rapid Test");
        m.insert("1606", "RapiGEN Inc, BIOCREDIT COVID-19 Ag - SARS-CoV 2 Antigen test");
        m.insert("1654", "Asan Pharmaceutical CO., LTD, Asan Easy Test COVID-19 Ag");
        m.insert("1736", "Anhui Deep Blue Medical Technology Co., Ltd, COVID-19 (SARS-CoV-2) Antigen Test Kit(Colloidal Gold)");
        m.insert("1747", "Guangdong Hecin Scientific, Inc., 2019-nCoV Antigen Test Kit (colloidal gold method)");
        m.insert("1763", "Xiamen AmonMed Biotechnology Co., Ltd, COVID-19 Antigen Rapid Test Kit (Colloidal Gold)");
        m.insert("1764", "JOYSBIO (Tianjin) Biotechnology Co., Ltd, SARS-CoV-2 Antigen Rapid Test Kit (Colloidal Gold)");
        m.insert("1767", "Healgen Scientific, Coronavirus Ag Rapid Test Cassette");
        m.insert("1769", "Shenzhen Watmind Medical Co., Ltd, SARS-CoV-2 Ag Diagnostic Test Kit (Colloidal Gold)");
        m.insert("1815", "Anhui Deep Blue Medical Technology Co., Ltd, COVID-19 (SARS-CoV-2) Antigen Test Kit (Colloidal Gold) - Nasal Swab");
        m.insert("1822", "Anbio (Xiamen) Biotechnology Co., Ltd, Rapid COVID-19 Antigen Test(Colloidal Gold)");
        m.insert("1833", "AAZ-LMB, COVID-VIRO");
        m.insert("1844", "Hangzhou Immuno Biotech Co.,Ltd, Immunobio SARS-CoV-2 Antigen ANTERIOR NASAL Rapid Test Kit (minimal invasive)");
        m.insert("1870", "Beijing Hotgen Biotech Co., Ltd, Novel Coronavirus 2019-nCoV Antigen Test (Colloidal Gold)");
        m.insert("1884", "Xiamen Wiz Biotech Co., Ltd, SARS-CoV-2 Antigen Rapid Test (Colloidal Gold)");
        m.insert("1906", "Azure Biotech Inc, COVID-19 Antigen Rapid Test Device");
        m.insert("1919", "Core Technology Co., Ltd, Coretests COVID-19 Ag Test");
        m.insert("1934", "Tody Laboratories Int., Coronavirus (SARS-CoV 2) Antigen - Oral Fluid");
        m.insert("2010", "Atlas Link Technology Co., Ltd., NOVA Test® SARS-CoV-2 Antigen Rapid Test Kit (Colloidal Gold Immunochromatography)");
        m.insert("2017", "Shenzhen Ultra-Diagnostics Biotec.Co.,Ltd, SARS-CoV-2 Antigen Test Kit");
        m.insert("260373001", "Detected");
        m.insert("260415000", "Not detected");
        m.insert("LP6464-4", "Nucleic acid amplification with probe detection");
        m.insert("LP217198-3", "Rapid immunoassay");
        m.insert("ORG-100001699", "AstraZeneca AB");
        m.insert("ORG-100030215", "Biontech Manufacturing GmbH");
        m.insert("ORG-100001417", "Janssen-Cilag International");
        m.insert("ORG-100031184", "Moderna Biotech Spain S.L.");
        m.insert("ORG-100006270", "Curevac AG");
        m.insert("ORG-100013793", "CanSino Biologics");
        m.insert("ORG-100020693", "China Sinopharm International Corp. - Beijing location");
        m.insert("ORG-100010771", "Sinopharm Weiqida Europe Pharmaceutical s.r.o. - Prague location");
        m.insert("ORG-100024420", "Sinopharm Zhijun (Shenzhen) Pharmaceutical Co. Ltd. - Shenzhen location");
        m.insert("ORG-100032020", "Novavax CZ AS");
        m.insert("Gamaleya-Research-Institute", "Gamaleya Research Institute");
        m.insert("Vector-Institute", "Vector Institute");
        m.insert("Sinovac-Biotech", "Sinovac Biotech");
        m.insert("Bharat-Biotech", "Bharat Biotech");
        m.insert("EU/1/20/1528", "Comirnaty");
        m.insert("EU/1/20/1507", "COVID-19 Vaccine Moderna");
        m.insert("EU/1/21/1529", "Vaxzevria");
        m.insert("EU/1/20/1525", "COVID-19 Vaccine Janssen");
        m.insert("CVnCoV", "CVnCoV");
        m.insert("Sputnik-V", "Sputnik-V");
        m.insert("Convidecia", "Convidecia");
        m.insert("EpiVacCorona", "EpiVacCorona");
        m.insert("BBIBP-CorV", "BBIBP-CorV");
        m.insert("Inactivated-SARS-CoV-2-Vero-Cell", "Inactivated SARS-CoV-2 (Vero Cell)");
        m.insert("CoronaVac", "CoronaVac");
        m.insert("Covaxin", "Covaxin (also known as BBV152 A, B, C)");
        m.insert("1119305005", "SARS-CoV-2 antigen vaccine");
        m.insert("1119349007", "SARS-CoV-2 mRNA vaccine");
        m.insert("J07BX03", "covid-19 vaccines");

        m
    };
}
