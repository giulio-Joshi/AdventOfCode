use std::collections::HashMap;
use std::iter::Map;
use std::{collections::HashSet, fs, io::ErrorKind, str::FromStr};
use std::slice::Iter;

fn main() {
    let file_content = fs::read_to_string("day19/input.txt").expect("Can't find input");

    let scanners: Vec<Scanner> = Scanner::parse_scanners(&file_content);

    println!("found {} scanners", scanners.len());
}

#[derive(Debug)]
enum Face {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

impl Face {
    fn iterator() -> Iter<'static, Face> {
        static FACES : [Face; 6] = [Face::Up, Face::Down, Face::Left, Face::Right, Face::Front,Face::Back];
        FACES.iter()
    }

    fn orient( &self, pos: &Position) -> Position {

        match self {
            Up => { Position{ x: pos.x , y: pos.y , z: pos.z} },
            Down => { Position{ x: pos.x , y: pos.y , z: - pos.z} },
            Left => { Position{ x: pos.x , y: -pos.y , z: pos.z} },
            Right => { Position{ x: -pos.x , y: -pos.y , z: pos.z} },
            Front => { Position{ x: -pos.x , y: pos.y , z: pos.z} },
            Back => { Position{ x: pos.x , y: pos.y , z: pos.z} },
        }
    }
}

#[derive(Debug)]
enum Rotation {
    Snake, // snake is flat, 0°
    Sheep, // Sheep is 90° by default
    Stars, // Stars are 180°
    Ouch,  // cracked spine at 270°
}

impl Rotation {
    fn iterator() -> Iter<'static, Rotation> {
        static ROTATIONS : [Rotation; 4] = [Rotation::Snake,Rotation::Sheep, Rotation::Stars, Rotation::Ouch];
        ROTATIONS.iter()
    }
}

#[derive(Debug)]
struct Orientation {
    face: Face,
    rotation: Rotation,
}


impl Orientation {
    fn new(face: Face, rotation: Rotation) -> Self {
        Orientation { face, rotation }
    }

    /*fn orient( &self, position: &Position) -> Position {

        //match self.rotation 

    }*/
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn new(coords: &[i32]) -> Self {
        Position {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }

    // calculate all "delta distances"
    fn delta(&self, other: &Position) -> HashSet<Position> {
        let mut possible = HashSet::<Position>::new();
        possible.insert(Position {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        });
        possible.insert(Position {
            x: self.x - other.y,
            y: self.y - other.z,
            z: self.z - other.x,
        });
        possible.insert(Position {
            x: self.x - other.z,
            y: self.y - other.x,
            z: self.z - other.y,
        });

        possible
    }

    fn distance(&self, other: &Position) -> f32 {
        ((self.x - other.x).pow(2) as f32
            + (self.y - other.y).pow(2) as f32
            + (self.z - other.z).pow(2) as f32)
            .sqrt()
    }
}

impl FromStr for Position {
    type Err = ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<i32> = s.split(',').map(i32::from_str).flatten().collect();
        if coords.len() == 3 {
            return Ok(Position::new(&coords));
        }

        Err(ErrorKind::Unsupported)
    }
}

#[derive(Debug)]
struct Scanner {
    id: u16,
    position: Option<Position>,
    orientation: Option<Orientation>,
    beacons: Vec<Position>,
    permutations: Option<HashMap<Orientation, Vec<Position>>>
}

impl FromStr for Scanner {
    type Err = ErrorKind;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.lines().count() < 3 {
            return Err(ErrorKind::Unsupported);
        }

        let id = input.lines().next().unwrap()[0..2]
            .trim()
            .parse::<u16>()
            .unwrap();

        let beacons = input
            .lines()
            .skip(1)
            .map(Position::from_str)
            .flatten()
            .collect::<Vec<Position>>();

        Ok(Scanner {
            id,
            orientation: None,
            position: None,
            beacons,
            permutations: None
        })
    }
}

impl Scanner {
    // Reads list of scanners and beacons from input
    fn parse_scanners(input: &str) -> Vec<Scanner> {
        let mut scanners: Vec<Scanner> = input
            .split("--- scanner ")
            .map(Scanner::from_str)
            .flatten()
            .collect();

        // Setup scanner 0 at 0 position
        if scanners.len() > 1 {
            scanners[0].position = Some(Position::new(&[0, 0, 0]));
            scanners[0].orientation = Some(Orientation::new(Face::Up, Rotation::Snake));
        } else {
            scanners
                .iter_mut()
                .skip(1)
                .for_each(Scanner::spin);

        }

        scanners
    }


    fn spin(&mut self){

        if self.orientation.is_some() {
            return;
        }

        
        
    }

    fn overlaps(&self, other: &Scanner) -> Vec<Position> {
        let mut found: Vec<Position> = vec![];

        found
    }
}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn test_convert() {
        let scanners: Vec<Scanner> = Scanner::parse_scanners(two_scanners());
        println!("{:?}", scanners[0].overlaps(&scanners[1]));
        assert_eq!(5, scanners.len());
    }

    fn two_scanners() -> &'static str {
        r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"#
    }
}
