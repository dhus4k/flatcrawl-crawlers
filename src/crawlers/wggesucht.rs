extern crate kuchiki;
extern crate reqwest;
extern crate std;

use crawlers::{Crawler, Error};
use kuchiki::{ElementData, NodeDataRef};
use models::FlatData;

pub struct WGGesucht {}

impl Crawler for WGGesucht {

  fn name(&self) -> &'static str {
    "wggesucht"
  }

  fn selector(&self) -> &'static str {
    "tr[adid^=wohnungen]"
  }

  fn transform_result(&self, result: NodeDataRef<ElementData>) -> Result<FlatData, Error> {
    let only_limited = Self::get_text(&result, ".ang_spalte_freibis")?
      .trim()
      .len() > 0;
    if only_limited {
      Err( Error { message: "Flat is only available for a limited time.".to_owned() } )
    } else {
      let rent = Self::get_text(&result, ".ang_spalte_miete")?;
      let squaremeters = Self::get_text(&result, ".ang_spalte_groesse")?;
      let rooms = Self::get_text(&result, ".ang_spalte_zimmer")?;
      let title = "Wohnung auf WG Gesucht".to_owned();
      let address = "München, ".to_owned() +
        Self::get_text(&result, ".ang_spalte_stadt")?
        .replace("\n", "")
        .trim();
      let externalid = Self::get_attr(&result, "adid")?;
    Ok(FlatData {
      rent: Self::parse_number(rent)?,
      squaremeters: Self::parse_number(squaremeters)?,
      address,
      title,
      rooms: Self::parse_number(rooms)?,
      externalid,
    })
    }
  }
}
