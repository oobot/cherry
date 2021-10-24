#[macro_use]
extern crate cherry_derive;

use std::error::Error;

use cherry::{Arguments, MySqlTemplate, pools};

use crate::datasource::{Primary, Secondary};
use crate::model::User;

mod model;
mod datasource;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = toml::from_str(include_str!("../db.toml"))?;
    pools::setup_pools(config).await?;

    let jack = User { id: 100, name: "Jack".to_owned(), };
    let lily = User { id: 101, name: "Lily".to_owned(), };

    let _eff_rows = Primary.insert(&jack).await?;

    // let mut args = MySqlArguments::new();
    // args.add(jack.id);
    let _result = Primary.select::<User>("id", Arguments::from(jack.id)).await?;

    let data = [jack, lily];
    let mut tx = Secondary.begin().await?;
    // If no transaction is specified, it is automatically enabled internally.
    Secondary.insert_replace(&data, Some(&mut tx)).await?;
    tx.commit().await?;

    todo!()
}
