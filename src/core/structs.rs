#[derive(Debug)]
pub struct GetManyOptions {
    pub query: String,
    pub page: u64,
    pub limit: u64,
    pub skip: u64,
}

impl GetManyOptions {
    pub fn new() -> GetManyOptions {
        GetManyOptions {
            query: "".to_string(),
            page: 1,
            limit: 10,
            skip: 0,
        }
    }

    pub fn set_page(self: &mut GetManyOptions, page: u64) -> &GetManyOptions {
        self.page = page;
        self.skip = (self.page - 1) * self.limit;
        self
    }

    pub fn set_skip(self: &mut GetManyOptions, skip: u64) -> &GetManyOptions {
        self.skip = skip;
        self
    }

    pub fn set_limit(self: &mut GetManyOptions, limit: u64) -> &GetManyOptions {
        self.limit = limit;
        self
    }

    pub fn as_prepared_tuple(self: &GetManyOptions) -> (String, String, String) {
        (
            format!("%{}%", self.query),
            format!("{}", self.limit),
            format!("{}", self.skip),
        )
    }
}
