use crate::schema::emails;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Insertable, Queryable, Debug, juniper::GraphQLObject)]
#[table_name = "emails"]
#[graphql(description = "An email of an RCOS user.")]
pub struct Email {
    pub email: String,
    pub user_id: Uuid,
}