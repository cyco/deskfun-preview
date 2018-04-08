use super::zone::Zone;
use my_byte_order::ByteOrderExt;
use std::io;

pub enum HotspotType {
    TriggerLocation,
    SpawnLocation,
    ForceLocation,
    VehicleTo,
    VehicleBack,
    LocatorThingy,
    CrateItem,
    PuzzleNPC,
    CrateWeapon,
    DoorIn,
    DoorOut,
    UnknownIndyOnly,
    Lock,
    Teleporter,
    xWingFromD,
    xWingToD,
}

pub struct Hotspot {
    pub hotspot_type: HotspotType,
    pub x: i16,
    pub y: i16,
    pub enabled: bool,
    pub argument: i16,
}

impl From<u32> for HotspotType {
    fn from(n: u32) -> Self {
        match n {
            0 => HotspotType::TriggerLocation,
            1 => HotspotType::SpawnLocation,
            2 => HotspotType::ForceLocation,
            3 => HotspotType::VehicleTo,
            4 => HotspotType::VehicleBack,
            5 => HotspotType::LocatorThingy,
            6 => HotspotType::CrateItem,
            7 => HotspotType::PuzzleNPC,
            8 => HotspotType::CrateWeapon,
            9 => HotspotType::DoorIn,
            10 => HotspotType::DoorOut,
            11 => HotspotType::UnknownIndyOnly,
            12 => HotspotType::Lock,
            13 => HotspotType::Teleporter,
            14 => HotspotType::xWingFromD,
            15 => HotspotType::xWingToD,
            _ => panic!("Invalid hotspot type {} given.", n),
        }
    }
}

pub trait ReadHotspotExt: io::Read {
    fn read_hotspot(&mut self) -> io::Result<Hotspot> {
        let hotspot_type = HotspotType::from(self.read_u32_le()?);

        let x = self.read_i16_le()?;
        let y = self.read_i16_le()?;
        let enabled = self.read_u16_le()? != 0;
        let argument = self.read_i16_le()?;

        Ok(Hotspot {
            hotspot_type: hotspot_type,
            x: x,
            y: y,
            enabled: enabled,
            argument: argument,
        })
    }

    fn read_hotspots(&mut self, zones: &mut Vec<Zone>) -> io::Result<()> {
        let _size = self.read_u32_le()? as usize;

        loop {
            let zone_id = self.read_i16_le()?;
            if zone_id == -1 {
                break;
            }

            let count = self.read_u16_le()?;
            let mut hotspots = Vec::with_capacity(count as usize);
            for _ in 0..count {
                hotspots.push(self.read_hotspot()?);
            }

            zones[zone_id as usize].hotspots = hotspots;
        }

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadHotspotExt for R {}
