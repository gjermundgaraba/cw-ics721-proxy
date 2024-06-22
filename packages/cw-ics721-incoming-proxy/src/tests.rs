use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    testing::mock_dependencies, to_json_binary, IbcEndpoint, IbcTimeout, Timestamp,
};
use ics721_types::token_types::ClassId;

use super::*;

#[cw_serde]
#[derive(Default)]
pub struct IncomingProxyContract {}

impl IncomingProxyExecute for IncomingProxyContract {}
impl IncomingProxyQuery for IncomingProxyContract {}

#[test]
fn test_assert_origin() {
    let mut deps = mock_dependencies();
    let error = IncomingProxyContract::default()
        .assert_origin(deps.as_ref().storage, "sender".to_string())
        .unwrap_err();
    assert_eq!(
        error,
        IncomingProxyError::UnauthorizedOrigin("sender".to_string())
    );

    ORIGIN
        .save(deps.as_mut().storage, &Addr::unchecked("sender"))
        .unwrap();
    IncomingProxyContract::default()
        .assert_origin(deps.as_ref().storage, "sender".to_string())
        .unwrap();
}

#[test]
fn test_assert_packet_data_channel() {
    let mut deps = mock_dependencies();
    let data = NonFungibleTokenPacketData {
        class_id: ClassId::new("class-0".to_string()),
        class_uri: None,
        token_data: None,
        token_ids: vec![],
        token_uris: None,
        sender: "sender".to_string(),
        receiver: "receiver".to_string(),
        class_data: None,
        memo: None,
    };
    let packet = IbcPacket::new(
        to_json_binary(&data).unwrap(),
        IbcEndpoint {
            port_id: "port-0".to_string(),
            channel_id: "channel-0".to_string(),
        },
        IbcEndpoint {
            port_id: "port-1".to_string(),
            channel_id: "channel-1".to_string(),
        },
        0,
        IbcTimeout::with_timestamp(Timestamp::from_seconds(0)),
    );

    let error = IncomingProxyContract::default()
        .assert_channel(deps.as_ref().storage, packet.clone())
        .unwrap_err();
    assert_eq!(
        error,
        IncomingProxyError::UnauthorizedChannel("channel-1".to_string())
    );

    CHANNELS
        .save(
            deps.as_mut().storage,
            "channel-1".to_string(),
            &"channel-1".to_string(),
        )
        .unwrap();
    IncomingProxyContract::default()
        .assert_channel(deps.as_ref().storage, packet)
        .unwrap();
}

#[test]
fn test_assert_packet_class_id() {
    let mut deps = mock_dependencies();
    CHANNELS
        .save(
            deps.as_mut().storage,
            "channel-1".to_string(),
            &"channel-1".to_string(),
        )
        .unwrap();

    let data = NonFungibleTokenPacketData {
        class_id: ClassId::new("class-0".to_string()),
        class_uri: None,
        token_data: None,
        token_ids: vec![],
        token_uris: None,
        sender: "sender".to_string(),
        receiver: "receiver".to_string(),
        class_data: None,
        memo: None,
    };

    CLASS_IDS.save(
        deps.as_mut().storage,
        "class-0".to_string(),
        &"class-0".to_string(),
    ).unwrap();

    IncomingProxyContract::default()
        .assert_class_id(deps.as_ref().storage, data.clone())
        .unwrap();

    CLASS_IDS.clear(deps.as_mut().storage);
    CLASS_IDS.save(
        deps.as_mut().storage,
        "class-1".to_string(),
        &"class-1".to_string(),
    ).unwrap();

    let error = IncomingProxyContract::default()
        .assert_class_id(deps.as_ref().storage, data)
        .unwrap_err();
    assert_eq!(
        error,
        IncomingProxyError::UnauthorizedClassId("class-0".to_string())
    );
}
