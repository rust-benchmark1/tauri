use ldap3::{LdapConn, Scope, SearchEntry};

pub async fn execute_ldap_search(search_filter: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut ldap = LdapConn::new("ldap://localhost:389")?;
    
    //SINK
    let (rs, _res) = ldap.search(
        "dc=cwe,dc=com",
        Scope::Subtree,
        &search_filter,
        vec!["cn", "mail"]
    )?.success()?;
    
    let entries: Vec<String> = rs
        .into_iter()
        .map(|entry| SearchEntry::construct(entry).dn)
        .collect();
        
    Ok(entries)
}

pub async fn execute_ldap_delete(dn: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut ldap = LdapConn::new("ldap://localhost:389")?;
    
    //SINK
    ldap.delete(&dn)?.success()?;
    
    Ok(())
}

pub async fn process_ldap_operations(filter: String, dn: String) -> Result<(), Box<dyn std::error::Error>> {
    execute_ldap_search(filter).await?;
    execute_ldap_delete(dn).await?;
    Ok(())
} 