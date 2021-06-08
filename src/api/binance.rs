use bevy::{math::Vec3, prelude::{Res, ResMut, Transform}};
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

    
        /* let mut rng = rand::thread_rng();
        let start_time = Instant::now();
        let duration = Duration::from_secs_f32(rng.gen_range(0.05..0.2));
        while Instant::now() - start_time < duration {
            // Spinning for 'duration', simulating doing hard
            // compute work generating translation coords!
        }
        // Such hard work, all done!
        Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32)) */
    
    
    // Spawn new entity and add our new task as a component
    // commands.spawn().insert(task);

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
