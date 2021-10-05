use std::collections::HashMap;

#[cfg(test)]
mod tests;

// Silakan kontributor baru coba: 
// 1. Tambahkan 2 konten: BottomRightWidthHeight 
//    dan LeftBottomWidthHeight
// 2. Urutan leksikografis(A ke Z)
enum PositionType {
    TopLeftBottomRight,
    TopLeftWidthHeight,
    TopRightWidthHeight,
    TopLeftBottomWidth,
    TopBottomRightWidth,
    TopLeftRightHeight,
    LeftBottomRightHeight,
}

impl PositionType {
    #[inline]
    pub fn to_rect(&self, width: u32, height: u32, param1: u32, 
    param2: u32, param3: u32, param4: u32) -> (u32, u32, u32, u32) {
        match self {
            PositionType::TopLeftBottomRight => (param2, param1, width - param4, height - param3),
            PositionType::TopLeftWidthHeight => (param2, param1, param2 + param3, param1 + param4),
            // Silakan kontributor baru coba: 
            // Returnan ialah tupel(atau tuple dalam Inggris) yang dibuat oleh x1, y1, x2 dan y2
            // x1 dan y1 adalah koordinat ujung kiri atas, 
            // x2 dan y2 adalah koordinat ujung kanan bawah
            // hapus #[ignore] untuk fungsi yang bernama 
            // full_test_for_type_positiontype di mod tests
            _ => (0, 0, 0, 0)
        }
    }
}

enum ComponentType {
    Block,
    Custom,
    RoundedBlock(u32),
    // Yang tipenya boolean adalah apakah teks ditulis 
    // dari kanan ke kiri, seperti bahasa Arab
    Text(String, bool),
}

#[derive(Hash, Eq, PartialEq)]
enum Event {
    Click,
    Enter,
    Out,
    Move,
    Focus,
    Blur,
}

struct Page {
}

struct Component {
    name: String,
    component_type: ComponentType,
    position: (PositionType, u32, u32, u32, u32),
    events: HashMap<Event, String>
}
