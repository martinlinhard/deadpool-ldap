use async_trait::async_trait;
use crate::ldap_conn::LdapConn;
use ldap3::result::LdapError as Error;

pub type Pool = deadpool::managed::Pool<LdapConn, Error>;
pub struct Manager {}

#[async_trait]
impl deadpool::managed::Manager<LdapConn, Error> for Manager {
    async fn create(&self) -> Result<LdapConn, Error> {
        LdapConn::new("ldap://ldap.htl-kaindorf.at:389").await
    }
    async fn recycle(&self, _conn: &mut LdapConn) -> deadpool::managed::RecycleResult<Error> {
        Ok(())
    }
}
