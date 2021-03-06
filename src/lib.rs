#![recursion_limit="128"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
//extern crate diesel_infer_schema;
extern crate dotenv;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde;
extern crate sapper;
#[macro_use]
extern crate sapper_std;
extern crate rand;
extern crate tiny_keccak;
extern crate comrak;
extern crate redis;
extern crate r2d2;
extern crate r2d2_redis;
extern crate r2d2_diesel;
extern crate uuid;


pub mod schema;
pub mod models;
pub mod util;
pub mod api;
pub mod web;

pub(crate) use schema::{ articles, users, article_with_tag, tags, article_tag_relation, comments };
pub(crate) use models::{ NewArticle, ArticlesWithTag, ArticleList, ModifyPublish, EditArticle, PublishedStatistics };
pub(crate) use models::{ UserInfo, Users, NewUser, ChangePassword, RegisteredUser, EditUser, LoginUser, ChangePermission, DisabledUser };
pub(crate) use models::{ NewTag, Tags, TagCount };
pub(crate) use models::{ Comments, NewComments, DeleteComment };
pub(crate) use util::{ sha3_256_encode, random_string, markdown_render, get_password,
                       admin_verification_cookie, user_verification_cookie, UserSession, AdminSession };
pub(crate) use util::visitor_log;
pub use util::{ create_redis_pool, RedisPool, Redis };
pub use util::{ create_pg_pool, Postgresql };
pub use api::Visitor;
pub use api::User;
pub use api::AdminArticle;
pub use api::Tag;
pub use api::AdminUser;
pub use api::ChartData;
pub use web::{ ArticleWeb, Admin };
