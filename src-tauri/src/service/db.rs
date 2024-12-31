use chrono::prelude::*;
use futures::{ future::ok, StreamExt };
use serde::Serialize;
use serde_json::Error;
use sqlx::{
    error,
    migrate::MigrateDatabase,
    pool,
    query,
    sqlite::{ SqliteQueryResult, SqliteRow },
    FromRow,
    Row,
    Sqlite,
    SqlitePool,
};
use tauri_plugin_dialog::{ DialogExt, MessageDialogKind };
use std::{ borrow::BorrowMut, result::{ self, Result } };

use self::constants::API;
use self::{ constants::DB_URL, Entity::{ SynchronizationStruct, TransactionUpdate } };

#[path = "../model/Entity.rs"]
mod Entity;
#[path = "../utils/constants.rs"]
mod constants;

#[tauri::command]
pub async fn load_entity(username: &str, password: &str) -> Result<Option<Entity::Entity>, String> {
    //https://office-server-481p.onrender.com/
    println!("getting entity");

    let request_url = format!("{}{}", String::from(&API.to_string()), String::from("load-entity"));
    let mut entity: Option<Entity::Entity> = None;

    let params = [
        ("username", username),
        ("password", password),
    ];
    let client = reqwest::Client::new();
    let response = client.post(request_url).form(&params).send().await;
    match response {
        Ok(res) => {
            let result = res.text().await;
            match result {
                Ok(s) => {
                    let ress: Result<Entity::LoadEntity, Error> = serde_json::from_str(s.as_str());
                    match ress {
                        Ok(parsed) => {
                            let cloned_entity = parsed.entity.clone();
                            // println!("Success! {:?}", parsed);
                            entity = Some(parsed.entity);

                            let result = save_entity(&DB_URL, cloned_entity, parsed.products).await;
                            match result {
                                Ok(res) => {
                                    println!("Hm,  Success");
                                    Ok(res)
                                }
                                Err(e) => {
                                    println!("{:?}", e);
                                    return Err(e.to_string());
                                }
                            }
                        }
                        Err(e) => {
                            println!("{:?}", e);
                            return Err(e.to_string());
                        }
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                    return Err(e.to_string());
                }
            }
            // match result {
            //     Ok(parsed) => {
            //         let cloned_entity = parsed.entity.clone();
            //         println!("Success! {:?}", parsed);
            //         entity = Some(parsed.entity);

            //         let result = save_entity(&DB_URL, cloned_entity, parsed.products).await;
            //         match result {
            //             Ok(res) => {}
            //             Err(e) => {
            //                 println!("{:?}", e);
            //             }
            //         }
            //     }
            //     Err(e) => println!("Hm,  {:?}", e),
            // }
        }
        Err(e) => {
            println!("{e}");
            return Err(e.to_string());
        }
    }
    // Ok(entity)
}

pub async fn create_tables(dbUrl: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(&DB_URL).await?;
    let qry =
        "
    CREATE TABLE IF NOT EXISTS entity
    (
        entity_id     INTEGER PRIMARY KEY AUTOINCREMENT,
        username     VARCHAR(255) NOT NULL,
        business_name     VARCHAR(255) ,
        _id     VARCHAR(255) NOT NULL,
        password     VARCHAR(255) NOT NULL,
        email     VARCHAR(255)  NOT NULL,
        phone     VARCHAR(255)    NOT NULL,
        message    VARCHAR(255)   ,
        address     VARCHAR(255)  ,
        terminal     VARCHAR(255) NOT NULL,
        terminal_count     INTEGER NOT NULL,
        createdAt     DATETIME   NOT NULL,
        updatedAt      DATETIME   NOT NULL,
        printSilently   BOOLEAN   NOT NULL DEFAULT 0,
        syncTime INTEGER NOT NULL,
        __v INTEGER
    );
    CREATE TABLE IF NOT EXISTS product
    (
        product_id      INTEGER PRIMARY KEY AUTOINCREMENT,
        name    VARCHAR(255),
        _id    VARCHAR(255) UNIQUE,
        quantity    FLOAT,
        category    VARCHAR(255),
        barcode    VARCHAR(255),
        reorderLimit    FLOAT,
        sell_price    FLOAT,
        cost_price    FLOAT,
        total_profit    FLOAT,
        quantity_sold    FLOAT,
        refunded_quantity    FLOAT DEFAULT 0,
        refunded_amount    FLOAT DEFAULT 0,
        entityId    VARCHAR(255),
        is_archived    BOOLEAN,
        isBalanced    BOOLEAN,
        updatedAt      DATETIME DEFAULT    (datetime('now','localtime')),
        createdAt      DATETIME DEFAULT    (datetime('now','localtime')),
        expiry_date      DATETIME DEFAULT    (datetime('now','localtime')) );

    CREATE TABLE IF NOT EXISTS \"transaction\"( 
        transaction_id INTEGER PRIMARY KEY AUTOINCREMENT,
      _id    VARCHAR(255),
        staff    VARCHAR(255),
        staffId    INTEGER,
        discount    FLOAT,
        profit    FLOAT,
        total    FLOAT,
        change FLOAT,
        invoiceNumber   VARCHAR(255),
        terminal     VARCHAR(255),
        __v    INTEGER,
        entityId    VARCHAR(255),
        mop    VARCHAR(255),      
        is_synced BOOLEAN DEFAULT false,
        customer INTEGER,
        createdAt      DATETIME DEFAULT    (datetime('now','localtime'))
   );
   PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS item (
    item_id INTEGER PRIMARY KEY AUTOINCREMENT,  
    _id VARCHAR(255),
    quantity FLOAT,
    product_id VARCHAR(255),
    product_name VARCHAR(255), 
    discount FLOAT,
    sell_price FLOAT,
    profit FLOAT,
    total FLOAT, 
    transaction_id INTEGER,
    FOREIGN KEY(product_id) REFERENCES product(_id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY(transaction_id) REFERENCES \"transaction\"(transaction_id) ON DELETE CASCADE ON UPDATE CASCADE
);
    CREATE TABLE IF NOT EXISTS staff
    (   
        staffId INTEGER PRIMARY KEY AUTOINCREMENT,
        username VARCHAR(255),
        password VARCHAR(255),
        name VARCHAR(255),
        surname VARCHAR(255),
        position VARCHAR(255)
    );


    CREATE TABLE IF NOT EXISTS payment_method
    (   
        sqlId INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(255),
        hidden BOOLEAN,
        _id VARCHAR(255) UNIQUE 
   
    );

    CREATE TABLE IF NOT EXISTS payment
    (   
        sqlId INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(255),
        transactionId INTEGER,  
        value REAL,
        payment_method_id VARCHAR(255),
        _id VARCHAR(255) ,  
        FOREIGN KEY(payment_method_id) REFERENCES payment_method(_id),
    FOREIGN KEY(transactionId) REFERENCES \"transaction\"(transaction_id) ON DELETE CASCADE ON UPDATE CASCADE );
    CREATE TABLE IF NOT EXISTS customer
    (   
        customerId INTEGER PRIMARY KEY AUTOINCREMENT,
        mongoId VARCHAR(255),
        firstName VARCHAR(255),
        fullName VARCHAR(255),
        lastName VARCHAR(255),
        phone VARCHAR(255),
        entityId VARCHAR(255),
        wallet REAL DEFAULT 0.0,
        totalRevenue REAL DEFAULT 0.0,
         createdAt  DATETIME DEFAULT    (datetime('now','localtime')),
        updatedAt  DATETIME DEFAULT    (datetime('now','localtime'))
    );

    CREATE TABLE IF NOT EXISTS product_update
    (   
        update_id INTEGER PRIMARY KEY AUTOINCREMENT,
        entity_id VARCHAR(255),
        _id VARCHAR(255),
        change_type VARCHAR(255),
        origin VARCHAR(255),
        createdAt  DATETIME DEFAULT    (datetime('now','localtime')),
        updatedAt  DATETIME DEFAULT    (datetime('now','localtime')),
        product_id VARCHAR(255),
        quantity FLOAT,
        profit FLOAT,
        total_profit FLOAT,
        quantity_sold FLOAT,
        cost_price FLOAT,
        sell_price FLOAT,
        name VARCHAR(255),
        expiry_date DATETIME,
        reorderLimit VARCHAR(255),
        is_archived    BOOLEAN,
        barcode VARCHAR(255),    category    VARCHAR(255)
    );
    CREATE TABLE IF NOT EXISTS transaction_update
    (   
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        entity_id VARCHAR(255),
        cloud_id VARCHAR(255),
        transaction_id INTEGER,
        createdAt  DATETIME DEFAULT    (datetime('now','localtime')),
        origin VARCHAR(255)
    );
    CREATE TABLE IF NOT EXISTS transaction_update_details
    (   
        detail_id INTEGER PRIMARY KEY AUTOINCREMENT,
        product_id VARCHAR(255),
        quantity  FLOAT,
        profit  FLOAT,
        discount  FLOAT,
        total  FLOAT,
        transaction_id INTEGER,
        FOREIGN KEY(transaction_id) REFERENCES transaction_update(update_id) ON DELETE CASCADE ON UPDATE CASCADE
    ); ";
    let result = sqlx::query(&qry).execute(&pool).await;
    pool.close().await;
    return result;
}

#[tauri::command]
pub async fn check_entity() -> bool {
    println!("checking entity entity");
    let mut exists = false;
    let pool = SqlitePool::connect(&DB_URL).await;
    match pool {
        Ok(pool) => {
            let query = "Select * From entity;";
            let result = sqlx::query(&query).fetch_one(&pool).await;
            match result {
                Ok(res) => {
                    let username: String = res.get("username");
                    println!("{:#?}", username);
                    exists = true;
                }
                Err(e) => {
                    println!("{e}");
                    exists = false;
                }
            }
        }
        Err(e) => {
            exists = false;
        }
    }

    exists
}

#[tauri::command]
pub async fn login(username: &str, password: &str) -> Result<Option<Entity::Staff>, String> {
    println!("{username} {password}");

    let mut staff: Option<Entity::Staff> = None;
    let pool = SqlitePool::connect(&DB_URL).await;
    match pool {
        Ok(pool) => {
            let query = "Select * From staff WHERE username = $1 AND password = $2;";
            let result = sqlx::query(&query).bind(username).bind(password).fetch_one(&pool).await;
            match result {
                Ok(res) => {
                    staff = Some(Entity::Staff {
                        username: res.get("username"),
                        name: res.get("name"),
                        surname: res.get("surname"),
                        position: res.get("position"),
                        staffId: res.get("staffId"),
                    });
                    return Ok(staff);
                }
                Err(e) => {
                    println!("{:?}", e);
                    return Err(e.to_string());
                }
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
pub async fn search_products(queryString: String) -> Result<Option<Vec<Entity::Product>>, String> {
    log::info!("Searching for {}", queryString); // corrected the println statement

    let pool = SqlitePool::connect(&DB_URL).await.map_err(|e| e.to_string())?;

    let query = "SELECT * FROM product WHERE name LIKE ?1 ORDER BY name ASC LIMIT 30;"; // corrected the SQL query
    let queryResult = sqlx
        ::query(query)
        .bind(format!("%{}%", queryString)) // corrected the binding
        .fetch_all(&pool).await
        .map_err(|e| e.to_string())?;

    let products: Vec<Entity::Product> = queryResult
        .into_iter()
        .map(|row: SqliteRow| Entity::Product::from_row(&row).map_err(|e| e.to_string()))
        .collect::<Result<Vec<Entity::Product>, _>>()?;
    if products.is_empty() {
        Ok(Some(products))
    } else {
        Ok(Some(products))
    }
}

pub async fn save_entity(
    dbUrl: &str,
    entity: Entity::Entity,
    products: Vec<Entity::Product>
) -> Result<Option<Entity::Entity>, sqlx::Error> {
    println!("saving entity");
    let pool = SqlitePool::connect(&DB_URL).await?;

    let qry =
        "INSERT INTO entity(username,password,email,phone,address,terminal,createdAt,updatedAt,__v,printSilently,syncTime,_id,business_name,terminal_count) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14);
    INSERT INTO staff(username,name,surname,position,password) VALUES('admin','admin','admin','ADMIN','admin');
    ";
    let result = sqlx
        ::query(&qry)
        .bind(entity.username)
        .bind(entity.password)
        .bind(entity.email)
        .bind(entity.phone)
        .bind(entity.address)
        .bind(entity.terminal)
        .bind(entity.createdAt)
        .bind(entity.updatedAt)
        .bind(entity.__v)
        .bind(entity.printSilently)
        .bind(entity.syncTime)
        .bind(entity._id)
        .bind(entity.businessName)
        .bind(entity.terminal_count)
        .execute(&pool).await;

    for product in products {
        let product_name = product.name.clone();
        let query =
            "INSERT INTO product (_id,name,quantity,category,barcode,reorderLimit,sell_price,cost_price,expiry_date,total_profit,quantity_sold,refunded_quantity,refunded_amount,entityId,is_archived,isBalanced,createdAt,updatedAt) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18);";

        let result = sqlx
            ::query(&query)
            .bind(product._id)
            .bind(product.name)
            .bind(product.quantity)
            .bind(product.category)
            .bind(product.barcode)
            .bind(product.reorderLimit)
            .bind(product.sell_price)
            .bind(product.cost_price)
            .bind(product.expiry_date)
            .bind(product.total_profit)
            .bind(product.quantity_sold)
            .bind(product.refunded_quantity)
            .bind(product.refunded_amount)
            .bind(product.entityId)
            .bind(product.isArchived)
            .bind(product.isBalanced)
            .bind(product.createdAt)
            .bind(product.updatedAt)
            .execute(&pool).await;
    }

    for payment_method in entity.paymentMethods.unwrap() {
        println!("payment method {:?}", payment_method);
        let result = sqlx
            ::query("INSERT INTO payment_method(_id,name,hidden) VALUES(?1,?2,?3);")
            .bind(payment_method._id)
            .bind(payment_method.name)
            .bind(payment_method.hidden)
            .execute(&pool).await;
    }
    pool.close().await;
    return Ok(Some(get_entity().await));
}

#[tauri::command]
pub async fn save_transaction(transaction: Entity::Transaction) -> Result<bool, String> {
    dbg!(&transaction);
    println!("{:#?}", transaction);
    let pool = SqlitePool::connect(&DB_URL).await;
    match pool {
        Ok(pool) => {
            let transaction_query =
                "INSERT INTO \"transaction\" ( staff, staffId, discount, profit, total, change, invoiceNumber, terminal, entityId, mop,customer) 
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,$11);";
            match
                sqlx
                    ::query(transaction_query)
                    .bind(transaction.staff)
                    .bind(transaction.staffId)
                    .bind(transaction.discount)
                    .bind(transaction.profit)
                    .bind(&transaction.total)
                    .bind(transaction.change)
                    .bind(transaction.invoiceNumber)
                    .bind(&transaction.terminal)
                    .bind(&transaction.entityId)
                    .bind(transaction.mop)
                    .bind(&transaction.customer)
                    .execute(&pool).await
            {
                Ok(result) => {
                    println!("Transaction saved successfully");
                    let transaction_id = result.last_insert_rowid();
                    for item in transaction.items {
                        let query =
                            "
                            UPDATE product
                            SET 
                                quantity = quantity - $1,
                                quantity_sold = quantity_sold + $1,
                                total_profit = total_profit + $2
                            WHERE
                                _id = $3
                            ";
                        let result = sqlx
                            ::query(query)
                            .bind(item.quantity)
                            .bind(item.profit)
                            .bind(&item.product_id)
                            .execute(&pool).await;
                        match result {
                            Ok(result) => {
                                println!("Product updated successfully");
                            }
                            Err(error) => {
                                println!("{:#?}", error);
                            }
                        }
                        let item_query =
                            "INSERT INTO item ( quantity, product_id, product_name, discount, sell_price, profit, total, transaction_id)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8);";
                        println!("{}", item.product_id);
                        match
                            sqlx
                                ::query(&item_query)
                                .bind(item.quantity)
                                .bind(&item.product_id)
                                .bind(item.name)
                                .bind(item.discount)
                                .bind(item.sell_price)
                                .bind(item.profit)
                                .bind(item.total)
                                .bind(&transaction_id)
                                .execute(&pool).await
                        {
                            Ok(result) => {
                                println!("item saved successfully");
                            }
                            Err(error) => {
                                println!("{:#?}", error.to_string());
                            }
                        }
                        let update_query =
                            "INSERT INTO product_update ( change_type, origin, quantity, profit,entity_id,product_id)
                        VALUES ($1, $2, $3, $4, $5, $6);";
                        println!("{}", item.product_id);
                        match
                            sqlx
                                ::query(&update_query)
                                .bind(String::from("sale"))
                                .bind(&transaction.terminal)
                                .bind(item.quantity)
                                .bind(item.profit)
                                .bind(&transaction.entityId)
                                .bind(item.product_id)
                                .execute(&pool).await
                        {
                            Ok(result) => {
                                println!("update saved successfully");
                            }
                            Err(error) => {
                                println!("{:#?}", error.to_string());
                            }
                        }
                    }

                    let customer_query =
                        "   
                            UPDATE customer
                            SET 
                                wallet = wallet + $1,
                                totalRevenue = totalRevenue + $2
                                
                            WHERE
                                customerId = $3
                            ";

                    match
                        sqlx
                            ::query(customer_query)
                            .bind(0.0025 * transaction.total)
                            .bind(&transaction.total)
                            .bind(transaction.customer)
                            .execute(&pool).await
                    {
                        Ok(res) => {
                            println!("customer update successful");
                        }
                        Err(e) => {
                            println!("customer query error {:#?}", e);
                        }
                    }
                    for payment in transaction.payments.unwrap() {
                        let payment_query =
                            "INSERT INTO payment (name,value,transactionId,payment_method_id) VALUES($1,$2,$3,$4);";
                        match
                            sqlx
                                ::query(&payment_query)
                                .bind(payment.name)
                                .bind(payment.value)
                                .bind(&transaction_id)
                                .bind(payment.payment_method_id)
                                .execute(&pool).await
                        {
                            Ok(res) => {
                                println!("payment saved successfully {:#?}", res);
                            }
                            Err(e) => {
                                println!("payment error {:#?}", e);
                            }
                        };
                    }
                }

                Err(error) => {
                    println!("Transaction table error {:#?}", error.to_string());
                }
            }

            pool.close().await;
        }
        Err(error) => {
            println!("{:#?}", error);
            error.to_string();
        }
    }

    Ok(true)
}
#[tauri::command]
pub async fn synchronize() -> Result<Entity::SyncInfo, Entity::SyncInfo> {
    let mut entity: Entity::Entity = get_entity().await;
    let clonedEntity = entity.clone();
    let mut syncResult = Entity::SyncInfo {
        localUpdates: 0,
        cloudUpdates: 0,
        status: false,
        message: String::from("Update failed"),
        entity: clonedEntity,
    };
    let request_url = format!("{}{}", String::from(&API.to_string()), String::from("synchronize"));
    let mut productUpdates = get_product_updates().await.unwrap();
    let cloned = productUpdates.clone();
    let terminal = &entity.terminal;
    //  let mut transactionUpdates = vec![];
    let mut transactions = get_transactions_for_synchronization().await.unwrap();
    let client = reqwest::Client::new();
    let clonedLocalTrxs = transactions.clone();
    let syncStruct = SynchronizationStruct {
        productUpdates,
        transactionUpdates: get_transaction_updates().await.unwrap(),
        transactions,
        terminal: String::from(terminal),
        entity_id: String::from(&entity._id),
    };
    syncResult.localUpdates = cloned.len() as i32;
    let response = client.post(request_url).json(&syncStruct).send().await;
    match response {
        Ok(res) => {
            let pool = match SqlitePool::connect(&DB_URL).await {
                Ok(pool) => pool,
                Err(e) => {
                    eprintln!("Failed to fetch product updates: {}", e);
                    syncResult.cloudUpdates = 0;
                    syncResult.localUpdates = 0;
                    syncResult.message = String::from(
                        String::from("Sync failed, ") + &e.to_string()
                    );
                    return Err(syncResult);
                }
            };

            let cloudUpdates = res.json::<Entity::CloudStruct>().await;
            match cloudUpdates {
                Ok(mut updates) => {
                    clear_updates(&updates.transactions).await;
                    syncResult.entity = updates.entity.clone();
                    let entity = updates.entity.clone();
                    let mut query = "";
                    let cloned = updates.productUpdates.clone();
                    syncResult.cloudUpdates = cloned.len() as i32;
                    let request_url = format!(
                        "{}{}",
                        String::from(&API.to_string()),
                        String::from("confirm_update")
                    );
                    let response = client.post(request_url).json(&updates).send().await;
                    match response {
                        Ok(res) => {
                            println!("update confirmed {:?}", res.status());
                            syncResult.status = true;
                            syncResult.message = format!(
                                "Synchronization Successful\n{} cloud Updates\n{} Local Updates",
                                syncResult.cloudUpdates,
                                syncResult.localUpdates
                            );
                        }
                        Err(e) => {
                            println!("{}", e.to_string());
                            syncResult.cloudUpdates = 0;
                            syncResult.localUpdates = 0;
                            syncResult.message = String::from(
                                String::from("Sync failed, ") + &e.to_string()
                            );
                            return Err(syncResult);
                        }
                    }
                    for update in &mut updates.productUpdates {
                        match update.change_type.as_mut().unwrap().as_str() {
                            "new" => {
                                query =
                                    "INSERT INTO product (_id,name,quantity,category,barcode,reorderLimit,sell_price,cost_price,expiry_date,total_profit,quantity_sold,entityId,is_archived,createdAt,updatedAt) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15);";

                                let result = sqlx
                                    ::query(&query)
                                    .bind(&update._id)
                                    .bind(&update.name)
                                    .bind(update.quantity)
                                    .bind(&update.category)
                                    .bind(&update.barcode)
                                    .bind(update.reorderLimit)
                                    .bind(update.sell_price)
                                    .bind(update.cost_price)
                                    .bind(&update.expiry_date)
                                    .bind(update.total_profit)
                                    .bind(update.quantity_sold)
                                    .bind(&update.entity_id)
                                    .bind(update.isArchived)
                                    .bind(&update.createdAt)
                                    .bind(&update.updatedAt)
                                    .execute(&pool).await;
                                match result {
                                    Ok(result) => {
                                        println!("product updated {:?}", result);
                                    }
                                    Err(e) => {
                                        println!("{}", e.to_string());
                                        syncResult.cloudUpdates = 0;
                                        syncResult.localUpdates = 0;
                                        syncResult.message = String::from(
                                            String::from("Sync failed, ") + &e.to_string()
                                        );
                                        return Err(syncResult);
                                    }
                                }
                            }
                            "delete" => {
                                query = "DELETE FROM product WHERE _id = $1";

                                let result = sqlx
                                    ::query(&query)
                                    .bind(&update._id)
                                    .execute(&pool).await;
                                match result {
                                    Ok(result) => {
                                        println!("product updated {:?}", result);
                                    }
                                    Err(e) => {
                                        println!("{}", e.to_string());
                                        syncResult.cloudUpdates = 0;
                                        syncResult.localUpdates = 0;
                                        syncResult.message = String::from(
                                            String::from("Sync failed, ") + &e.to_string()
                                        );
                                        return Err(syncResult);
                                    }
                                }
                            }
                            _ => {
                                println!("\n {:?} \n", update);
                                query =
                                    "UPDATE product
                        SET 
                            quantity = $1,
                            quantity_sold = $2,
                            total_profit = $3,
                            cost_price = $4,
                            sell_price = $5,
                            name = $6,
                            barcode = $7,
                            reorderLimit = $8,
                            expiry_date = $9,
                            is_archived = $10
                        WHERE
                            _id = $11";
                                let query_result = sqlx
                                    ::query(query)
                                    .bind(update.quantity)
                                    .bind(update.quantity_sold)
                                    .bind(update.total_profit)
                                    .bind(update.cost_price)
                                    .bind(&update.sell_price)
                                    .bind(&update.name)
                                    .bind(&update.barcode)
                                    .bind(update.reorderLimit)
                                    .bind(&update.expiry_date)
                                    .bind(&update.isArchived)
                                    .bind(&update.product_id)
                                    .execute(&pool).await;
                                match query_result {
                                    Ok(result) => {
                                        println!("product updated {:?}", result);
                                    }
                                    Err(e) => {
                                        println!("{}", e.to_string());
                                        syncResult.cloudUpdates = 0;
                                        syncResult.localUpdates = 0;
                                        syncResult.message = String::from(
                                            String::from("Sync failed, ") + &e.to_string()
                                        );
                                        return Err(syncResult);
                                    }
                                }
                            }
                        }
                    }

                    let query =
                        "UPDATE entity  
                    Set
                     business_name = $1,
                     email = $2,
                     phone =  $3,
                     message =  $4,
                     password =  $5,
                     address =  $6,
                     terminal =  $7,
                     createdAt =  $8,
                     updatedAt =  $9,
                     printSilently = $10,
                     syncTime =  $11
                     WHERE _id = $12
                    ;";
                    let query_result = sqlx
                        ::query(&query)
                        .bind(entity.businessName)
                        .bind(entity.email)
                        .bind(entity.phone)
                        .bind(entity.message)
                        .bind(entity.password)
                        .bind(entity.address)
                        .bind(entity.terminal)
                        .bind(entity.createdAt)
                        .bind(entity.updatedAt)
                        .bind(entity.printSilently)
                        .bind(entity.syncTime)
                        .bind(entity._id)
                        .execute(&pool).await;
                    match query_result {
                        Ok(result) => println!("Entity update successfull {:?}", result),
                        Err(e) => {
                            println!("{}", e.to_string());
                            syncResult.cloudUpdates = 0;
                            syncResult.localUpdates = 0;
                            syncResult.message = String::from(
                                String::from("Sync failed, ") + &e.to_string()
                            );
                            return Err(syncResult);
                        }
                    }

                    for payment_method in entity.paymentMethods.unwrap() {
                        println!("payment method {:?}", payment_method);
                        let p_result = sqlx
                            ::query(
                                "INSERT OR IGNORE INTO payment_method(_id,name,hidden) VALUES(?1,?2,?3);"
                            )
                            .bind(payment_method._id)
                            .bind(payment_method.name)
                            .bind(payment_method.hidden)
                            .execute(&pool).await;
                        match p_result {
                            Ok(result) => {
                                println!("payment method update successfull {:?}", &result);
                            }
                            Err(e) => {
                                println!("{:?}", e.to_string());
                                syncResult.cloudUpdates = 0;
                                syncResult.localUpdates = 0;
                                syncResult.message = String::from(
                                    String::from("Sync failed, ") + &e.to_string()
                                );
                                return Err(syncResult);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("error{:?}", e);
                    syncResult.cloudUpdates = 0;
                    syncResult.localUpdates = 0;
                    syncResult.message = String::from(
                        String::from("Sync failed, ") + &e.to_string()
                    );
                    return Err(syncResult);
                }
            }
        }
        Err(e) => {
            println!("{}", e.to_string());
            syncResult.cloudUpdates = 0;
            syncResult.localUpdates = 0;
            syncResult.message = String::from(String::from("Sync failed, ") + &e.to_string());
            return Err(syncResult);
        }
    }
    Ok(syncResult)
}

pub async fn get_product_updates() -> Option<Vec<Entity::ProductUpdate>> {
    // corrected the println statement
    let pool = match SqlitePool::connect(&DB_URL).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to fetch product updates: {}", e);
            return None;
        }
    };

    let query = "SELECT * FROM product_update;"; // corrected the SQL query
    let queryResult = match sqlx::query(query).fetch_all(&pool).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to fetch product updates: {}", e);
            return None;
        }
    };

    let product_updates: Vec<Entity::ProductUpdate> = queryResult
        .into_iter()
        .filter_map(|row| {
            match Entity::ProductUpdate::from_row(&row) {
                Ok(product_update) => Some(product_update),
                Err(e) => {
                    eprintln!("Failed to fetch product updates: {}", e);
                    None
                }
            }
        })
        .collect();
    pool.close();
    Some(product_updates)
}

#[tauri::command]
pub async fn get_transactions_for_synchronization() -> Option<Vec<Entity::Transaction>> {
    // corrected the println statement

    let pool = match SqlitePool::connect(&DB_URL).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to fetch transactions: {}", e);
            return None;
        }
    };

    let query = "SELECT * FROM \"transaction\" where is_synced = false;"; // corrected the SQL query
    let queryResult = match sqlx::query(query).fetch_all(&pool).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to fetch transactions: {}", e);
            return None;
        }
    };

    let mut transactions: Vec<Entity::Transaction> = futures::stream
        ::iter(queryResult)
        .filter_map(|row| async move {
            match Entity::Transaction::from_row(&row) {
                Ok(transaction) => {
                    print!("transaction synced");
                    Some(transaction)
                }
                Err(e) => {
                    eprintln!("Failed to parse transaction: {}", e);
                    None
                }
            }
        })
        .collect().await;

    for transaction in &mut transactions {
        let query = "SELECT * FROM item where transaction_id = $1";
        let queryResult = match
            sqlx::query(query).bind(transaction.transaction_id).fetch_all(&pool).await
        {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Failed to fetch transactions: {}", e);
                return None;
            }
        };
        let mut items: Vec<Entity::Item> = queryResult
            .into_iter()
            .filter_map(|row| {
                match Entity::Item::from_row(&row) {
                    Ok(item) => Some(item),
                    Err(e) => {
                        eprintln!("Failed to fetch transactions: {}", e);
                        return None;
                    }
                }
            })
            .collect();
        for item in &mut items {
            let query = "SELECT * FROM product where _id = $1";
            let queryResult = match
                sqlx::query(&query).bind(&item.product_id).fetch_one(&pool).await
            {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Failed to fetch transactions: {}", e);
                    return None;
                }
            };
            let product: Entity::Product = match Entity::Product::from_row(&queryResult) {
                Ok(prod) => prod,
                Err(e) => {
                    eprintln!("Failed to fetch transactions: {}", e);
                    return None;
                }
            };
            item.product = Some(product);
        }

        let payment_query =
            "SELECT value, payment_method_id, name, transactionId from payment where transactionId = $1";
        let payments = match
            sqlx
                ::query_as::<_, Entity::Payment>(&payment_query)
                .bind(&transaction.transaction_id)
                .fetch_all(&pool).await
        {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Failed to fetch transactions: {}", e);
                return None;
            }
        };
        transaction.items = items;
        transaction.payments = Some(payments);
    }
    Some(transactions)
}

#[tauri::command]
pub async fn get_transactions() -> Option<Vec<Entity::Transaction>> {
    // corrected the println statement

    let pool = match SqlitePool::connect(&DB_URL).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to fetch transactions: {}", e);
            return None;
        }
    };
    let date = NaiveDate::from_ymd(Utc::now().year(), Utc::now().month(), Utc::now().day());

    // Create a NaiveTime instance representing midnight
    let midnight = NaiveTime::from_hms(0, 0, 0);

    // Combine the date and time to get the beginning of the day
    let day = NaiveDateTime::new(date, midnight).to_string();
    println!("{:?}", day);
    let query = "SELECT * FROM \"transaction\" where createdAt >?1 ORDER BY createdAt DESC;"; // corrected the SQL query
    let queryResult = match sqlx::query(query).bind(&day).fetch_all(&pool).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to fetch transactions: {}", e);
            return None;
        }
    };

    let mut transactions: Vec<Entity::Transaction> = queryResult
        .into_iter()
        .filter_map(|row| {
            match Entity::Transaction::from_row(&row) {
                Ok(transaction) => {
                    return Some(transaction);
                }
                Err(e) => {
                    eprintln!("Failed to fetch transactions: {}", e);
                    return None;
                }
            }
        })
        .collect();

    for transaction in &mut transactions {
        let query = "SELECT * FROM item where transaction_id = $1";
        let queryResult = match
            sqlx::query(query).bind(transaction.transaction_id).fetch_all(&pool).await
        {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Failed to fetch transactions: {}", e);
                return None;
            }
        };
        let mut items: Vec<Entity::Item> = queryResult
            .into_iter()
            .filter_map(|row| {
                match Entity::Item::from_row(&row) {
                    Ok(item) => Some(item),
                    Err(e) => {
                        eprintln!("Failed to fetch transactions: {}", e);
                        return None;
                    }
                }
            })
            .collect();
        for item in &mut items {
            let query = "SELECT * FROM product where _id = $1";
            let queryResult = match
                sqlx::query(&query).bind(&item.product_id).fetch_one(&pool).await
            {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Failed to fetch transactions: {}", e);
                    return None;
                }
            };
            let product: Entity::Product = match Entity::Product::from_row(&queryResult) {
                Ok(prod) => prod,
                Err(e) => {
                    eprintln!("Failed to fetch transactions: {}", e);
                    return None;
                }
            };
            item.product = Some(product);
        }
        transaction.items = items;
    }
    Some(transactions)
}

#[tauri::command]
pub async fn search_transactions(search_query: String) -> Option<Vec<Entity::Transaction>> {
    // corrected the println statement

    let pool = match SqlitePool::connect(&DB_URL).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to fetch transactions: {}", e);
            return None;
        }
    };

    let query =
        "SELECT t.*,i.product_name,i.product_id FROM \"transaction\" t JOIN item i ON t.transaction_id = i.transaction_id WHERE product_name like ?1 OR t.invoiceNumber like ?2 GROUP by t.transaction_id ORDER BY t.createdAt DESC"; // corrected the SQL query
    let queryResult = match
        sqlx
            ::query(query)
            .bind(format!("%{}%", search_query))
            .bind(format!("%{}%", search_query))
            .fetch_all(&pool).await
    {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to fetch transactions: {}", e);
            return None;
        }
    };

    let mut transactions: Vec<Entity::Transaction> = queryResult
        .into_iter()
        .filter_map(|row| {
            match Entity::Transaction::from_row(&row) {
                Ok(transaction) => {
                    println!("{:?}", transaction);
                    return Some(transaction);
                }
                Err(e) => {
                    eprintln!("Failed to fetch transactions: {}", e);
                    return None;
                }
            }
        })
        .collect();

    for transaction in &mut transactions {
        let query = "SELECT * FROM item where transaction_id = $1";
        let queryResult = match
            sqlx::query(query).bind(transaction.transaction_id).fetch_all(&pool).await
        {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Failed to fetch transactions: {}", e);
                return None;
            }
        };
        let mut items: Vec<Entity::Item> = queryResult
            .into_iter()
            .filter_map(|row| {
                match Entity::Item::from_row(&row) {
                    Ok(item) => Some(item),
                    Err(e) => {
                        eprintln!("Failed to fetch transactions: {}", e);
                        return None;
                    }
                }
            })
            .collect();
        for item in &mut items {
            let query = "SELECT * FROM product where _id = $1";
            let queryResult = match
                sqlx::query(&query).bind(&item.product_id).fetch_one(&pool).await
            {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Failed to fetch transactions: {}", e);
                    return None;
                }
            };
            let product: Entity::Product = match Entity::Product::from_row(&queryResult) {
                Ok(prod) => prod,
                Err(e) => {
                    eprintln!("Failed to fetch transactions: {}", e);
                    return None;
                }
            };
            item.product = Some(product);
        }
        transaction.items = items;
    }
    Some(transactions)
}
pub async fn get_transaction_updates() -> Option<Vec<Entity::TransactionUpdate>> {
    // corrected the println statement
    let mut transaction_updates: Vec<TransactionUpdate> = vec![];
    let pool = match SqlitePool::connect(&DB_URL).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to fetch transaction updates: {}", e);
            return None;
        }
    };
    let query = "SELECT * FROM transaction_update;"; // corrected the SQL query
    let queryResult = match sqlx::query(query).fetch_all(&pool).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to fetch transaction updates: {}", e);
            return None;
        }
    };
    transaction_updates = queryResult
        .into_iter()
        .filter_map(|row| {
            match Entity::TransactionUpdate::from_row(&row) {
                Ok(transaction_updates) => Some(transaction_updates),
                Err(e) => {
                    eprintln!("Failed to fetch transaction updates: {}", e);
                    return None;
                }
            }
        })
        .collect();

    if transaction_updates.is_empty() {
        return Some(vec![]);
    }

    Some(transaction_updates)
}

pub async fn get_entity() -> Entity::Entity {
    println!("checking entity entity");

    let pool = SqlitePool::connect(&DB_URL).await.unwrap();
    let mut entity: Entity::Entity;

    let query = "Select * From entity;";
    let result = sqlx::query(&query).fetch_one(&pool).await.unwrap();

    let payment_methods = sqlx
        ::query_as::<_, Entity::PaymentMethod>("Select * from payment_method WHERE hidden = FALSE;")
        .fetch_all(&pool).await
        .unwrap();

    entity = Entity::Entity::from_row(&result).unwrap();
    entity.paymentMethods = Some(payment_methods);
    pool.close().await;
    return entity;
}

#[derive(Serialize)]
pub struct ApiError {
    pub message: String,
}

#[tauri::command]
pub async fn save_customer(
    firstName: String,
    phone: String,
    lastName: String
) -> Result<Entity::Customer, ApiError> {
    let pool = SqlitePool::connect(&DB_URL).await.unwrap();
    let query =
        "INSERT INTO customer(firstName,lastName,phone,fullName) VALUES($1,$2,$3,$4) RETURNING *, CAST(wallet AS REAL) AS wallet,  CAST(totalRevenue AS REAL) AS totalRevenue;";
    let result = sqlx
        ::query_as::<_, Entity::Customer>(query)
        .bind(&firstName)
        .bind(&lastName)
        .bind(phone)
        .bind(format!("{} {}", &firstName, &lastName))
        .fetch_one(&pool).await
        .map_err(|e| ApiError {
            message: format!("Failed to execute query: {}", e),
        });

    // customer = Entity::Customer::from_row(&result).unwrap();
    pool.close().await;
    return result;
}

#[tauri::command]
pub async fn get_customers(search_query: String) -> Result<Vec<Entity::Customer>, ApiError> {
    let pool = SqlitePool::connect(&DB_URL).await.map_err(|e| ApiError {
        message: format!("Failed to connect to the database: {}", e),
    })?;

    let sql_query = "SELECT * FROM customer WHERE fullName LIKE ?1;";

    let customers: Vec<Entity::Customer> = sqlx
        ::query_as::<_, Entity::Customer>(sql_query)
        .bind(format!("%{}%", search_query)) // Bind the search term
        .fetch_all(&pool).await
        .map_err(|e| ApiError {
            message: format!("Failed to execute query: {}", e),
        })?;

    pool.close().await;

    Ok(customers)
}

async fn clear_updates(transactions: &Vec<Entity::Transaction>) {
    let pool = match SqlitePool::connect(&DB_URL).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to fetch transactions: {}", e);
            return ();
        }
    };

    let query = "DELETE FROM product_update;"; // corrected the SQL query
    let queryResult = match sqlx::query(query).execute(&pool).await {
        Ok(result) => {
            println!("Local product updates deleted successfully {:?}", result);
        }
        Err(e) => {
            eprintln!("Failed to fetch transactions: {}", e);
        }
    };

    for transaction in transactions {
        let query =
            "UPDATE \"transaction\" SET is_synced = true, _id = ?1 WHERE transaction_id = ?2 AND terminal = ?3;";
        let q_result = sqlx
            ::query(&query)
            .bind(&transaction._id)
            .bind(&transaction.transaction_id)
            .bind(&transaction.terminal)
            .execute(&pool).await
            .map_err(|e| {
                eprintln!("Failed to execute query: {}", e);
                e
            });
    }
}

#[tauri::command]
pub async fn refund_transaction(transaction: Entity::Transaction) -> Result<bool, String> {
    let pool = SqlitePool::connect(&DB_URL).await;
    println!("{:#?}", transaction);
    let transaction_id = &transaction.transaction_id;
    match pool {
        Ok(pool) => {
            let transaction_query = "DELETE FROM \"transaction\" WHERE transaction_id = $1;";
            let trx_update_query =
                "INSERT INTO transaction_update (transaction_id, origin,entity_id,cloud_id) VALUES ($1, $2, $3, $4)";
            match
                sqlx
                    ::query(&trx_update_query)
                    .bind(transaction_id)
                    .bind(&transaction.terminal)
                    .bind(&transaction.entityId)
                    .bind(&transaction._id)
                    .execute(&pool).await
            {
                Ok(_) => {
                    println!("refunded product update saved successfully");
                }
                Err(error) => {
                    println!("{:#?}", error.to_string());
                }
            }

            match sqlx::query(transaction_query).bind(transaction_id).execute(&pool).await {
                Ok(result) => {
                    // let delete_item_query = "DELETE FROM item WHERE transaction_id = $1";
                    // match
                    //     sqlx::query(&delete_item_query).bind(&transaction_id).execute(&pool).await
                    // {
                    //     Ok(result) => {
                    //         println!("refunded item deleted successfully");
                    //     }
                    //     Err(error) => {
                    //         println!("{:#?}", error.to_string());
                    //     }
                    // }
                    for item in transaction.items {
                        let query =
                            "
                            UPDATE product
                            SET 
                                quantity = quantity + $1,
                                quantity_sold = quantity_sold - $1,
                                total_profit = total_profit - $2
                            WHERE
                                _id = $3
                            ";
                        let result = sqlx
                            ::query(query)
                            .bind(item.quantity)
                            .bind(item.profit)
                            .bind(&item.product_id)
                            .execute(&pool).await;
                        match result {
                            Ok(result) => {
                                println!("refunded Product updated successfully");
                            }
                            Err(error) => {
                                println!("{:#?}", error);
                            }
                        }

                        let update_query =
                            "INSERT INTO product_update (change_type, origin, quantity, profit,entity_id,product_id)
                        VALUES ($1, $2, $3, $4, $5, $6);";
                        println!("{}", item.product_id);
                        match
                            sqlx
                                ::query(&update_query)
                                .bind(String::from("refund"))
                                .bind(&transaction.terminal)
                                .bind(item.quantity)
                                .bind(item.profit)
                                .bind(&transaction.entityId)
                                .bind(item.product_id)
                                .execute(&pool).await
                        {
                            Ok(result) => {
                                println!("refunded product update saved successfully");
                            }
                            Err(error) => {
                                println!("{:#?}", error.to_string());
                            }
                        }
                    }
                }

                Err(error) => {
                    println!("{:#?}", error.to_string());
                }
            }

            pool.close().await;
        }
        Err(error) => {
            println!("{:#?}", error);
            error.to_string();
        }
    }
    println!("Transaction refunded successfully");
    Ok(true)
}

#[tauri::command]
pub async fn force_update(app: tauri::AppHandle, entity: Entity::Entity) -> Result<bool, String> {
    log::info!("Force update started");
    app.dialog()
        .message("Force update started, please wait;")
        .title("Force Update")
        .kind(MessageDialogKind::Info)
        .blocking_show();
    match SqlitePool::connect(&DB_URL).await {
        Ok(pool) => {
            let url = format!(
                "{}{}{}",
                String::from(&API.to_string()),
                String::from("force_update/"),
                entity._id
            );
            let client = reqwest::Client::new();
            let request = client.get(url).send().await;
            match request {
                Ok(respose) => {
                    match respose.json::<Vec<Entity::Product>>().await {
                        Ok(products) => {
                            let query = "DELETE FROM product";
                            match sqlx::query(&query).execute(&pool).await {
                                Ok(result) => {
                                    for product in products {
                                        let query =
                                            "INSERT INTO product (_id,name,quantity,category,barcode,reorderLimit,sell_price,cost_price,expiry_date,total_profit,quantity_sold,refunded_quantity,refunded_amount,entityId,is_archived,isBalanced,createdAt,updatedAt) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18);";

                                        let result = sqlx
                                            ::query(&query)
                                            .bind(product._id)
                                            .bind(product.name)
                                            .bind(product.quantity)
                                            .bind(product.category)
                                            .bind(product.barcode)
                                            .bind(product.reorderLimit)
                                            .bind(product.sell_price)
                                            .bind(product.cost_price)
                                            .bind(product.expiry_date)
                                            .bind(product.total_profit)
                                            .bind(product.quantity_sold)
                                            .bind(product.refunded_quantity)
                                            .bind(product.refunded_amount)
                                            .bind(product.entityId)
                                            .bind(product.isArchived)
                                            .bind(product.isBalanced)
                                            .bind(product.createdAt)
                                            .bind(product.updatedAt)
                                            .execute(&pool).await;
                                    }
                                    app.dialog()
                                        .message("Force update successful")
                                        .title("PForce Update")
                                        .kind(MessageDialogKind::Info)
                                        .blocking_show();
                                    Ok(true)
                                }
                                Err(error) => {
                                    app.dialog()
                                        .message(error.to_string())
                                        .title("Product Delete failed")
                                        .kind(MessageDialogKind::Error)
                                        .blocking_show();
                                    log::error!("{:#?}", error);
                                    Err(error.to_string())
                                }
                            }
                        }
                        Err(error) => {
                            app.dialog()
                                .message(error.to_string())
                                .title("Force Update Failed")
                                .kind(MessageDialogKind::Error)
                                .blocking_show();
                            log::error!("{:#?}", error);
                            Err(error.to_string())
                        }
                    }
                }

                Err(error) => {
                    log::error!("{:#?}", error);
                    Err(error.to_string())
                }
            }
        }
        Err(error) => {
            log::error!("{:#?}", error);
            Err(error.to_string())
        }
    }
}
