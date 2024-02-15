use dfinity::candid::{CandidType, Deserialize, Encode};
use dfinity::principal::Principal;

#[derive(CandidType, Deserialize)]
struct Veri {
    deger: String,
    ek_deger: String,
}

impl Veri {
    fn new(deger: String, ek_deger: String) -> Self {
        Self {
            deger,
            ek_deger,
        }
    }

    fn get_deger(&self) -> &str {
        &self.deger
    }

    fn set_deger(&mut self, deger: String) {
        self.deger = deger;
    }

    fn get_ek_deger(&self) -> &str {
        &self.ek_deger
    }

    fn set_ek_deger(&mut self, ek_deger: String) {
        self.ek_deger = ek_deger;
    }
}

fn main() {
    let mut contract_data = Veri::new("Initial Value".to_string(), "Initial Extra Value".to_string());

    println!("Current Value: {}", contract_data.get_deger());
    println!("Current Extra Value: {}", contract_data.get_ek_deger());

    contract_data.set_deger("New Value".to_string());
    contract_data.set_ek_deger("New Extra Value".to_string());

    println!("Updated Value: {}", contract_data.get_deger());
    println!("Updated Extra Value: {}", contract_data.get_ek_deger());
}