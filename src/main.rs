use rss_rust::{
    core::{args::get_command_line_args, structs::GetManyOptions, traits::CrudAble},
    db::{blog::RssEntry, rss_entity::RssEntity},
    rss::Rss,
};
use thread_safe::ThreadSafe;

#[tokio::main]
async fn main() {
    let raw_connection = rss_rust::db::get_database_connection();
    let connection = ThreadSafe::new(&raw_connection);

    let matches = get_command_line_args().get_matches();
    let mut options = GetManyOptions::new();

    if let Some(matches) = matches.subcommand_matches("add") {
        let url = matches.value_of("url").expect("URL IS REQUIRED");
        let profile = matches.value_of("profile").expect("PROFILE IS REQUIRED");

        let rss_entity = RssEntity {
            connection: Some(connection),
            id: 12,
            profile: profile.to_string(),
            rss_url: url.to_string(),
        };
        let _ = Rss::parse(String::from(&rss_entity.rss_url))
            .await
            .expect("Parsing of the given rss url failed");
        rss_entity.save();
    } else if let Some(matches) = matches.subcommand_matches("search") {
        let page = matches
            .value_of("page")
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        let limit = matches
            .value_of("limit")
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        let regex = matches
            .value_of("regex")
            .unwrap_or("")
            .parse::<String>()
            .unwrap_or("".to_string());

        if limit > 0 {
            options.set_limit(limit);
        }
        if page > 0 {
            options.set_page(page);
        }
        options.query = regex.to_string();

        println!("{:?}", options);
        let rss_entries = RssEntry::get_many(&connection, options);
        println!("{} blogs found\n\n", rss_entries.len());
        for rss_entry in rss_entries {
            println!("{}", rss_entry);
        }
    } else if matches.subcommand_matches("sync").is_some() {
        RssEntity::sync(&connection).await;
    }
}
