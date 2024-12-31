pub const TABLE_FONT_SIZE_COUNT: usize = 7;
pub const TABLE_CONTENT_FONT_SIZES: [f32; TABLE_FONT_SIZE_COUNT] =
    [6.0, 6.0, 8.0, 12.0, 16.0, 16.0, 16.0];
pub const TABLE_CONTENT_ROW_HEIGHTS: [f32; TABLE_FONT_SIZE_COUNT] =
    [7.0, 8.0, 10.0, 12.0, 16.0, 20.0, 24.0];
pub const TABLE_CONTENT_TEE_SIZES: [f32; TABLE_FONT_SIZE_COUNT] = TABLE_CONTENT_ROW_HEIGHTS;
pub const TABLE_CONTENT_COLUMN_SPACING: [f32; TABLE_FONT_SIZE_COUNT] =
    [2.0, 2.0, 3.0, 8.0, 8.0, 8.0, 8.0];

pub const TABLE_CONTENT_WIDTH: [[f32; 6]; TABLE_FONT_SIZE_COUNT] = [
    [30.0, 8.0, 60.0, 40.0, 20.0, 24.0],
    [30.0, 8.0, 100.0, 70.0, 20.0, 24.0],
    [35.0, 15.0, 120.0, 80.0, 30.0, 30.0],
    [35.0, 15.0, 180.0, 120.0, 40.0, 30.0],
    [35.0, 18.0, 240.0, 160.0, 40.0, 40.0],
    [35.0, 22.0, 300.0, 200.0, 40.0, 60.0],
    [35.0, 26.0, 360.0, 240.0, 50.0, 60.0],
];
pub const TABLE_NAME_COLUMN_INDEX: usize = 2;

pub const TABLE_CONTENT_MIN_COLUMNS: usize = 3;