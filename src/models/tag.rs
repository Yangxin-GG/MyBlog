use super::super::tags;
use super::super::tags::dsl::tags as all_tags;
use super::Relations;

use diesel;
use diesel::expression::sql;
use diesel::{ ExecuteDsl, ExpressionMethods, FilterDsl, LoadDsl, PgConnection };
use uuid::Uuid;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Tags {
    id: Uuid,
    tag: String
}

impl Tags {
    pub fn view_list_tag(conn: &PgConnection) -> Result<Vec<Tags>, String> {
        let res = all_tags.load::<Tags>(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err))
        }
    }

    pub fn delete_tag(conn: &PgConnection, id: Uuid) -> Result<usize, String> {
        Relations::delete_all(conn, id, "tag");
        let res = diesel::delete(all_tags.filter(tags::id.eq(id)))
        .execute(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err))
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn edit_tag(&self, conn: &PgConnection) -> Result<usize, String> {
        let res = diesel::update(all_tags.filter(tags::id.eq(&self.id)))
            .set(tags::tag.eq(&self.tag))
            .execute(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err))
        }
    }
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct TagCount {
    id: Uuid,
    tag: String,
    count: i64
}

impl TagCount {
    pub fn view_tag_count(conn: &PgConnection) -> Result<Vec<Self>, String> {
        let query = sql::<(diesel::types::Uuid, diesel::types::Text ,diesel::types::BigInt)>
            ("select b.id, b.tag, count(*) from article_tag_relation a join tags b on a.tag_id=b.id group by b.id, b.tag");
        let res = query.load::<Self>(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err))
        }
    }

    pub fn view_all_tag_count(conn: &PgConnection, limit: i64, offset: i64) -> Result<Vec<Self>, String> {
        let raw_sql = format!("select a.id, a.tag, (case when b.count is null then 0 else b.count end) as count from tags a left join \
                (select tag_id, count(*) from article_tag_relation group by tag_id) b on a.id = b.tag_id order by a.id limit {} offset {};", limit, offset);
        let query = sql::<(diesel::types::Uuid, diesel::types::Text ,diesel::types::BigInt)>
            (&raw_sql);
        let res = query.load::<Self>(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err))
        }
    }
}

#[derive(Insertable, Debug, Clone, Deserialize, Serialize)]
#[table_name = "tags"]
pub struct NewTag {
    tag: String
}

impl NewTag {
    pub fn new(tag: &str) -> Self {
        NewTag {
            tag: tag.to_owned()
        }
    }

    pub fn insert(&self, conn: &PgConnection) -> bool {
        diesel::insert(self)
            .into(tags::table)
            .execute(conn)
            .is_ok()
    }

    pub fn insert_with_result(&self, conn: &PgConnection) -> Tags {
        diesel::insert(self)
            .into(tags::table)
            .get_result(conn)
            .unwrap()
    }

    pub fn insert_all(raw_tag: Vec<NewTag>, conn: &PgConnection) -> Vec<Uuid> {
        let new_tags: Vec<Tags> = diesel::insert(&raw_tag).into(tags::table)
            .get_results(conn).unwrap();
        new_tags.iter().map(|tag| tag.get_id()).collect()
    }
}
