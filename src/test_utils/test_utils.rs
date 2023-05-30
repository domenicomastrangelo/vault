#[cfg(test)]
pub fn setup_vault(vault_name: String) {
    let v = crate::db::vault::Vault { name: vault_name };

    let res = v.db_create();

    res.unwrap_or_else(|e| panic!("Failed to setup vault: {}", e));
}

#[cfg(test)]
pub fn destroy_vault(vault_name: String) {
    let v = crate::db::vault::Vault { name: vault_name };

    let res = v.db_delete();

    res.unwrap_or_else(|e| panic!("Failed to destroy vault: {}", e));
}
