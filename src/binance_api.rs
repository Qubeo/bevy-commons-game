use bevy::prelude::{Res, ResMut};
use binance::api::*;
use binance::market::*;

#[derive(Default)]
pub struct BinanceMarket {
    market: Option<Market>,
}

#[derive(Default)]
pub struct HotPrice {
    pub last: f64,
    pub actual: f64,
}

pub fn setup_binance(mut binance_market: ResMut<BinanceMarket>) {
    let market: Market = Binance::new(None, None);
    binance_market.market = Some(market);
}

pub fn update_price(hot_price: Res<HotPrice>) {
    //
}

pub fn get_price(
    mut hot_price: ResMut<HotPrice>,
    binance_market: Res<BinanceMarket>, /*, ticker_pair: String */
) {
    // Latest price for ONE symbol

    if let Some(answ) = &binance_market.market {
        let price = answ.get_price("HOTUSDT").unwrap().price;
        hot_price.last = hot_price.actual;
        hot_price.actual = price;
        println!("binance_api::get_price(): price: {:?}", price);
    } else {
        println!("binance_api::get_price(): error");
    }

    /* match answ {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    } */
}

/*
// Order book at default depth
    match market.get_depth("BNBETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Order book at depth 500
    match market.get_custom_depth("BNBETH", 500) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match market.get_all_prices() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
*/
