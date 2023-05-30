use crate::db::vault::Vault;

pub fn setup_vault(vault_name: String) {
    let v = Vault {
        id: 0,
        name: vault_name,
    };

    let res = v.db_create();

    res.unwrap_or_else(|e| panic!("Failed to setup vault: {}", e));
}

pub fn destroy_vault(vault_name: String) {
    let v = Vault {
        id: 0,
        name: vault_name,
    };

    let res = v.db_delete();

    res.unwrap_or_else(|e| panic!("Failed to destroy vault: {}", e));
}
