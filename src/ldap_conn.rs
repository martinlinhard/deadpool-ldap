use ldap3::result::LdapError as Error;
use ldap3::{Ldap, LdapConnAsync, Scope, SearchEntry};

pub struct LdapConn {
    ldap: Ldap,
}

impl LdapConn {
    /// The result is since the bind can fail
    /// Otherwise, the boolean tells you whether the credentials where correct
    pub async fn login(&mut self, username: &str, password: &str) -> Result<bool, Error> {
        let dn = format!("uid={},ou=users,dc=htl-kaindorf,dc=ac,dc=at", username);
        let res = self.ldap.simple_bind(&dn, password).await?;
        // .success returns a result again, we just want to make sure that it's not an error
        // if it isn't an error, we want to return true
        Ok(res.success().is_ok())
    }

    /// This needs to return a vec to match the signature of the search function
    pub async fn get_user_record(&mut self, user_id: &str) -> Result<Vec<SearchEntry>, Error> {
        let dn = format!("uid={},ou=users,dc=htl-kaindorf,dc=ac,dc=at", user_id);
        let (rs, ..) = self
            .ldap
            .search(&dn, Scope::Subtree, "(sn=*)", vec!["*"])
            .await?
            .success()?;
        Ok(rs
            .into_iter()
            .map(SearchEntry::construct)
            .collect::<Vec<SearchEntry>>())
    }

    pub async fn new(ldap_url: &str) -> Result<Self, Error> {
        let (conn, ldap) = LdapConnAsync::new(ldap_url).await?;
        ldap3::drive!(conn);
        Ok(Self { ldap })
    }
}
