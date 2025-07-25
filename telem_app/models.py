# This is an auto-generated Django model module.
# You'll have to do the following manually to clean this up:
#   * Rearrange models' order
#   * Make sure each model has one field with primary_key=True
#   * Make sure each ForeignKey and OneToOneField has `on_delete` set to the desired behavior
#   * Remove `managed = False` lines if you wish to allow Django to create, modify, and delete the table
# Feel free to rename the models, but don't rename db_table values or field names.
from django.db import models
from django.utils import timezone


class DieselSchemaMigrations(models.Model):
    version = models.CharField(primary_key=True, max_length=50)
    run_on = models.DateTimeField()

    class Meta:
        managed = False
        db_table = '__diesel_schema_migrations'


class AuthGroup(models.Model):
    name = models.CharField(unique=True, max_length=150)

    class Meta:
        managed = False
        db_table = 'auth_group'


class AuthGroupPermissions(models.Model):
    id = models.BigAutoField(primary_key=True)
    group = models.ForeignKey(AuthGroup, models.DO_NOTHING)
    permission = models.ForeignKey('AuthPermission', models.DO_NOTHING)

    class Meta:
        managed = False
        db_table = 'auth_group_permissions'
        unique_together = (('group', 'permission'),)


class AuthPermission(models.Model):
    name = models.CharField(max_length=255)
    content_type = models.ForeignKey('DjangoContentType', models.DO_NOTHING)
    codename = models.CharField(max_length=100)

    class Meta:
        managed = False
        db_table = 'auth_permission'
        unique_together = (('content_type', 'codename'),)


class AuthUser(models.Model):
    password = models.CharField(max_length=128)
    last_login = models.DateTimeField(blank=True, null=True)
    is_superuser = models.BooleanField()
    username = models.CharField(unique=True, max_length=150)
    first_name = models.CharField(max_length=150)
    last_name = models.CharField(max_length=150)
    email = models.CharField(max_length=254)
    is_staff = models.BooleanField()
    is_active = models.BooleanField()
    date_joined = models.DateTimeField()

    class Meta:
        managed = False
        db_table = 'auth_user'


class AuthUserGroups(models.Model):
    id = models.BigAutoField(primary_key=True)
    user = models.ForeignKey(AuthUser, models.DO_NOTHING)
    group = models.ForeignKey(AuthGroup, models.DO_NOTHING)

    class Meta:
        managed = False
        db_table = 'auth_user_groups'
        unique_together = (('user', 'group'),)


class AuthUserUserPermissions(models.Model):
    id = models.BigAutoField(primary_key=True)
    user = models.ForeignKey(AuthUser, models.DO_NOTHING)
    permission = models.ForeignKey(AuthPermission, models.DO_NOTHING)

    class Meta:
        managed = False
        db_table = 'auth_user_user_permissions'
        unique_together = (('user', 'permission'),)


class DjangoAdminLog(models.Model):
    action_time = models.DateTimeField()
    object_id = models.TextField(blank=True, null=True)
    object_repr = models.CharField(max_length=200)
    action_flag = models.SmallIntegerField()
    change_message = models.TextField()
    content_type = models.ForeignKey('DjangoContentType', models.DO_NOTHING, blank=True, null=True)
    user = models.ForeignKey(AuthUser, models.DO_NOTHING)

    class Meta:
        managed = False
        db_table = 'django_admin_log'


class DjangoContentType(models.Model):
    app_label = models.CharField(max_length=100)
    model = models.CharField(max_length=100)

    class Meta:
        managed = False
        db_table = 'django_content_type'
        unique_together = (('app_label', 'model'),)


class DjangoMigrations(models.Model):
    id = models.BigAutoField(primary_key=True)
    app = models.CharField(max_length=255)
    name = models.CharField(max_length=255)
    applied = models.DateTimeField()

    class Meta:
        managed = False
        db_table = 'django_migrations'


class DjangoSession(models.Model):
    session_key = models.CharField(primary_key=True, max_length=40)
    session_data = models.TextField()
    expire_date = models.DateTimeField()

    class Meta:
        managed = False
        db_table = 'django_session'


class TblCarTlm(models.Model):
    id = models.BigIntegerField()
    time = models.DateTimeField(primary_key=True, default=timezone.now)
    speed_kph = models.SmallIntegerField()
    throttle = models.FloatField()
    steer = models.FloatField()
    brake = models.FloatField()
    clutch = models.SmallIntegerField()
    gear = models.SmallIntegerField()
    engine_rpm = models.SmallIntegerField()
    drs = models.SmallIntegerField()
    rev_light_percent = models.SmallIntegerField()
    brake_1_temp = models.SmallIntegerField()
    brake_2_temp = models.SmallIntegerField()
    brake_3_temp = models.SmallIntegerField()
    brake_4_temp = models.SmallIntegerField()
    tyre_1_surface_temp = models.SmallIntegerField()
    tyre_2_surface_temp = models.SmallIntegerField()
    tyre_3_surface_temp = models.SmallIntegerField()
    tyre_4_surface_temp = models.SmallIntegerField()
    tyre_1_inner_temp = models.SmallIntegerField()
    tyre_2_inner_temp = models.SmallIntegerField()
    tyre_3_inner_temp = models.SmallIntegerField()
    tyre_4_inner_temp = models.SmallIntegerField()
    engine_temp = models.SmallIntegerField()
    tyre_1_pressure = models.FloatField()
    tyre_2_pressure = models.FloatField()
    tyre_3_pressure = models.FloatField()
    tyre_4_pressure = models.FloatField()
    tyre_1_surf_type = models.SmallIntegerField()
    tyre_2_surf_type = models.SmallIntegerField()
    tyre_3_surf_type = models.SmallIntegerField()
    tyre_4_surf_type = models.SmallIntegerField()

    class Meta:
        managed = False
        db_table = 'tbl_car_tlm'


class TblHeaderPkts(models.Model):
    id = models.BigIntegerField()
    time = models.DateTimeField(primary_key=True,default=timezone.now)
    packet_format = models.SmallIntegerField()
    maj_version = models.SmallIntegerField()
    min_version = models.SmallIntegerField()
    packet_version = models.SmallIntegerField()
    packet_id = models.SmallIntegerField()
    session_uid = models.BigIntegerField()
    session_time = models.FloatField()
    frame_id = models.IntegerField()
    player_car_idx = models.SmallIntegerField()
    sec_player_car_id = models.SmallIntegerField()

    class Meta:
        managed = False
        db_table = 'tbl_header_pkts'


class TblLapData(models.Model):
    id = models.BigIntegerField()
    time = models.DateTimeField(primary_key=True, default=timezone.now)
    last_lap_time = models.FloatField()
    curr_lap_time = models.FloatField()
    sector_1_time_ms = models.SmallIntegerField()
    sector_2_time_ms = models.SmallIntegerField()
    best_lap_time = models.FloatField()
    best_lap_num = models.SmallIntegerField()
    best_lap_s1_ms = models.SmallIntegerField()
    best_lap_s2_ms = models.SmallIntegerField()
    best_lap_s3_ms = models.SmallIntegerField()
    best_ovr_s1_ms = models.SmallIntegerField()
    best_ovr_s1_lap = models.SmallIntegerField()
    best_ovr_s2_ms = models.SmallIntegerField()
    best_ovr_s2_lap = models.SmallIntegerField()
    best_ovr_s3_ms = models.SmallIntegerField()
    best_ovr_s3_lap = models.SmallIntegerField()
    lap_distance = models.FloatField()
    total_distance = models.FloatField()
    safety_car_delta = models.FloatField()
    car_position = models.SmallIntegerField()
    curr_lap_num = models.SmallIntegerField()
    pit_status = models.SmallIntegerField()
    sector = models.SmallIntegerField()
    curr_lap_invalid = models.SmallIntegerField()
    penalties = models.SmallIntegerField()
    grid_postion = models.SmallIntegerField()
    driver_status = models.SmallIntegerField()
    result_status = models.SmallIntegerField()

    class Meta:
        managed = False
        db_table = 'tbl_lap_data'


class TblMotionData(models.Model):
    id = models.BigIntegerField()
    time = models.DateTimeField(primary_key=True, default=timezone.now)
    pos_x = models.FloatField()
    pos_y = models.FloatField()
    pos_z = models.FloatField()
    vel_x = models.FloatField()
    vel_y = models.FloatField()
    vel_z = models.FloatField()
    world_forward_x = models.SmallIntegerField()
    world_forward_y = models.SmallIntegerField()
    world_forward_z = models.SmallIntegerField()
    world_right_x = models.SmallIntegerField()
    world_right_y = models.SmallIntegerField()
    world_right_z = models.SmallIntegerField()
    lat_g_force = models.FloatField()
    long_g_force = models.FloatField()
    vert_g_force = models.FloatField()
    yaw_rads = models.FloatField()
    pitch_rads = models.FloatField()
    roll_rads = models.FloatField()

    class Meta:
        managed = False
        db_table = 'tbl_motion_data'


class TblTest(models.Model):
    data = models.IntegerField()

    class Meta:
        managed = False
        db_table = 'tbl_test'
