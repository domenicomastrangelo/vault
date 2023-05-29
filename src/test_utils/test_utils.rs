use std::error::Error;

use crate::db::vault::Vault;

pub fn setup_vault(vault_name: String) -> Result<usize, Box<dyn Error>> {
    let v = Vault {
        id: 0,
        name: vault_name,
    };

    v.db_create()
}

pub fn destroy_vault(vault_name: String) -> Result<usize, Box<dyn Error>> {
    let v = Vault {
        id: 0,
        name: vault_name,
    };

    v.db_delete()
}
