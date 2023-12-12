use crate::{
    constants::{
        GROUND_PLATFORM_COST, GROUND_PLATFORM_PROFITABILITY, GROUND_PLATFORM_UPGRADE_COST,
        PUMP_PLATFORM_COST, PUMP_PLATFORM_PROFITABILITY, PUMP_PLATFORM_UPGRADE_COST,
        RIG_PLATFORM_COST, RIG_PLATFORM_PROFITABILITY, RIG_PLATFORM_UPGRADE_COST,
    },
    model::{BeerModel, CreateBeerModel, MoneyTransactionModel, OilPlatformModel, PlatformType},
};
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

pub async fn seed_game_entities(pool: &PgPool) {
    // seed beers table
    let beer_table_epmty = match sqlx::query_as!(BeerModel, "SELECT * FROM beers")
        .fetch_all(pool)
        .await
    {
        Ok(beers) => {
            if beers.len() > 0 {
                false
            } else {
                true
            }
        }
        Err(SqlxError::RowNotFound) => true,
        Err(_) => false,
    };

    println!("Beer table is empty: {}", beer_table_epmty);

    let beers = vec![
        CreateBeerModel {
            title: "Pi*Wasser".to_string(),
            description: "Just do not drink it, please".to_string(),
            thumbnail: "https://4.bp.blogspot.com/_3Pn5KDoXX18/SjePv3avr4I/AAAAAAAAAWw/8dez3djTLp0/s400/DSC00661.JPG".to_string(),
            cost: 100,
        },
        CreateBeerModel {
            title: "Heineken".to_string(),
            description: "Becoming the world's leading premium lager".to_string(),
            thumbnail: "https://www.lulu.lv/cache/images/2557649655/heineken-alus-0-33l-5-0_1819124495.jpg".to_string(),
            cost: 15000,
        },
        CreateBeerModel {
            title: "Carlsberg".to_string(),
            description: "Hundreds of beers at the heart of moments that bring people together"
                .to_string(),
            thumbnail: "https://cdn.webshopapp.com/shops/65337/files/422556506/carlsberg-00.jpg".to_string(),
            cost: 20000,
        },
        CreateBeerModel {
            title: "Cesu Premium".to_string(),
            description: "Crispy, refreshing and well-balanced lager beer born in Cesis!"
                .to_string(),
            thumbnail: "https://veikals.cesualus.lv/cdn/shop/products/Premium_PINT_2020-2_WEB_002_320x.png?v=1619431272".to_string(),
            cost: 30000,
        },
        CreateBeerModel {
            title: "Corona Extra".to_string(),
            description: "Mexican-born brew with a distinct flavor and iconic branding".to_string(),
            thumbnail: "https://booziecarry.lv/wp-content/uploads/2020/12/Alus-Corona-Extra-4-5-0-355l.jpg".to_string(),
            cost: 50000,
        },
        CreateBeerModel {
            title: "Lacplesis".to_string(),
            description: "Experience of many decades of brewing".to_string(),
            thumbnail: "https://alkoutlet.lv/media/catalog/product/cache/937da15ad1ee98697c5954ed139da50b/imp/ort/235157.jpg".to_string(),
            cost: 100000,
        },
        CreateBeerModel {
            title: "San Miguel".to_string(),
            description: "Brewing Friendships, Celebrating Life".to_string(),
            thumbnail: "https://assets-global.website-files.com/63be70c06e09535c2b5300c0/63ea1fb7bb15a65195ff79a0_san_miguel.png".to_string(),
            cost: 200000,
        },
        CreateBeerModel {
            title: "Guiness".to_string(),
            description: "It takes a thirst for adventure to do things the Guinness way"
                .to_string(),
            thumbnail: "https://dydza6t6xitx6.cloudfront.net/ci-guinness-draught-420c95ffc7f4bdc0.jpeg".to_string(),
            cost: 500000,
        },
    ];

    if beer_table_epmty {
        println!("Seeding beer table!");
        for beer in beers {
            match sqlx::query_as!(
                BeerModel,
                "INSERT INTO beers (title, description, thumbnail, cost) VALUES ($1, $2, $3, $4) RETURNING *", 
                beer.title,
                beer.description,
                beer.thumbnail,
                beer.cost,
            )
                .fetch_one(pool)
                .await
            {
                Ok(_) => (),
                Err(_) => (),
            };
        }
    }

    // seed platforms table
    let platform_table_epmty =
        match sqlx::query_as!(OilPlatformModel, "SELECT * FROM oil_platforms")
            .fetch_all(pool)
            .await
        {
            Ok(platforms) => {
                if platforms.len() > 0 {
                    false
                } else {
                    true
                }
            }
            Err(SqlxError::RowNotFound) => true,
            Err(_) => false,
        };

    println!("Platform table is empty: {}", platform_table_epmty);

    if platform_table_epmty {
        println!("Seeding platform table!");
        match sqlx::query_as!(
            OilPlatformModel,
            "INSERT INTO oil_platforms (platform_type, profitability) VALUES ($1, $2) RETURNING *",
            "Rig",
            5,
        )
        .fetch_one(pool)
        .await
        {
            Ok(_) => (),
            Err(_) => (),
        };
    }

    // seed transactions table
    let transactions_table_epmty =
        match sqlx::query_as!(MoneyTransactionModel, "SELECT * FROM money_transactions")
            .fetch_all(pool)
            .await
        {
            Ok(txs) => {
                if txs.len() > 0 {
                    false
                } else {
                    true
                }
            }
            Err(SqlxError::RowNotFound) => true,
            Err(_) => false,
        };

    println!("Transaction table is empty: {}", transactions_table_epmty);

    if transactions_table_epmty {
        println!("Seeding transactions table!");
        match sqlx::query_as!(
            MoneyTransactionModel,
            "INSERT INTO money_transactions (item_id, amount, reduces_balance) VALUES ($1, $2, $3) RETURNING *",
            Uuid::nil(),
            1000,
            false,
        )
        .fetch_one(pool)
        .await
        {
            Ok(_) => (),
            Err(_) => (),
        };
    }
}

// oil platform helpers
pub fn get_platform_cost(platform: PlatformType) -> i64 {
    match platform {
        PlatformType::Rig => RIG_PLATFORM_COST,
        PlatformType::Ground => GROUND_PLATFORM_COST,
        PlatformType::Pump => PUMP_PLATFORM_COST,
    }
}

pub fn get_platform_upgrade_cost(platform: PlatformType) -> i64 {
    match platform {
        PlatformType::Rig => RIG_PLATFORM_UPGRADE_COST,
        PlatformType::Ground => GROUND_PLATFORM_UPGRADE_COST,
        PlatformType::Pump => PUMP_PLATFORM_UPGRADE_COST,
    }
}

pub fn get_platform_profitability(platform: PlatformType) -> i64 {
    match platform {
        PlatformType::Rig => RIG_PLATFORM_PROFITABILITY,
        PlatformType::Ground => GROUND_PLATFORM_PROFITABILITY,
        PlatformType::Pump => PUMP_PLATFORM_PROFITABILITY,
    }
}
