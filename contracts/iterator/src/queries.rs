use crate::msg::MapEntriesResponse;
use crate::state::MAP;
use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::Bound;

// Settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

pub fn map_values(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<MapEntriesResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start_after = start_after.as_ref().map(|s| Bound::exclusive(&**s));
    let values = MAP
        .range_raw(deps.storage, start_after, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<Vec<bool>>>()?;
    Ok(MapEntriesResponse { values })
}

#[cfg(test)]
mod tests {
    use crate::contract::instantiate;
    use cosmwasm_std::testing::mock_info;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    use crate::msg::InstantiateMsg;

    const CREATOR: &str = "creator";

    #[test]
    fn test_map_values() {
        let mut deps = mock_dependencies();
        let info = mock_info(CREATOR, &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), InstantiateMsg {}).unwrap();

        let values = crate::queries::map_values(deps.as_ref(), None, None)
            .unwrap()
            .values;
        assert_eq!(values.len(), 2);
        assert!(values[0]);
        assert!(!values[1]);

        // Query map with limit
        let values = crate::queries::map_values(deps.as_ref(), None, Some(1))
            .unwrap()
            .values;
        assert_eq!(values.len(), 1);
        assert!(values[0]);

        // Query values with start_after
        let values = crate::queries::map_values(deps.as_ref(), Some("1".to_string()), None)
            .unwrap()
            .values;
        assert_eq!(values.len(), 1);
        assert!(!values[0]);
    }
}
