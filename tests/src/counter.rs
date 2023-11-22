use cosmwasm_std::{coin, Addr, StdResult, Uint128};

use speculoos::assert_that;

use counter_base::{
    constants::CHAIN_ID_DEV, counter::types::ActionType, counter_new::msg::QueryCountersResponse,
};

use crate::helpers::{
    counter::CounterExtension,
    counter_new::CounterNewExtension,
    suite::{
        codes::WithCodes,
        core::Project,
        types::{ProjectAccount, ProjectCoin},
    },
};

#[test]
fn migrate_default() -> StdResult<()> {
    let mut project = Project::new(Some(CHAIN_ID_DEV));

    project.counter_try_create_counter(ProjectAccount::Alice, Some((10, ProjectCoin::Denom)))?;

    project.counter_try_update_counter(ProjectAccount::Alice, ActionType::Add, 5)?;

    let (_, counter) = project
        .counter_query_counters()?
        .into_iter()
        .find(|(address, _)| address == Into::<Addr>::into(ProjectAccount::Alice))
        .unwrap();

    let total_calls = project.counter_query_total_calls()?;

    assert_that(&counter.u128()).is_equal_to(5);
    assert_that(&total_calls.u128()).is_equal_to(2);

    project.migrate_counter_new(
        ProjectAccount::Admin,
        project.get_counter_address(),
        project.get_counter_new_code_id(),
        counter_base::counter_new::msg::MigrateMsg::V2_0_0,
    )?;

    project.counter_try_create_counter(ProjectAccount::Bob, Some((10, ProjectCoin::Denom)))?;
    project.counter_new_try_set_counter(ProjectAccount::Bob, 6)?;
    project.counter_new_try_update_counter(
        ProjectAccount::Bob,
        counter_base::counter_new::types::ActionType::Mul,
        7,
    )?;

    let counters = project.counter_new_query_counters(None)?;
    let total_calls = project.counter_query_total_calls()?;
    let total_calls_previous = project.counter_query_total_calls_previous()?;
    let balances = project
        .app
        .wrap()
        .query_all_balances(project.get_counter_address())?;

    assert_that(&counters).is_equal_to(vec![
        QueryCountersResponse {
            owner: ProjectAccount::Alice.into(),
            counter_value: Uint128::new(5),
        },
        QueryCountersResponse {
            owner: ProjectAccount::Bob.into(),
            counter_value: Uint128::new(42),
        },
    ]);
    assert_that(&total_calls.u128()).is_equal_to(3);
    assert_that(&total_calls_previous.u128()).is_equal_to(2);
    assert_that(&balances).is_equal_to(vec![coin(20, ProjectCoin::Denom.to_string())]);

    Ok(())
}
