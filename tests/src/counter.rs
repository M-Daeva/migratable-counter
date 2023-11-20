use cosmwasm_std::{Addr, StdResult};

use speculoos::assert_that;

use counter_base::{constants::CHAIN_ID_DEV, counter::types::ActionType};

use crate::helpers::{
    counter::CounterExtension,
    suite::{
        core::{assert_error, Project},
        types::{ProjectAccount, ProjectCoin},
    },
};

#[test]
fn create_update_reset() -> StdResult<()> {
    let mut project = Project::new(Some(CHAIN_ID_DEV));

    let res = project
        .counter_try_update_counter(ProjectAccount::Alice, ActionType::Add, 5)
        .unwrap_err();

    assert_error(&res, "not found");

    project.counter_try_create_counter(ProjectAccount::Alice, 1, ProjectCoin::Denom)?;

    project.counter_try_update_counter(ProjectAccount::Alice, ActionType::Add, 5)?;

    let (_, counter) = project
        .counter_query_counters()?
        .into_iter()
        .find(|(address, _)| address == &Into::<Addr>::into(ProjectAccount::Alice))
        .unwrap();

    assert_that(&counter.u128()).is_equal_to(5);

    project.counter_try_reset_counter(ProjectAccount::Alice)?;

    let (_, counter) = project
        .counter_query_counters()?
        .into_iter()
        .find(|(address, _)| address == &Into::<Addr>::into(ProjectAccount::Alice))
        .unwrap();

    assert_that(&counter.u128()).is_equal_to(0);

    Ok(())
}
