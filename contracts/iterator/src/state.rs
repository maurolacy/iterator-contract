use cw_storage_plus::Map;

pub(crate) const MAP: Map<&str, bool> = Map::new("map");
