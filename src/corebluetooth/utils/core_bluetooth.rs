// btleplug Source Code File
//
// Copyright 2020 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.
//
// Some portions of this file are taken and/or modified from blurmac
// (https://github.com/servo/devices), using a BSD 3-Clause license under the
// following copyright:
//
// Copyright (c) 2017 Akos Kiss.
//
// Licensed under the BSD 3-Clause License
// <LICENSE.md or https://opensource.org/licenses/BSD-3-Clause>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use objc::runtime::Object;
use uuid::Uuid;

use super::super::framework::{cb, nil, ns};
use super::nsstring::nsstring_to_string;

/// Convert a CBUUID object to the standard Uuid type.
pub fn cbuuid_to_uuid(cbuuid: *mut Object) -> Uuid {
    // NOTE: CoreBluetooth tends to return uppercase UUID strings, and only 4
    // character long if the UUID is short (16 bits). It can also return 8
    // character strings if the rest of the UUID matches the generic UUID.
    let uuid = nsstring_to_string(cb::uuid_uuidstring(cbuuid)).unwrap();
    let long = if uuid.len() == 4 {
        format!("0000{}-0000-1000-8000-00805f9b34fb", uuid)
    } else if uuid.len() == 8 {
        format!("{}-0000-1000-8000-00805f9b34fb", uuid)
    } else {
        uuid
    };
    let uuid_string = long.to_lowercase();
    uuid_string.parse().unwrap()
}

pub fn peripheral_debug(peripheral: *mut Object) -> String {
    if peripheral == nil {
        return String::from("nil");
    }
    let name = nsstring_to_string(cb::peripheral_name(peripheral));
    let uuid = nsstring_to_string(ns::uuid_uuidstring(cb::peer_identifier(peripheral))).unwrap();
    if let Some(name) = name {
        format!("CBPeripheral({}, {})", name, uuid)
    } else {
        format!("CBPeripheral({})", uuid)
    }
}

pub fn service_debug(service: *mut Object) -> String {
    if service == nil {
        return String::from("nil");
    }
    let uuid = cb::uuid_uuidstring(cb::attribute_uuid(service));
    format!("CBService({})", nsstring_to_string(uuid).unwrap())
}

pub fn characteristic_debug(characteristic: *mut Object) -> String {
    if characteristic == nil {
        return String::from("nil");
    }
    let uuid = cb::uuid_uuidstring(cb::attribute_uuid(characteristic));
    format!("CBCharacteristic({})", nsstring_to_string(uuid).unwrap())
}

#[cfg(test)]
mod tests {
    use super::super::nsstring::str_to_nsstring;
    use objc::runtime::Class;
    use objc::{msg_send, sel, sel_impl};

    use super::*;

    #[test]
    fn parse_uuid_short() {
        let uuid_string = "1234";
        let uuid_nsstring = str_to_nsstring(uuid_string);
        let cbuuid: *mut Object =
            unsafe { msg_send![Class::get("CBUUID").unwrap(), UUIDWithString: uuid_nsstring] };
        let uuid = cbuuid_to_uuid(cbuuid);
        assert_eq!(
            uuid,
            Uuid::from_u128(0x00001234_0000_1000_8000_00805f9b34fb)
        );
    }

    #[test]
    fn parse_uuid_long() {
        let uuid_nsstring = str_to_nsstring("12345678-0000-1111-2222-333344445555");
        let cbuuid: *mut Object =
            unsafe { msg_send![Class::get("CBUUID").unwrap(), UUIDWithString: uuid_nsstring] };
        let uuid = cbuuid_to_uuid(cbuuid);
        assert_eq!(
            uuid,
            Uuid::from_u128(0x12345678_0000_1111_2222_333344445555)
        );
    }
}
