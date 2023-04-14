use pipeline_rs::pipes::*;
use pipeline_rs::plumber::Plumber;

fn main() {
    let mut reservoir = WaterTank {
        content: WaterUnit {
            volume: 10.0,
            temperature: 20.0,
        },
    };

    let pump = move || reservoir.drain(1.0);

    let heater = |water: WaterUnit| WaterUnit {
        volume: water.volume,
        temperature: water.temperature + 10.0,
    };

    let mut tap = Plumber::from_mut_source(pump)
        .with_transformer(heater)
        .build();

    loop {
        let water = tap.recv_mut();
        dbg!(&water);

        if water.volume == 0.0 {
            break;
        }
    }
}

#[derive(Debug)]
struct WaterTank {
    content: WaterUnit,
}

impl WaterTank {
    fn drain(&mut self, volume: f32) -> WaterUnit {
        let drain_volume = self.content.volume.min(volume);
        self.content.volume -= drain_volume;

        WaterUnit {
            volume: drain_volume,
            temperature: self.content.temperature,
        }
    }
}

#[derive(Debug, Clone)]
struct WaterUnit {
    volume: f32,
    temperature: f32,
}
