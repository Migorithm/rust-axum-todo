use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use uuid::Uuid;

use crate::models::Todo;

pub(crate) type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;
