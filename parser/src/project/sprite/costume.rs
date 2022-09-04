use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Costume {
    pub name: String,
    pub bitmap_resolution: i32,
    pub data_format: String,
    pub asset_id: String,
    pub md5ext: String,
    pub rotation_center_x: i32,
    pub rotation_center_y: i32,
}
