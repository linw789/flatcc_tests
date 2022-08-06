extern crate flatbuffers;

mod settings_generated;

use settings_generated::ddsettings::rpc::{
    SettingsComponentValues, SettingsComponentValuesArgs, SettingsType, SettingsValue,
    SettingsValueArgs, SettingsValuesAll, SettingsValuesAllArgs,
};
use std::{mem::transmute, ptr};

#[no_mangle]
pub extern "C" fn get_user_overrides(recv_buffer: *mut u8, revc_buffer_size: u32) -> u32 {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    let mut components = Vec::new();

    {
        let mut values = Vec::new();

        {
            let b_val: bool = true;
            let b_val_bytes: [u8; 1] = unsafe { transmute::<bool, [u8; 1]>(b_val) };
            let value = builder.create_vector(&b_val_bytes);

            let setting_value = SettingsValue::create(
                &mut builder,
                &SettingsValueArgs {
                    hash: 1,
                    type_: SettingsType::Bool,
                    size_: 1,
                    value: Some(value),
                },
            );

            values.push(setting_value);
        }

        {
            let u64_val: u64 = 11111;
            let u64_val_bytes: [u8; 8] = unsafe { transmute::<u64, [u8; 8]>(u64_val) };
            let value = builder.create_vector(&u64_val_bytes);

            let setting_value = SettingsValue::create(
                &mut builder,
                &SettingsValueArgs {
                    hash: 2,
                    type_: SettingsType::Bool,
                    size_: 1,
                    value: Some(value),
                },
            );

            values.push(setting_value);
        }

        let values = builder.create_vector(&values);

        let component_name = builder.create_string("PalPlatform");
        let component_values = SettingsComponentValues::create(
            &mut builder,
            &SettingsComponentValuesArgs {
                component_name: Some(component_name),
                component_hash: 1,
                values: Some(values),
            },
        );

        components.push(component_values);
    }

    {
        let mut values = Vec::new();

        {
            let s_val = "//temp/log".to_owned();
            let s_val_bytes = &s_val.as_bytes();
            let value = builder.create_vector(s_val_bytes);

            let setting_value = SettingsValue::create(
                &mut builder,
                &SettingsValueArgs {
                    hash: 1,
                    type_: SettingsType::Bool,
                    size_: s_val_bytes.len() as u32,
                    value: Some(value),
                },
            );

            values.push(setting_value);
        }

        let values = builder.create_vector(&values);

        let component_name = builder.create_string("PalPlatform");
        let component_values = SettingsComponentValues::create(
            &mut builder,
            &SettingsComponentValuesArgs {
                component_name: Some(component_name),
                component_hash: 1,
                values: Some(values),
            },
        );

        components.push(component_values);
    }

    let components = builder.create_vector(&components);

    let settings_values_all = SettingsValuesAll::create(
        &mut builder,
        &SettingsValuesAllArgs {
            components: Some(components),
        },
    );

    builder.finish(settings_values_all, None);

    let buf = builder.finished_data();
    let buf_len = buf.len() as u32;

    if !recv_buffer.is_null() && buf_len <= revc_buffer_size {
        unsafe {
            ptr::copy_nonoverlapping(buf.as_ptr(), recv_buffer, buf.len());
        }
    }

    buf_len
}
