use std::ascii::AsciiExt;

#[derive(PartialEq,Debug)]
pub struct RibonucleicAcid {
    bases: String
}

#[derive(PartialEq,Debug)]
pub struct DeoxyribonucleicAcid {
    bases: String
}

impl RibonucleicAcid {
    pub fn new(rna: &str) -> RibonucleicAcid {
        RibonucleicAcid { bases: rna.to_string() }
    }
}

impl DeoxyribonucleicAcid {
    pub fn new(dna: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { bases: dna.to_string() }
    }

    fn transcribe(&self, ch: char) -> char {
        match ch.to_ascii_uppercase() {
            'A' => 'U',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _   => ch
        }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let rna: String = self.bases.chars().map(|c| self.transcribe(c)).collect();
        RibonucleicAcid::new(rna.as_str())
    }
}
