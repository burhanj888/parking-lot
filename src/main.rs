use std::collections::HashMap;

/// Parking Spot Type
#[derive(Debug, PartialEq, Eq, Hash)]
enum SpotType {
    Motorcycle,
    Car,
    Large,
}
/// Vehicle Type
#[derive(Debug, PartialEq)]
enum VehicleType {
    Motorcycle,
    Car,
    Van,
}

/// Individual Parking Spot
#[derive(Debug)]
struct ParkingSpot {
    spot_type: SpotType,
    is_occupied: bool,
}

/// Parking Lot
#[derive(Debug)]
struct ParkingLot {
    spots: Vec<ParkingSpot>,
    total_spots: HashMap<SpotType, usize>,
    occupied_spots: HashMap<SpotType, usize>,
}

impl ParkingLot {
    /// Creates a new parking lot with specified spot counts
    fn new(motorcycle_spots: usize, car_spots: usize, large_spots: usize) -> Self {
        let mut spots = Vec::new();

        // Add motorcycle spots
        for _ in 0..motorcycle_spots {
            spots.push(ParkingSpot {
                spot_type: SpotType::Motorcycle,
                is_occupied: false,
            });
        }

        // Add car spots
        for _ in 0..car_spots {
            spots.push(ParkingSpot {
                spot_type: SpotType::Car,
                is_occupied: false,
            });
        }

        // Add large spots
        for _ in 0..large_spots {
            spots.push(ParkingSpot {
                spot_type: SpotType::Large,
                is_occupied: false,
            });
        }

        ParkingLot {
            spots,
            total_spots: HashMap::from([
                (SpotType::Motorcycle, motorcycle_spots),
                (SpotType::Car, car_spots),
                (SpotType::Large, large_spots),
            ]),
            occupied_spots: HashMap::from([
                (SpotType::Motorcycle, 0),
                (SpotType::Car, 0),
                (SpotType::Large, 0),
            ]),
        }
    }

    /// Parks a vehicle in the appropriate spot
    fn park_vehicle(&mut self, vehicle: VehicleType) -> bool {
        match vehicle {
            VehicleType::Motorcycle => {
                self.park_in_first_available(SpotType::Motorcycle)
                    || self.park_in_first_available(SpotType::Car)
                    || self.park_in_first_available(SpotType::Large)
            }
            VehicleType::Car => {
                self.park_in_first_available(SpotType::Car)
                    || self.park_in_first_available(SpotType::Large)
            }
            VehicleType::Van => self.park_van(),
        }
    }

    /// Finds and parks in the first available spot of the specified type
    fn park_in_first_available(&mut self, spot_type: SpotType) -> bool {
        for spot in self.spots.iter_mut() {
            if !spot.is_occupied && spot.spot_type == spot_type {
                spot.is_occupied = true;
                *self.occupied_spots.get_mut(&spot_type).unwrap() += 1;
                return true;
            }
        }
        false
    }

    /// Parks a van, which requires 3 consecutive large spots
    fn park_van(&mut self) -> bool {
        let mut large_spot_indices = vec![];

        for (i, spot) in self.spots.iter().enumerate() {
            if !spot.is_occupied && spot.spot_type == SpotType::Large {
                large_spot_indices.push(i);
                if large_spot_indices.len() == 3 {
                    break;
                }
            } else {
                large_spot_indices.clear();
            }
        }

        if large_spot_indices.len() == 3 {
            for &i in &large_spot_indices {
                self.spots[i].is_occupied = true;
            }
            *self.occupied_spots.get_mut(&SpotType::Large).unwrap() += 3;
            return true;
        }
        false
    }

    /// Returns the number of remaining spots of a specific type
    fn remaining_spots(&self) -> usize {
        self.spots.len() - self.spots.iter().filter(|spot| spot.is_occupied).count()
    }

    /// Returns true if the parking lot is full
    fn is_full(&self) -> bool {
        self.remaining_spots() == 0
    }

    /// Returns true if the parking lot is empty
    fn is_empty(&self) -> bool {
        self.spots.iter().all(|spot| !spot.is_occupied)
    }
}

fn main() {
    let mut parking_lot = ParkingLot::new(5, 10, 3);

    println!("Total spots: {:?}", parking_lot.total_spots);
    println!("Is parking lot empty? {}", parking_lot.is_empty());

    parking_lot.park_vehicle(VehicleType::Motorcycle);
    parking_lot.park_vehicle(VehicleType::Car);
    parking_lot.park_vehicle(VehicleType::Van);

    println!("Remaining spots: {}", parking_lot.remaining_spots());
    println!("Is parking lot full? {}", parking_lot.is_full());
}
