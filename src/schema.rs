// @generated automatically by Diesel CLI.

diesel::table! {
    tbl_car_tlm (time) {
        id -> Int8,
        time -> Timestamptz,
        speed_kph -> Int2,
        throttle -> Float4,
        steer -> Float4,
        brake -> Float4,
        clutch -> Int2,
        gear -> Int2,
        engine_rpm -> Int2,
        drs -> Int2,
        rev_light_percent -> Int2,
        brake_1_temp -> Int2,
        brake_2_temp -> Int2,
        brake_3_temp -> Int2,
        brake_4_temp -> Int2,
        tyre_1_surface_temp -> Int2,
        tyre_2_surface_temp -> Int2,
        tyre_3_surface_temp -> Int2,
        tyre_4_surface_temp -> Int2,
        tyre_1_inner_temp -> Int2,
        tyre_2_inner_temp -> Int2,
        tyre_3_inner_temp -> Int2,
        tyre_4_inner_temp -> Int2,
        engine_temp -> Int2,
        tyre_1_pressure -> Int2,
        tyre_2_pressure -> Int2,
        tyre_3_pressure -> Int2,
        tyre_4_pressure -> Int2,
        tyre_1_surf_type -> Int2,
        tyre_2_surf_type -> Int2,
        tyre_3_surf_type -> Int2,
        tyre_4_surf_type -> Int2,
    }
}

diesel::table! {
    tbl_test (id) {
        id -> Int4,
        data -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tbl_car_tlm,
    tbl_test,
);
