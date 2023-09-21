use clap::{App, Arg, SubCommand};

pub fn get_command_line_args() -> App<'static, 'static> {
    App::new("Blog Blunter")
    .version("1.1.0")
    .author("Nirjal Paudel <nirjalpaudel@gmail.com>")
    .about("BB is a rust script that helps you parse through the blogs from command line using rust. Rss feed is required")
    .subcommand(
        SubCommand::with_name("add")
            .about("Add an rss feed into the database")
            .arg(
                Arg::with_name("url")
                    .short("u")
                    .long("url")
                    .value_name("URL")
                    .help("URL to insert")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("profile")
                    .short("p")
                    .long("profile")
                    .value_name("PROFILE")
                    .help("Profile to insert as [will be useful later]")
                    .takes_value(true),
            ),
    )
    .subcommand(
        SubCommand::with_name("search")
            .about("Search rss entries by regular expression")
            .arg(
                Arg::with_name("regex")
                    .short("r")
                    .long("regex")
                    .value_name("REGEX")
                    .help("Regex pattern to search the blogs with [but uses LIKE SQL statement]")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("page")
                    .short("p")
                    .long("page")
                    .value_name("PAGE")
                    .help("page number of response, to look")
                    .takes_value(true)
                    .required(false)
            )
            .arg(
                Arg::with_name("limit")
                    .short("l")
                    .long("limit")
                    .value_name("LIMIT")
                    .help("limit number of responses")
                    .takes_value(true)
                    .required(false)
            ),
    )
    .subcommand(SubCommand::with_name("sync").about("Syncs something"))
}
