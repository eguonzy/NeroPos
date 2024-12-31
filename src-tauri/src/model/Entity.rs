use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Error, FromRow, Row};
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Entity {
    pub _id: String,
    pub username: String,
    pub businessName: String,
    pub email: String,
    pub phone: String,
    pub message: String,
    pub password: String,
    pub address: String,
    pub terminal: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub __v: i32,
    pub printSilently: bool,
    pub syncTime: i32,
    pub terminal_count: i32,
    #[serde(default)]
    pub paymentMethods: Option<Vec<PaymentMethod>>,
}

impl FromRow<'_, SqliteRow> for Entity {
    fn from_row(row: &SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            // Add more fields as needed
            _id: row.try_get("_id")?,
            __v: row.try_get("__v")?,
            address: row.try_get("address")?,
            businessName: row.try_get("business_name")?,
            createdAt: row.try_get("createdAt")?,
            email: row.try_get("email")?,
            message: row.try_get("message")?,
            password: row.try_get("password")?,
            phone: row.try_get("phone")?,
            printSilently: row.try_get("printSilently")?,
            syncTime: row.try_get("syncTime")?,
            terminal: row.try_get("terminal")?,
            updatedAt: row.try_get("updatedAt")?,
            username: row.try_get("username")?,
            terminal_count: row.try_get("terminal_count")?,
            paymentMethods: None,
        })
    }
}

#[derive(Clone, Deserialize, Debug, Serialize, sqlx::FromRow)]
pub struct PaymentMethod {
    pub name: String,
    pub hidden: bool,
    pub _id: String,
}
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Product {
    pub _id: Option<String>,
    pub name: Option<String>,
    pub quantity: f32,
    pub category: Option<String>,
    pub barcode: Option<String>,
    pub reorderLimit: Option<f32>,
    pub sell_price: f32,
    pub cost_price: f32,
    pub expiry_date: Option<String>,
    pub total_profit: Option<f32>,
    pub quantity_sold: Option<f32>,
    pub refunded_quantity: Option<f32>,
    pub refunded_amount: Option<f32>,
    pub entityId: Option<String>,
    pub isArchived: bool,
    pub isBalanced: bool,
    pub createdAt: Option<String>,
    pub updatedAt: Option<String>,
}
impl FromRow<'_, SqliteRow> for Product {
    fn from_row(row: &SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            name: row.try_get("name")?,
            quantity: row.try_get("quantity")?,
            category: row.try_get("category")?,
            barcode: row.try_get("barcode")?,
            reorderLimit: row.try_get("reorderLimit")?,
            sell_price: row.try_get("sell_price")?,
            cost_price: row.try_get("cost_price")?,
            expiry_date: row.try_get("expiry_date")?,
            total_profit: row.try_get("total_profit")?,
            quantity_sold: row.try_get("quantity_sold")?,
            refunded_quantity: row.try_get("refunded_quantity")?,
            refunded_amount: row.try_get("refunded_amount")?,
            entityId: row.try_get("entityId")?,
            isArchived: row.try_get("is_archived")?,
            isBalanced: row.try_get("isBalanced")?,
            createdAt: row.try_get("createdAt")?,
            updatedAt: row.try_get("updatedAt")?,
            _id: row.try_get("_id")?,
            // Add more fields as needed
        })
    }
}
impl FromIterator<Product> for Option<Vec<Product>> {
    fn from_iter<I: IntoIterator<Item = Product>>(iter: I) -> Self {
        let vec: Vec<Product> = iter.into_iter().collect();
        Some(vec)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct LoadEntity {
    pub entity: Entity,
    pub products: Vec<Product>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Transaction {
    pub _id: Option<String>,
    pub createdAt: Option<String>,
    pub discount: f32,
    pub entityId: String,
    pub invoiceNumber: String,
    pub items: Vec<Item>,
    pub mop: String,
    pub profit: f32,
    pub staff: String,
    pub staffId: i32,
    pub terminal: String,
    pub total: f32,
    pub change: f32,
    pub transaction_id: Option<i32>,
    pub is_synced: Option<bool>,
    pub customer: Option<i32>,
    pub payments: Option<Vec<Payment>>,
}
impl FromRow<'_, SqliteRow> for Transaction {
    fn from_row(row: &SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            createdAt: row.try_get("createdAt")?,
            change: row.try_get("change")?,
            discount: row.try_get("discount")?,
            invoiceNumber: row.try_get("invoiceNumber")?,
            entityId: row.try_get("entityId")?,
            mop: row.try_get("mop")?,
            profit: row.try_get("profit")?,
            staff: row.try_get("staff")?,
            staffId: row.try_get("staffId")?,
            terminal: row.try_get("terminal")?,
            total: row.try_get("total")?,
            transaction_id: row.try_get("transaction_id")?,
            _id: row.try_get("_id")?,
            items: vec![],
            is_synced: row.try_get("is_synced")?,
            customer: row.try_get("customer")?,
            payments: None,
            // Add more fields as needed
        })
    }
}
impl FromIterator<Transaction> for Option<Vec<Transaction>> {
    fn from_iter<I: IntoIterator<Item = Transaction>>(iter: I) -> Self {
        let vec: Vec<Transaction> = iter.into_iter().collect();
        Some(vec)
    }
}
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Item {
    pub quantity: f32,
    pub item_id: Option<i32>,
    pub _id: Option<String>,
    pub name: String,
    pub discount: f32,
    pub sell_price: f32,
    pub profit: f32,
    pub total: f32,
    pub product: Option<Product>,
    pub product_id: String,
}
impl FromRow<'_, SqliteRow> for Item {
    fn from_row(row: &SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            profit: row.try_get("profit")?,
            total: row.try_get("total")?,
            discount: row.try_get("discount")?,
            item_id: row.try_get("item_id")?,
            name: row.try_get("product_name")?,
            product_id: row.try_get("product_id")?,
            quantity: row.try_get("quantity")?,
            sell_price: row.try_get("sell_price")?,
            _id: row.try_get("_id")?,
            product: None,
            // Add more fields as needed
        })
    }
}
impl FromIterator<Item> for Option<Vec<Item>> {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        let vec: Vec<Item> = iter.into_iter().collect();
        Some(vec)
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Staff {
    pub staffId: i32,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub position: String,
}
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Update {
    pub update_Id: i32,
    pub collection: String,
    pub collection_Id: String,
    pub quantity: f32,
    pub profit: f32,
    pub is_refund: bool,
    pub terminal: i32,
    pub entity_id: String,
}
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ProductUpdate {
    pub entity_id: String,
    pub _id: Option<String>,
    pub change_type: Option<String>,
    pub origin: Option<String>,
    pub createdAt: Option<String>,
    pub updatedAt: Option<String>,
    pub update_id: Option<i32>,
    pub category: Option<String>,
    pub product_id: Option<String>,
    pub quantity: f32,
    pub profit: Option<f32>,
    pub total_profit: Option<f32>,
    pub quantity_sold: Option<f32>,
    pub reorderLimit: Option<f32>,
    pub cost_price: Option<f32>,
    pub sell_price: Option<f32>,
    pub name: Option<String>,
    pub expiry_date: Option<String>,
    pub barcode: Option<String>,
    pub isArchived: Option<bool>,
}
impl FromRow<'_, SqliteRow> for ProductUpdate {
    fn from_row(row: &SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            name: row.try_get("name")?,
            quantity: row.try_get("quantity")?,
            barcode: row.try_get("barcode")?,
            reorderLimit: row.try_get("reorderLimit")?,
            sell_price: row.try_get("sell_price")?,
            cost_price: row.try_get("cost_price")?,
            expiry_date: row.try_get("expiry_date")?,
            total_profit: row.try_get("total_profit")?,
            quantity_sold: row.try_get("quantity_sold")?,
            createdAt: row.try_get("createdAt")?,
            updatedAt: row.try_get("updatedAt")?,
            profit: row.try_get("profit")?,
            product_id: row.try_get("product_id")?,
            change_type: row.try_get("change_type")?,
            entity_id: row.try_get("entity_id")?,
            origin: row.try_get("origin")?,
            update_id: row.try_get("update_id")?,
            isArchived: row.try_get("is_archived")?,
            category: row.try_get("category")?,

            _id: row.try_get("_id")?,
            // Add more fields as needed
        })
    }
}
impl FromIterator<ProductUpdate> for Option<Vec<ProductUpdate>> {
    fn from_iter<I: IntoIterator<Item = ProductUpdate>>(iter: I) -> Self {
        let vec: Vec<ProductUpdate> = iter.into_iter().collect();
        Some(vec)
    }
}
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct TransactionUpdate {
    entity_id: String,
    id: i32,
    transaction_id: i32,
    origin: String,
    createdAt: String,
    cloud_id: String,
}
impl FromRow<'_, SqliteRow> for TransactionUpdate {
    fn from_row(row: &SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            createdAt: row.try_get("createdAt")?,
            id: row.try_get("id")?,
            transaction_id: row.try_get("transaction_id")?,
            entity_id: row.try_get("entity_id")?,
            origin: row.try_get("origin")?,
            cloud_id: row.try_get("cloud_id")?,
            // Add more fields as needed
        })
    }
}
impl FromIterator<TransactionUpdate> for Option<Vec<TransactionUpdate>> {
    fn from_iter<I: IntoIterator<Item = TransactionUpdate>>(iter: I) -> Self {
        let vec: Vec<TransactionUpdate> = iter.into_iter().collect();
        Some(vec)
    }
}
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct TransactionUpdateDetails {
    product_id: String,
    quantity: f32,
    profit: f32,
    discount: f32,
    total: f32,
}
impl FromRow<'_, SqliteRow> for TransactionUpdateDetails {
    fn from_row(row: &SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            profit: row.try_get("profit")?,
            total: row.try_get("total")?,
            discount: row.try_get("discount")?,
            product_id: row.try_get("product_id")?,
            quantity: row.try_get("quantity")?,
            // Add more fields as needed
        })
    }
}
impl FromIterator<TransactionUpdateDetails> for Option<Vec<TransactionUpdateDetails>> {
    fn from_iter<I: IntoIterator<Item = TransactionUpdateDetails>>(iter: I) -> Self {
        let vec: Vec<TransactionUpdateDetails> = iter.into_iter().collect();
        Some(vec)
    }
}
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct SynchronizationStruct {
    pub productUpdates: Vec<ProductUpdate>,
    pub transactionUpdates: Vec<TransactionUpdate>,
    pub transactions: Vec<Transaction>,
    pub entity_id: String,
    pub terminal: String,
}
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct CloudStruct {
    pub productUpdates: Vec<ProductUpdate>,
    pub transactions: Vec<Transaction>,
    pub entity: Entity,
}
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct SyncInfo {
    pub localUpdates: i32,
    pub cloudUpdates: i32,
    pub status: bool,
    pub message: String,
    pub entity: Entity,
}

#[derive(Clone, Deserialize, Debug, Serialize, sqlx::FromRow)]
pub struct Customer {
    pub firstName: String,
    pub lastName: String,
    pub fullName: String,
    pub phone: String,
    pub customerId: i32,
    pub wallet: f32,
    pub totalRevenue: f32,
}

#[derive(Clone, Deserialize, Debug, Serialize, sqlx::FromRow)]
pub struct Payment {
    pub name: String,
    pub value: f32,
    pub transactionId: Option<i32>,
    pub payment_method_id: String,
}
