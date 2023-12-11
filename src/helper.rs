use crate::model::{BeerModel, CreateBeerModel};
use sqlx::{Error as SqlxError, PgPool};

pub async fn seed_game_entities(pool: &PgPool) {
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
            title: "Heineken".to_string(),
            description: "Becoming the world's leading premium lager".to_string(),
            thumbnail: "https://www.lulu.lv/cache/images/2557649655/heineken-alus-0-33l-5-0_1819124495.jpg".to_string(),
            cost: 15000,
        },
        CreateBeerModel {
            title: "Carlsberg".to_string(),
            description: "Hundreds of beers at the heart of moments that bring people together"
                .to_string(),
            thumbnail: "https://www.spiritsandwine.lv/img/items/92/9230.jpg".to_string(),
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
            thumbnail: "https://www.spiritsandwine.lv/img/items/39/3983.jpeg".to_string(),
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
}
