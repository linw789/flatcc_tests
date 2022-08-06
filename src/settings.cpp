#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>
#include <settings_builder.h>
#include <Windows.h>

typedef uint32_t (*pfn_send_current_values_t)(uint8_t* buffer, uint32_t buffer_size);
typedef uint32_t (*pfn_get_user_overrides_t)(uint8_t* buffer, uint32_t buffer_size);

int main(int argc, char** argv)
{
    flatcc_builder_t builder;
    flatcc_builder_init(&builder);

    DDSettings_Rpc_SettingsValuesAll_start_as_root(&builder);
    DDSettings_Rpc_SettingsValuesAll_components_start(&builder);

    {
        DDSettings_Rpc_SettingsValue_vec_start(&builder);

        {
            bool b_val = false;
            flatbuffers_uint8_vec_ref_t value1 = flatbuffers_uint8_vec_create(
                &builder,
                (uint8_t*)&b_val,
                sizeof(b_val));

            DDSettings_Rpc_SettingsValue_vec_push_create(
                &builder,
                1,
                DDSettings_Rpc_SettingsType_Bool,
                sizeof(b_val),
                value1);
        }

        {
            uint64_t u64_val = 11111;
            flatbuffers_uint8_vec_ref_t value2 = flatbuffers_uint8_vec_create(
                &builder,
                (uint8_t*)&u64_val,
                sizeof(u64_val));

            DDSettings_Rpc_SettingsValue_vec_push_create(
                &builder,
                2,
                DDSettings_Rpc_SettingsType_Uint64,
                sizeof(u64_val),
                value2);
        }

        DDSettings_Rpc_SettingsValue_vec_ref_t values_vec =
            DDSettings_Rpc_SettingsValue_vec_end(&builder);

        flatbuffers_ref_t component_name = flatbuffers_string_create_str(&builder, "PalPlatform");

        DDSettings_Rpc_SettingsValuesAll_components_push_create(
            &builder,
            component_name,
            1,
            values_vec);
    }

    {
        DDSettings_Rpc_SettingsValue_vec_start(&builder);

        {
            float f_val = 0.25f;
            flatbuffers_uint8_vec_ref_t value1 = flatbuffers_uint8_vec_create(
                &builder,
                (uint8_t*)&f_val,
                sizeof(f_val));

            DDSettings_Rpc_SettingsValue_vec_push_create(
                &builder,
                1,
                DDSettings_Rpc_SettingsType_Float,
                sizeof(f_val),
                value1);
        }

        {
            char s_val[] = "c:\\dummy\\";
            flatbuffers_uint8_vec_ref_t value2 = flatbuffers_uint8_vec_create(
                &builder,
                (uint8_t*)s_val,
                // size include null-terminator
                sizeof(s_val));

            DDSettings_Rpc_SettingsValue_vec_push_create(
                &builder,
                2,
                DDSettings_Rpc_SettingsType_String,
                sizeof(s_val),
                value2);
        }

        DDSettings_Rpc_SettingsValue_vec_ref_t values_vec =
            DDSettings_Rpc_SettingsValue_vec_end(&builder);

        flatbuffers_ref_t component_name = flatbuffers_string_create_str(&builder, "Pal");

        DDSettings_Rpc_SettingsValuesAll_components_push_create(
            &builder,
            component_name,
            2,
            values_vec);
    }

    DDSettings_Rpc_SettingsValuesAll_components_end(&builder);
    DDSettings_Rpc_SettingsValuesAll_end_as_root(&builder);

    size_t buffer_size = 0;
    void* buffer = flatcc_builder_finalize_buffer(&builder, &buffer_size);
    assert(buffer_size != 0);

    HMODULE settings_rs_dll = LoadLibraryA("C:\\projects\\flatcc\\rust\\target\\debug\\settings_rs.dll");
    assert(settings_rs_dll != NULL);

    // pfn_send_current_values_t send_current_values =
    //     (pfn_send_current_values_t)GetProcAddress(settings_rs_dll, "get_current_values");
    // assert(send_current_values != nullptr);

    pfn_get_user_overrides_t get_user_overrides =
        (pfn_get_user_overrides_t)GetProcAddress(settings_rs_dll, "get_user_overrides");
    assert(get_user_overrides != nullptr);

    uint8_t* user_overrides_buffer = nullptr;
    uint32_t user_overrides_buffer_size = get_user_overrides(user_overrides_buffer, 0);
    assert(user_overrides_buffer_size > 0);

    user_overrides_buffer = (uint8_t*)malloc(user_overrides_buffer_size);
    user_overrides_buffer_size =
        get_user_overrides(user_overrides_buffer, user_overrides_buffer_size);

    printf("user overrides buffer size: %u", user_overrides_buffer_size);

    return 0;
}