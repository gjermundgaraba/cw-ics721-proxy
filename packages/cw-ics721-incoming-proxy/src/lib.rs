use cw_paginate_storage::paginate_map_keys;
use cw_storage_plus::{Item, Map};

use cosmwasm_std::{Addr, Api, Deps, IbcPacket, MessageInfo, Order, Response, StdError, StdResult, Storage};
use ics721_types::ibc_types::NonFungibleTokenPacketData;
use thiserror::Error;

const ORIGIN: Item<Addr> = Item::new("origin");
const CHANNELS: Map<String, String> = Map::new("channels");
const CLASS_IDS: Map<String, String> = Map::new("class_ids");

#[derive(Error, Debug, PartialEq)]
pub enum IncomingProxyError {
    #[error(transparent)]
    Std(#[from] StdError),

    #[error("Unauthorized channel: {0}")]
    UnauthorizedChannel(String),
    
    #[error("Unauthorized class id: {0}")]
    UnauthorizedClassId(String),

    #[error("Sender is not origin contract: {0}")]
    UnauthorizedOrigin(String),
}

pub trait IncomingProxyExecute {
    fn initialize(
        &self,
        storage: &mut dyn Storage,
        api: &dyn Api,
        origin: Option<String>,
        channels: Option<Vec<String>>,
        class_ids: Option<Vec<String>>,
    ) -> StdResult<()> {
        if let Some(origin) = origin {
            ORIGIN.save(storage, &api.addr_validate(&origin)?)?;
        }
        if let Some(channels) = channels {
            for channel in channels {
                CHANNELS.save(storage, channel.clone(), &channel)?;
            }
        }
        if let Some(class_ids) = class_ids {
            for class_id in class_ids {
                CLASS_IDS.save(storage, class_id.clone(), &class_id)?;
            }
        }
        Ok(())
    }

    fn execute_ics721_receive_packet_msg<T>(
        &self,
        storage: &mut dyn Storage,
        info: &MessageInfo,
        packet: IbcPacket,
        data: NonFungibleTokenPacketData,
    ) -> Result<Response<T>, IncomingProxyError> {
        self.assert_origin(storage, info.sender.to_string())?;
        self.assert_channel(storage, packet)?;
        self.assert_class_id(storage, data)?;
        Ok(Response::default()
            .add_attribute("method", "execute")
            .add_attribute("action", "ics721_receive_packet_msg"))
    }

    fn assert_origin(
        &self,
        storage: &dyn Storage,
        sender: String,
    ) -> Result<(), IncomingProxyError> {
        if let Some(origin) = ORIGIN.may_load(storage)? {
            if origin == sender {
                return Ok(());
            }
        }
        Err(IncomingProxyError::UnauthorizedOrigin(sender))
    }

    fn assert_channel(
        &self,
        storage: &dyn Storage,
        packet: IbcPacket,
    ) -> Result<(), IncomingProxyError> {
        if CHANNELS.has(storage, packet.dest.channel_id.clone()) {
            return Ok(());
        }
        Err(IncomingProxyError::UnauthorizedChannel(
            packet.dest.channel_id,
        ))
    }
    
    fn assert_class_id(
        &self,
        storage: &dyn Storage,
        data: NonFungibleTokenPacketData,
    ) -> Result<(), IncomingProxyError> {
        let class_id = data.class_id.to_string();
        if CLASS_IDS.is_empty(storage) || CLASS_IDS.has(storage, class_id.clone()) {
            return Ok(());
        }
        return Err(IncomingProxyError::UnauthorizedClassId(
            class_id,
        ));
    }

    fn migrate(
        &mut self,
        storage: &mut dyn Storage,
        api: &dyn Api,
        origin: Option<String>,
        channels: Option<Vec<String>>,
        class_ids: Option<Vec<String>>,
    ) -> StdResult<Response> {
        if let Some(origin) = origin.clone() {
            ORIGIN.save(storage, &api.addr_validate(&origin)?)?;
        }
        if let Some(channels) = channels.clone() {
            CHANNELS.clear(storage);
            for channel in channels {
                CHANNELS.save(storage, channel.clone(), &channel)?;
            }
        }
        if let Some(class_ids) = class_ids.clone() {
            CLASS_IDS.clear(storage);
            for class_id in class_ids {
                CLASS_IDS.save(storage, class_id.clone(), &class_id)?;
            }
        }
        Ok(Response::default()
            .add_attribute("method", "migrate")
            .add_attribute("origin", origin.unwrap_or("not migrated".to_string()))
            .add_attribute(
                "channels",
                channels.map_or("not migrated".to_string(), |v| v.join(",")),
            ))
    }
}

pub trait IncomingProxyQuery {
    fn get_channels(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<Vec<String>> {
        paginate_map_keys(deps, &CHANNELS, start_after, limit, Order::Ascending)
    }

    fn get_origin(&self, storage: &dyn Storage) -> StdResult<Option<Addr>> {
        ORIGIN.may_load(storage)
    }
    
    fn get_class_ids(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<Vec<String>> {
        paginate_map_keys(deps, &CLASS_IDS, start_after, limit, Order::Ascending)
    }
}

#[cfg(test)]
mod tests;
