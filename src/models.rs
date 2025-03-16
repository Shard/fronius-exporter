use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct FroniusBatterManagementReadableChannels {
    #[serde(alias = "BAT_VALUE_STATE_OF_CHARGE_RELATIVE_U16")]
    fronius_bat_value: Value,
    #[serde(alias = "BAT_VOLTAGE_DC_INTERNAL_F64")]
    fronius_bat_voltage: Value,
    #[serde(alias = "BAT_VALUE_STATE_OF_HEALTH_RELATIVE_U16")]
    fronius_bat_health: Value,
    #[serde(alias = "DEVICE_TEMPERATURE_AMBIENTEMEAN_F32")]
    fronius_temp_bat_ambient: Value,
    #[serde(alias = "BAT_TEMPERATURE_CELL_F64")]
    fronius_temp_bat_cell: Value,
    #[serde(alias = "BAT_ENERGYACTIVE_LIFETIME_CHARGED_F64")]
    fronius_bat_energy_charged: Value,
    #[serde(alias = "BAT_ENERGYACTIVE_LIFETIME_DISCHARGED_F64")]
    fronius_bat_energy_discharged: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FroniusCacheReadableChannels {
    #[serde(alias = "ACBRIDGE_POWERACTIVE_SUM_MEAN_F32")]
    fronius_power_gen: Value,
    #[serde(alias = "ACBRIDGE_POWERAPPARENT_SUM_MEAN_F32")]
    fronius_power_apparent: Value,
    #[serde(alias = "ACBRIDGE_POWERREACTIVE_SUM_MEAN_F32")]
    fronius_power_reactive: Value,
    #[serde(alias = "DEVICE_TEMPERATURE_AMBIENTEMEAN_F32")]
    fronius_temp_inverter: Value,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FroniusPowerflowReadableBody {
    #[serde(alias = "BackupMode")]
    fronius_backup_mode: Option<Value>,
    #[serde(alias = "P_Akku")]
    fronius_power_akku: Option<Value>,
    #[serde(alias = "P_Grid")]
    fronius_power_grid: Option<Value>,
    #[serde(alias = "P_Load")]
    fronius_power_load: Option<Value>,
    #[serde(alias = "P_PV")]
    fronius_power_pv: Option<Value>,
    #[serde(alias = "rel_Autonomy")]
    fronius_autonomy: Option<Value>,
    #[serde(alias = "rel_SelfConsumption")]
    fronius_self_consumption: Option<Value>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FroniusPowerflow2ReadableBody {
    #[serde(alias = "P")]
    fronius_p: Value,
    #[serde(alias = "E_Total")]
    fronius_e_total: Option<Value>,
}

pub struct Metrics {
    pub powerflow: FroniusPowerflowReadableBody,
    pub powerflow2: FroniusPowerflow2ReadableBody,
    pub cache: FroniusCacheReadableChannels,
    pub batman: Option<FroniusBatterManagementReadableChannels>,
}
