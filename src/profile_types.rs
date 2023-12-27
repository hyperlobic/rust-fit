/// Auto-generated, do not edit.
use num_enum::FromPrimitive;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum File {
    Device = 1,
    Settings = 2,
    Sport = 3,
    Activity = 4,
    Workout = 5,
    Course = 6,
    Schedules = 7,
    Weight = 9,
    Totals = 10,
    Goals = 11,
    BloodPressure = 14,
    MonitoringA = 15,
    ActivitySummary = 20,
    MonitoringDaily = 28,
    MonitoringB = 32,
    Segment = 34,
    SegmentList = 35,
    ExdConfiguration = 40,
    MfgRangeMin = 0xF7,
    MfgRangeMax = 0xFe,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u16)]
#[derive(Debug, FromPrimitive)]
pub enum MesgNum {
    FileId = 0,
    Capabilities = 1,
    DeviceSettings = 2,
    UserProfile = 3,
    HrmProfile = 4,
    SdmProfile = 5,
    BikeProfile = 6,
    ZonesTarget = 7,
    HrZone = 8,
    PowerZone = 9,
    MetZone = 10,
    Sport = 12,
    Goal = 15,
    Session = 18,
    Lap = 19,
    Record = 20,
    Event = 21,
    DeviceInfo = 23,
    Workout = 26,
    WorkoutStep = 27,
    Schedule = 28,
    WeightScale = 30,
    Course = 31,
    CoursePoint = 32,
    Totals = 33,
    Activity = 34,
    Software = 35,
    FileCapabilities = 37,
    MesgCapabilities = 38,
    FieldCapabilities = 39,
    FileCreator = 49,
    BloodPressure = 51,
    SpeedZone = 53,
    Monitoring = 55,
    TrainingFile = 72,
    Hrv = 78,
    AntRx = 80,
    AntTx = 81,
    AntChannelId = 82,
    Length = 101,
    MonitoringInfo = 103,
    Pad = 105,
    SlaveDevice = 106,
    Connectivity = 127,
    WeatherConditions = 128,
    WeatherAlert = 129,
    CadenceZone = 131,
    Hr = 132,
    SegmentLap = 142,
    MemoGlob = 145,
    SegmentId = 148,
    SegmentLeaderboardEntry = 149,
    SegmentPoint = 150,
    SegmentFile = 151,
    WorkoutSession = 158,
    WatchfaceSettings = 159,
    GpsMetadata = 160,
    CameraEvent = 161,
    TimestampCorrelation = 162,
    GyroscopeData = 164,
    AccelerometerData = 165,
    ThreeDSensorCalibration = 167,
    VideoFrame = 169,
    ObdiiData = 174,
    NmeaSentence = 177,
    AviationAttitude = 178,
    Video = 184,
    VideoTitle = 185,
    VideoDescription = 186,
    VideoClip = 187,
    OhrSettings = 188,
    ExdScreenConfiguration = 200,
    ExdDataFieldConfiguration = 201,
    ExdDataConceptConfiguration = 202,
    FieldDescription = 206,
    DeveloperDataId = 207,
    MagnetometerData = 208,
    BarometerData = 209,
    OneDSensorCalibration = 210,
    MonitoringHrData = 211,
    TimeInZone = 216,
    Set = 225,
    StressLevel = 227,
    MaxMetData = 229,
    DiveSettings = 258,
    DiveGas = 259,
    DiveAlarm = 262,
    ExerciseTitle = 264,
    DiveSummary = 268,
    Spo2Data = 269,
    SleepLevel = 275,
    Jump = 285,
    BeatIntervals = 290,
    RespirationRate = 297,
    Split = 312,
    SplitSummary = 313,
    ClimbPro = 317,
    TankUpdate = 319,
    TankSummary = 323,
    SleepAssessment = 346,
    HrvStatusSummary = 370,
    HrvValue = 371,
    DeviceAuxBatteryInfo = 375,
    DiveApneaAlarm = 393,
    MfgRangeMin = 0xFf00,
    MfgRangeMax = 0xFffe,
    #[num_enum(catch_all)]
    UnknownValue(u16),
}
pub type Checksum = u8;
pub type FileFlags = u8;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum MesgCount {
    NumPerFile = 0,
    MaxPerFile = 1,
    MaxPerFileType = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type DateTime = u32;
pub type LocalDateTime = u32;
pub type MessageIndex = u16;
pub type DeviceIndex = u8;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Gender {
    Female = 0,
    Male = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Language {
    English = 0,
    French = 1,
    Italian = 2,
    German = 3,
    Spanish = 4,
    Croatian = 5,
    Czech = 6,
    Danish = 7,
    Dutch = 8,
    Finnish = 9,
    Greek = 10,
    Hungarian = 11,
    Norwegian = 12,
    Polish = 13,
    Portuguese = 14,
    Slovakian = 15,
    Slovenian = 16,
    Swedish = 17,
    Russian = 18,
    Turkish = 19,
    Latvian = 20,
    Ukrainian = 21,
    Arabic = 22,
    Farsi = 23,
    Bulgarian = 24,
    Romanian = 25,
    Chinese = 26,
    Japanese = 27,
    Korean = 28,
    Taiwanese = 29,
    Thai = 30,
    Hebrew = 31,
    BrazilianPortuguese = 32,
    Indonesian = 33,
    Malaysian = 34,
    Vietnamese = 35,
    Burmese = 36,
    Mongolian = 37,
    Custom = 254,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type LanguageBits0 = u8;
pub type LanguageBits1 = u8;
pub type LanguageBits2 = u8;
pub type LanguageBits3 = u8;
pub type LanguageBits4 = u8;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum TimeZone {
    Almaty = 0,
    Bangkok = 1,
    Bombay = 2,
    Brasilia = 3,
    Cairo = 4,
    CapeVerdeIs = 5,
    Darwin = 6,
    Eniwetok = 7,
    Fiji = 8,
    HongKong = 9,
    Islamabad = 10,
    Kabul = 11,
    Magadan = 12,
    MidAtlantic = 13,
    Moscow = 14,
    Muscat = 15,
    Newfoundland = 16,
    Samoa = 17,
    Sydney = 18,
    Tehran = 19,
    Tokyo = 20,
    UsAlaska = 21,
    UsAtlantic = 22,
    UsCentral = 23,
    UsEastern = 24,
    UsHawaii = 25,
    UsMountain = 26,
    UsPacific = 27,
    Other = 28,
    Auckland = 29,
    Kathmandu = 30,
    EuropeWesternWet = 31,
    EuropeCentralCet = 32,
    EuropeEasternEet = 33,
    Jakarta = 34,
    Perth = 35,
    Adelaide = 36,
    Brisbane = 37,
    Tasmania = 38,
    Iceland = 39,
    Amsterdam = 40,
    Athens = 41,
    Barcelona = 42,
    Berlin = 43,
    Brussels = 44,
    Budapest = 45,
    Copenhagen = 46,
    Dublin = 47,
    Helsinki = 48,
    Lisbon = 49,
    London = 50,
    Madrid = 51,
    Munich = 52,
    Oslo = 53,
    Paris = 54,
    Prague = 55,
    Reykjavik = 56,
    Rome = 57,
    Stockholm = 58,
    Vienna = 59,
    Warsaw = 60,
    Zurich = 61,
    Quebec = 62,
    Ontario = 63,
    Manitoba = 64,
    Saskatchewan = 65,
    Alberta = 66,
    BritishColumbia = 67,
    Boise = 68,
    Boston = 69,
    Chicago = 70,
    Dallas = 71,
    Denver = 72,
    KansasCity = 73,
    LasVegas = 74,
    LosAngeles = 75,
    Miami = 76,
    Minneapolis = 77,
    NewYork = 78,
    NewOrleans = 79,
    Phoenix = 80,
    SantaFe = 81,
    Seattle = 82,
    WashingtonDc = 83,
    UsArizona = 84,
    Chita = 85,
    Ekaterinburg = 86,
    Irkutsk = 87,
    Kaliningrad = 88,
    Krasnoyarsk = 89,
    Novosibirsk = 90,
    PetropavlovskKamchatskiy = 91,
    Samara = 92,
    Vladivostok = 93,
    MexicoCentral = 94,
    MexicoMountain = 95,
    MexicoPacific = 96,
    CapeTown = 97,
    Winkhoek = 98,
    Lagos = 99,
    Riyahd = 100,
    Venezuela = 101,
    AustraliaLh = 102,
    Santiago = 103,
    Manual = 253,
    Automatic = 254,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum DisplayMeasure {
    Metric = 0,
    Statute = 1,
    Nautical = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum DisplayHeart {
    Bpm = 0,
    Max = 1,
    Reserve = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum DisplayPower {
    Watts = 0,
    PercentFtp = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum DisplayPosition {
    Degree = 0,
    DegreeMinute = 1,
    DegreeMinuteSecond = 2,
    AustrianGrid = 3,
    BritishGrid = 4,
    DutchGrid = 5,
    HungarianGrid = 6,
    FinnishGrid = 7,
    GermanGrid = 8,
    IcelandicGrid = 9,
    IndonesianEquatorial = 10,
    IndonesianIrian = 11,
    IndonesianSouthern = 12,
    IndiaZone0 = 13,
    IndiaZoneIa = 14,
    IndiaZoneIb = 15,
    IndiaZoneIia = 16,
    IndiaZoneIib = 17,
    IndiaZoneIiia = 18,
    IndiaZoneIiib = 19,
    IndiaZoneIva = 20,
    IndiaZoneIvb = 21,
    IrishTransverse = 22,
    IrishGrid = 23,
    Loran = 24,
    MaidenheadGrid = 25,
    MgrsGrid = 26,
    NewZealandGrid = 27,
    NewZealandTransverse = 28,
    QatarGrid = 29,
    ModifiedSwedishGrid = 30,
    SwedishGrid = 31,
    SouthAfricanGrid = 32,
    SwissGrid = 33,
    TaiwanGrid = 34,
    UnitedStatesGrid = 35,
    UtmUpsGrid = 36,
    WestMalayan = 37,
    BorneoRso = 38,
    EstonianGrid = 39,
    LatvianGrid = 40,
    SwedishRef99Grid = 41,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Switch {
    Off = 0,
    On = 1,
    Auto = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Sport {
    Generic = 0,
    Running = 1,
    Cycling = 2,
    Transition = 3,
    FitnessEquipment = 4,
    Swimming = 5,
    Basketball = 6,
    Soccer = 7,
    Tennis = 8,
    AmericanFootball = 9,
    Training = 10,
    Walking = 11,
    CrossCountrySkiing = 12,
    AlpineSkiing = 13,
    Snowboarding = 14,
    Rowing = 15,
    Mountaineering = 16,
    Hiking = 17,
    Multisport = 18,
    Paddling = 19,
    Flying = 20,
    EBiking = 21,
    Motorcycling = 22,
    Boating = 23,
    Driving = 24,
    Golf = 25,
    HangGliding = 26,
    HorsebackRiding = 27,
    Hunting = 28,
    Fishing = 29,
    InlineSkating = 30,
    RockClimbing = 31,
    Sailing = 32,
    IceSkating = 33,
    SkyDiving = 34,
    Snowshoeing = 35,
    Snowmobiling = 36,
    StandUpPaddleboarding = 37,
    Surfing = 38,
    Wakeboarding = 39,
    WaterSkiing = 40,
    Kayaking = 41,
    Rafting = 42,
    Windsurfing = 43,
    Kitesurfing = 44,
    Tactical = 45,
    Jumpmaster = 46,
    Boxing = 47,
    FloorClimbing = 48,
    Diving = 53,
    Hiit = 62,
    Racket = 64,
    WheelchairPushWalk = 65,
    WheelchairPushRun = 66,
    Meditation = 67,
    WaterTubing = 76,
    Wakesurfing = 77,
    All = 254,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type SportBits0 = u8;
pub type SportBits1 = u8;
pub type SportBits2 = u8;
pub type SportBits3 = u8;
pub type SportBits4 = u8;
pub type SportBits5 = u8;
pub type SportBits6 = u8;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum SubSport {
    Generic = 0,
    Treadmill = 1,
    Street = 2,
    Trail = 3,
    Track = 4,
    Spin = 5,
    IndoorCycling = 6,
    Road = 7,
    Mountain = 8,
    Downhill = 9,
    Recumbent = 10,
    Cyclocross = 11,
    HandCycling = 12,
    TrackCycling = 13,
    IndoorRowing = 14,
    Elliptical = 15,
    StairClimbing = 16,
    LapSwimming = 17,
    OpenWater = 18,
    FlexibilityTraining = 19,
    StrengthTraining = 20,
    WarmUp = 21,
    Match = 22,
    Exercise = 23,
    Challenge = 24,
    IndoorSkiing = 25,
    CardioTraining = 26,
    IndoorWalking = 27,
    EBikeFitness = 28,
    Bmx = 29,
    CasualWalking = 30,
    SpeedWalking = 31,
    BikeToRunTransition = 32,
    RunToBikeTransition = 33,
    SwimToBikeTransition = 34,
    Atv = 35,
    Motocross = 36,
    Backcountry = 37,
    Resort = 38,
    RcDrone = 39,
    Wingsuit = 40,
    Whitewater = 41,
    SkateSkiing = 42,
    Yoga = 43,
    Pilates = 44,
    IndoorRunning = 45,
    GravelCycling = 46,
    EBikeMountain = 47,
    Commuting = 48,
    MixedSurface = 49,
    Navigate = 50,
    TrackMe = 51,
    Map = 52,
    SingleGasDiving = 53,
    MultiGasDiving = 54,
    GaugeDiving = 55,
    ApneaDiving = 56,
    ApneaHunting = 57,
    VirtualActivity = 58,
    Obstacle = 59,
    Breathing = 62,
    SailRace = 65,
    Ultra = 67,
    IndoorClimbing = 68,
    Bouldering = 69,
    Hiit = 70,
    Amrap = 73,
    Emom = 74,
    Tabata = 75,
    Pickleball = 84,
    Padel = 85,
    IndoorWheelchairWalk = 86,
    IndoorWheelchairRun = 87,
    IndoorHandCycling = 88,
    FlyCanopy = 110,
    FlyParaglide = 111,
    FlyParamotor = 112,
    FlyPressurized = 113,
    FlyNavigate = 114,
    FlyTimer = 115,
    FlyAltimeter = 116,
    FlyWx = 117,
    FlyVfr = 118,
    FlyIfr = 119,
    All = 254,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum SportEvent {
    Uncategorized = 0,
    Geocaching = 1,
    Fitness = 2,
    Recreation = 3,
    Race = 4,
    SpecialEvent = 5,
    Training = 6,
    Transportation = 7,
    Touring = 8,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Activity {
    Manual = 0,
    AutoMultiSport = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Intensity {
    Active = 0,
    Rest = 1,
    Warmup = 2,
    Cooldown = 3,
    Recovery = 4,
    Interval = 5,
    Other = 6,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum SessionTrigger {
    ActivityEnd = 0,
    Manual = 1,
    AutoMultiSport = 2,
    FitnessEquipment = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum AutolapTrigger {
    Time = 0,
    Distance = 1,
    PositionStart = 2,
    PositionLap = 3,
    PositionWaypoint = 4,
    PositionMarked = 5,
    Off = 6,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum LapTrigger {
    Manual = 0,
    Time = 1,
    Distance = 2,
    PositionStart = 3,
    PositionLap = 4,
    PositionWaypoint = 5,
    PositionMarked = 6,
    SessionEnd = 7,
    FitnessEquipment = 8,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum TimeMode {
    Hour12 = 0,
    Hour24 = 1,
    Military = 2,
    Hour12WithSeconds = 3,
    Hour24WithSeconds = 4,
    Utc = 5,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum BacklightMode {
    Off = 0,
    Manual = 1,
    KeyAndMessages = 2,
    AutoBrightness = 3,
    SmartNotifications = 4,
    KeyAndMessagesNight = 5,
    KeyAndMessagesAndSmartNotifications = 6,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum DateMode {
    DayMonth = 0,
    MonthDay = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type BacklightTimeout = u8;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Event {
    Timer = 0,
    Workout = 3,
    WorkoutStep = 4,
    PowerDown = 5,
    PowerUp = 6,
    OffCourse = 7,
    Session = 8,
    Lap = 9,
    CoursePoint = 10,
    Battery = 11,
    VirtualPartnerPace = 12,
    HrHighAlert = 13,
    HrLowAlert = 14,
    SpeedHighAlert = 15,
    SpeedLowAlert = 16,
    CadHighAlert = 17,
    CadLowAlert = 18,
    PowerHighAlert = 19,
    PowerLowAlert = 20,
    RecoveryHr = 21,
    BatteryLow = 22,
    TimeDurationAlert = 23,
    DistanceDurationAlert = 24,
    CalorieDurationAlert = 25,
    Activity = 26,
    FitnessEquipment = 27,
    Length = 28,
    UserMarker = 32,
    SportPoint = 33,
    Calibration = 36,
    FrontGearChange = 42,
    RearGearChange = 43,
    RiderPositionChange = 44,
    ElevHighAlert = 45,
    ElevLowAlert = 46,
    CommTimeout = 47,
    AutoActivityDetect = 54,
    DiveAlert = 56,
    DiveGasSwitched = 57,
    TankPressureReserve = 71,
    TankPressureCritical = 72,
    TankLost = 73,
    RadarThreatAlert = 75,
    TankBatteryLow = 76,
    TankPodConnected = 81,
    TankPodDisconnected = 82,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum EventType {
    Start = 0,
    Stop = 1,
    ConsecutiveDepreciated = 2,
    Marker = 3,
    StopAll = 4,
    BeginDepreciated = 5,
    EndDepreciated = 6,
    EndAllDepreciated = 7,
    StopDisable = 8,
    StopDisableAll = 9,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum TimerTrigger {
    Manual = 0,
    Auto = 1,
    FitnessEquipment = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum FitnessEquipmentState {
    Ready = 0,
    InUse = 1,
    Paused = 2,
    Unknown = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Tone {
    Off = 0,
    Tone = 1,
    Vibrate = 2,
    ToneAndVibrate = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Autoscroll {
    None = 0,
    Slow = 1,
    Medium = 2,
    Fast = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum ActivityClass {
    Level = 0x7F,
    LevelMax = 100,
    Athlete = 0x80,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum HrZoneCalc {
    Custom = 0,
    PercentMaxHr = 1,
    PercentHrr = 2,
    PercentLthr = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum PwrZoneCalc {
    Custom = 0,
    PercentFtp = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum WktStepDuration {
    Time = 0,
    Distance = 1,
    HrLessThan = 2,
    HrGreaterThan = 3,
    Calories = 4,
    Open = 5,
    RepeatUntilStepsCmplt = 6,
    RepeatUntilTime = 7,
    RepeatUntilDistance = 8,
    RepeatUntilCalories = 9,
    RepeatUntilHrLessThan = 10,
    RepeatUntilHrGreaterThan = 11,
    RepeatUntilPowerLessThan = 12,
    RepeatUntilPowerGreaterThan = 13,
    PowerLessThan = 14,
    PowerGreaterThan = 15,
    TrainingPeaksTss = 16,
    RepeatUntilPowerLastLapLessThan = 17,
    RepeatUntilMaxPowerLastLapLessThan = 18,
    Power3sLessThan = 19,
    Power10sLessThan = 20,
    Power30sLessThan = 21,
    Power3sGreaterThan = 22,
    Power10sGreaterThan = 23,
    Power30sGreaterThan = 24,
    PowerLapLessThan = 25,
    PowerLapGreaterThan = 26,
    RepeatUntilTrainingPeaksTss = 27,
    RepetitionTime = 28,
    Reps = 29,
    TimeOnly = 31,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum WktStepTarget {
    Speed = 0,
    HeartRate = 1,
    Open = 2,
    Cadence = 3,
    Power = 4,
    Grade = 5,
    Resistance = 6,
    Power3s = 7,
    Power10s = 8,
    Power30s = 9,
    PowerLap = 10,
    SwimStroke = 11,
    SpeedLap = 12,
    HeartRateLap = 13,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Goal {
    Time = 0,
    Distance = 1,
    Calories = 2,
    Frequency = 3,
    Steps = 4,
    Ascent = 5,
    ActiveMinutes = 6,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum GoalRecurrence {
    Off = 0,
    Daily = 1,
    Weekly = 2,
    Monthly = 3,
    Yearly = 4,
    Custom = 5,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum GoalSource {
    Auto = 0,
    Community = 1,
    User = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Schedule {
    Workout = 0,
    Course = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum CoursePoint {
    Generic = 0,
    Summit = 1,
    Valley = 2,
    Water = 3,
    Food = 4,
    Danger = 5,
    Left = 6,
    Right = 7,
    Straight = 8,
    FirstAid = 9,
    FourthCategory = 10,
    ThirdCategory = 11,
    SecondCategory = 12,
    FirstCategory = 13,
    HorsCategory = 14,
    Sprint = 15,
    LeftFork = 16,
    RightFork = 17,
    MiddleFork = 18,
    SlightLeft = 19,
    SharpLeft = 20,
    SlightRight = 21,
    SharpRight = 22,
    UTurn = 23,
    SegmentStart = 24,
    SegmentEnd = 25,
    Campsite = 27,
    AidStation = 28,
    RestArea = 29,
    GeneralDistance = 30,
    Service = 31,
    EnergyGel = 32,
    SportsDrink = 33,
    MileMarker = 34,
    Checkpoint = 35,
    Shelter = 36,
    MeetingSpot = 37,
    Overlook = 38,
    Toilet = 39,
    Shower = 40,
    Gear = 41,
    SharpCurve = 42,
    SteepIncline = 43,
    Tunnel = 44,
    Bridge = 45,
    Obstacle = 46,
    Crossing = 47,
    Store = 48,
    Transition = 49,
    Navaid = 50,
    Transport = 51,
    Alert = 52,
    Info = 53,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type Manufacturer = u16;
pub type GarminProduct = u16;
pub type AntplusDeviceType = u8;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum AntNetwork {
    Public = 0,
    Antplus = 1,
    Antfs = 2,
    Private = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type WorkoutCapabilities = u32;
pub type BatteryStatus = u8;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum HrType {
    Normal = 0,
    Irregular = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type CourseCapabilities = u32;
pub type Weight = u16;
pub type WorkoutHr = u32;
pub type WorkoutPower = u32;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum BpStatus {
    NoError = 0,
    ErrorIncompleteData = 1,
    ErrorNoMeasurement = 2,
    ErrorDataOutOfRange = 3,
    ErrorIrregularHeartRate = 4,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type UserLocalId = u16;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum SwimStroke {
    Freestyle = 0,
    Backstroke = 1,
    Breaststroke = 2,
    Butterfly = 3,
    Drill = 4,
    Mixed = 5,
    Im = 6,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum ActivityType {
    Generic = 0,
    Running = 1,
    Cycling = 2,
    Transition = 3,
    FitnessEquipment = 4,
    Swimming = 5,
    Walking = 6,
    Sedentary = 8,
    All = 254,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum ActivitySubtype {
    Generic = 0,
    Treadmill = 1,
    Street = 2,
    Trail = 3,
    Track = 4,
    Spin = 5,
    IndoorCycling = 6,
    Road = 7,
    Mountain = 8,
    Downhill = 9,
    Recumbent = 10,
    Cyclocross = 11,
    HandCycling = 12,
    TrackCycling = 13,
    IndoorRowing = 14,
    Elliptical = 15,
    StairClimbing = 16,
    LapSwimming = 17,
    OpenWater = 18,
    All = 254,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum ActivityLevel {
    Low = 0,
    Medium = 1,
    High = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Side {
    Right = 0,
    Left = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type LeftRightBalance = u8;
pub type LeftRightBalance100 = u16;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum LengthType {
    Idle = 0,
    Active = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum DayOfWeek {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type ConnectivityCapabilities = u32;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum WeatherReport {
    Current = 0,
    Forecast = 1,
    HourlyForecast = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum WeatherStatus {
    Clear = 0,
    PartlyCloudy = 1,
    MostlyCloudy = 2,
    Rain = 3,
    Snow = 4,
    Windy = 5,
    Thunderstorms = 6,
    WintryMix = 7,
    Fog = 8,
    Hazy = 11,
    Hail = 12,
    ScatteredShowers = 13,
    ScatteredThunderstorms = 14,
    UnknownPrecipitation = 15,
    LightRain = 16,
    HeavyRain = 17,
    LightSnow = 18,
    HeavySnow = 19,
    LightRainSnow = 20,
    HeavyRainSnow = 21,
    Cloudy = 22,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum WeatherSeverity {
    Unknown = 0,
    Warning = 1,
    Watch = 2,
    Advisory = 3,
    Statement = 4,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum WeatherSevereType {
    Unspecified = 0,
    Tornado = 1,
    Tsunami = 2,
    Hurricane = 3,
    ExtremeWind = 4,
    Typhoon = 5,
    InlandHurricane = 6,
    HurricaneForceWind = 7,
    Waterspout = 8,
    SevereThunderstorm = 9,
    WreckhouseWinds = 10,
    LesSuetesWind = 11,
    Avalanche = 12,
    FlashFlood = 13,
    TropicalStorm = 14,
    InlandTropicalStorm = 15,
    Blizzard = 16,
    IceStorm = 17,
    FreezingRain = 18,
    DebrisFlow = 19,
    FlashFreeze = 20,
    DustStorm = 21,
    HighWind = 22,
    WinterStorm = 23,
    HeavyFreezingSpray = 24,
    ExtremeCold = 25,
    WindChill = 26,
    ColdWave = 27,
    HeavySnowAlert = 28,
    LakeEffectBlowingSnow = 29,
    SnowSquall = 30,
    LakeEffectSnow = 31,
    WinterWeather = 32,
    Sleet = 33,
    Snowfall = 34,
    SnowAndBlowingSnow = 35,
    BlowingSnow = 36,
    SnowAlert = 37,
    ArcticOutflow = 38,
    FreezingDrizzle = 39,
    Storm = 40,
    StormSurge = 41,
    Rainfall = 42,
    ArealFlood = 43,
    CoastalFlood = 44,
    LakeshoreFlood = 45,
    ExcessiveHeat = 46,
    Heat = 47,
    Weather = 48,
    HighHeatAndHumidity = 49,
    HumidexAndHealth = 50,
    Humidex = 51,
    Gale = 52,
    FreezingSpray = 53,
    SpecialMarine = 54,
    Squall = 55,
    StrongWind = 56,
    LakeWind = 57,
    MarineWeather = 58,
    Wind = 59,
    SmallCraftHazardousSeas = 60,
    HazardousSeas = 61,
    SmallCraft = 62,
    SmallCraftWinds = 63,
    SmallCraftRoughBar = 64,
    HighWaterLevel = 65,
    Ashfall = 66,
    FreezingFog = 67,
    DenseFog = 68,
    DenseSmoke = 69,
    BlowingDust = 70,
    HardFreeze = 71,
    Freeze = 72,
    Frost = 73,
    FireWeather = 74,
    Flood = 75,
    RipTide = 76,
    HighSurf = 77,
    Smog = 78,
    AirQuality = 79,
    BriskWind = 80,
    AirStagnation = 81,
    LowWater = 82,
    Hydrological = 83,
    SpecialWeather = 84,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type TimeIntoDay = u32;
pub type LocaltimeIntoDay = u32;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum StrokeType {
    NoEvent = 0,
    Other = 1,
    Serve = 2,
    Forehand = 3,
    Backhand = 4,
    Smash = 5,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum BodyLocation {
    LeftLeg = 0,
    LeftCalf = 1,
    LeftShin = 2,
    LeftHamstring = 3,
    LeftQuad = 4,
    LeftGlute = 5,
    RightLeg = 6,
    RightCalf = 7,
    RightShin = 8,
    RightHamstring = 9,
    RightQuad = 10,
    RightGlute = 11,
    TorsoBack = 12,
    LeftLowerBack = 13,
    LeftUpperBack = 14,
    RightLowerBack = 15,
    RightUpperBack = 16,
    TorsoFront = 17,
    LeftAbdomen = 18,
    LeftChest = 19,
    RightAbdomen = 20,
    RightChest = 21,
    LeftArm = 22,
    LeftShoulder = 23,
    LeftBicep = 24,
    LeftTricep = 25,
    LeftBrachioradialis = 26,
    LeftForearmExtensors = 27,
    RightArm = 28,
    RightShoulder = 29,
    RightBicep = 30,
    RightTricep = 31,
    RightBrachioradialis = 32,
    RightForearmExtensors = 33,
    Neck = 34,
    Throat = 35,
    WaistMidBack = 36,
    WaistFront = 37,
    WaistLeft = 38,
    WaistRight = 39,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum SegmentLapStatus {
    End = 0,
    Fail = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum SegmentLeaderboardType {
    Overall = 0,
    PersonalBest = 1,
    Connections = 2,
    Group = 3,
    Challenger = 4,
    Kom = 5,
    Qom = 6,
    Pr = 7,
    Goal = 8,
    Rival = 9,
    ClubLeader = 10,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum SegmentDeleteStatus {
    DoNotDelete = 0,
    DeleteOne = 1,
    DeleteAll = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum SegmentSelectionType {
    Starred = 0,
    Suggested = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum SourceType {
    Ant = 0,
    Antplus = 1,
    Bluetooth = 2,
    BluetoothLowEnergy = 3,
    Wifi = 4,
    Local = 5,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type LocalDeviceType = u8;
pub type BleDeviceType = u8;
pub type AntChannelId = u32;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum DisplayOrientation {
    Auto = 0,
    Portrait = 1,
    Landscape = 2,
    PortraitFlipped = 3,
    LandscapeFlipped = 4,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum WorkoutEquipment {
    None = 0,
    SwimFins = 1,
    SwimKickboard = 2,
    SwimPaddles = 3,
    SwimPullBuoy = 4,
    SwimSnorkel = 5,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum WatchfaceMode {
    Digital = 0,
    Analog = 1,
    ConnectIq = 2,
    Disabled = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum DigitalWatchfaceLayout {
    Traditional = 0,
    Modern = 1,
    Bold = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum AnalogWatchfaceLayout {
    Minimal = 0,
    Traditional = 1,
    Modern = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum RiderPositionType {
    Seated = 0,
    Standing = 1,
    TransitionToSeated = 2,
    TransitionToStanding = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum PowerPhaseType {
    PowerPhaseStartAngle = 0,
    PowerPhaseEndAngle = 1,
    PowerPhaseArcLength = 2,
    PowerPhaseCenter = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum CameraEventType {
    VideoStart = 0,
    VideoSplit = 1,
    VideoEnd = 2,
    PhotoTaken = 3,
    VideoSecondStreamStart = 4,
    VideoSecondStreamSplit = 5,
    VideoSecondStreamEnd = 6,
    VideoSplitStart = 7,
    VideoSecondStreamSplitStart = 8,
    VideoPause = 11,
    VideoSecondStreamPause = 12,
    VideoResume = 13,
    VideoSecondStreamResume = 14,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum SensorType {
    Accelerometer = 0,
    Gyroscope = 1,
    Compass = 2,
    Barometer = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum BikeLightNetworkConfigType {
    Auto = 0,
    Individual = 4,
    HighVisibility = 5,
    Trail = 6,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type CommTimeoutType = u16;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum CameraOrientationType {
    CameraOrientation0 = 0,
    CameraOrientation90 = 1,
    CameraOrientation180 = 2,
    CameraOrientation270 = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum AttitudeStage {
    Failed = 0,
    Aligning = 1,
    Degraded = 2,
    Valid = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type AttitudeValidity = u16;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum AutoSyncFrequency {
    Never = 0,
    Occasionally = 1,
    Frequent = 2,
    OnceADay = 3,
    Remote = 4,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum ExdLayout {
    FullScreen = 0,
    HalfVertical = 1,
    HalfHorizontal = 2,
    HalfVerticalRightSplit = 3,
    HalfHorizontalBottomSplit = 4,
    FullQuarterSplit = 5,
    HalfVerticalLeftSplit = 6,
    HalfHorizontalTopSplit = 7,
    Dynamic = 8,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum ExdDisplayType {
    Numerical = 0,
    Simple = 1,
    Graph = 2,
    Bar = 3,
    CircleGraph = 4,
    VirtualPartner = 5,
    Balance = 6,
    StringList = 7,
    String = 8,
    SimpleDynamicIcon = 9,
    Gauge = 10,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum ExdDataUnits {
    NoUnits = 0,
    Laps = 1,
    MilesPerHour = 2,
    KilometersPerHour = 3,
    FeetPerHour = 4,
    MetersPerHour = 5,
    DegreesCelsius = 6,
    DegreesFarenheit = 7,
    Zone = 8,
    Gear = 9,
    Rpm = 10,
    Bpm = 11,
    Degrees = 12,
    Millimeters = 13,
    Meters = 14,
    Kilometers = 15,
    Feet = 16,
    Yards = 17,
    Kilofeet = 18,
    Miles = 19,
    Time = 20,
    EnumTurnType = 21,
    Percent = 22,
    Watts = 23,
    WattsPerKilogram = 24,
    EnumBatteryStatus = 25,
    EnumBikeLightBeamAngleMode = 26,
    EnumBikeLightBatteryStatus = 27,
    EnumBikeLightNetworkConfigType = 28,
    Lights = 29,
    Seconds = 30,
    Minutes = 31,
    Hours = 32,
    Calories = 33,
    Kilojoules = 34,
    Milliseconds = 35,
    SecondPerMile = 36,
    SecondPerKilometer = 37,
    Centimeter = 38,
    EnumCoursePoint = 39,
    Bradians = 40,
    EnumSport = 41,
    InchesHg = 42,
    MmHg = 43,
    Mbars = 44,
    HectoPascals = 45,
    FeetPerMin = 46,
    MetersPerMin = 47,
    MetersPerSec = 48,
    EightCardinal = 49,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum ExdQualifiers {
    NoQualifier = 0,
    Instantaneous = 1,
    Average = 2,
    Lap = 3,
    Maximum = 4,
    MaximumAverage = 5,
    MaximumLap = 6,
    LastLap = 7,
    AverageLap = 8,
    ToDestination = 9,
    ToGo = 10,
    ToNext = 11,
    NextCoursePoint = 12,
    Total = 13,
    ThreeSecondAverage = 14,
    TenSecondAverage = 15,
    ThirtySecondAverage = 16,
    PercentMaximum = 17,
    PercentMaximumAverage = 18,
    LapPercentMaximum = 19,
    Elapsed = 20,
    Sunrise = 21,
    Sunset = 22,
    ComparedToVirtualPartner = 23,
    Maximum24h = 24,
    Minimum24h = 25,
    Minimum = 26,
    First = 27,
    Second = 28,
    Third = 29,
    Shifter = 30,
    LastSport = 31,
    Moving = 32,
    Stopped = 33,
    EstimatedTotal = 34,
    Zone9 = 242,
    Zone8 = 243,
    Zone7 = 244,
    Zone6 = 245,
    Zone5 = 246,
    Zone4 = 247,
    Zone3 = 248,
    Zone2 = 249,
    Zone1 = 250,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum ExdDescriptors {
    BikeLightBatteryStatus = 0,
    BeamAngleStatus = 1,
    BateryLevel = 2,
    LightNetworkMode = 3,
    NumberLightsConnected = 4,
    Cadence = 5,
    Distance = 6,
    EstimatedTimeOfArrival = 7,
    Heading = 8,
    Time = 9,
    BatteryLevel = 10,
    TrainerResistance = 11,
    TrainerTargetPower = 12,
    TimeSeated = 13,
    TimeStanding = 14,
    Elevation = 15,
    Grade = 16,
    Ascent = 17,
    Descent = 18,
    VerticalSpeed = 19,
    Di2BatteryLevel = 20,
    FrontGear = 21,
    RearGear = 22,
    GearRatio = 23,
    HeartRate = 24,
    HeartRateZone = 25,
    TimeInHeartRateZone = 26,
    HeartRateReserve = 27,
    Calories = 28,
    GpsAccuracy = 29,
    GpsSignalStrength = 30,
    Temperature = 31,
    TimeOfDay = 32,
    Balance = 33,
    PedalSmoothness = 34,
    Power = 35,
    FunctionalThresholdPower = 36,
    IntensityFactor = 37,
    Work = 38,
    PowerRatio = 39,
    NormalizedPower = 40,
    TrainingStressScore = 41,
    TimeOnZone = 42,
    Speed = 43,
    Laps = 44,
    Reps = 45,
    WorkoutStep = 46,
    CourseDistance = 47,
    NavigationDistance = 48,
    CourseEstimatedTimeOfArrival = 49,
    NavigationEstimatedTimeOfArrival = 50,
    CourseTime = 51,
    NavigationTime = 52,
    CourseHeading = 53,
    NavigationHeading = 54,
    PowerZone = 55,
    TorqueEffectiveness = 56,
    TimerTime = 57,
    PowerWeightRatio = 58,
    LeftPlatformCenterOffset = 59,
    RightPlatformCenterOffset = 60,
    LeftPowerPhaseStartAngle = 61,
    RightPowerPhaseStartAngle = 62,
    LeftPowerPhaseFinishAngle = 63,
    RightPowerPhaseFinishAngle = 64,
    Gears = 65,
    Pace = 66,
    TrainingEffect = 67,
    VerticalOscillation = 68,
    VerticalRatio = 69,
    GroundContactTime = 70,
    LeftGroundContactTimeBalance = 71,
    RightGroundContactTimeBalance = 72,
    StrideLength = 73,
    RunningCadence = 74,
    PerformanceCondition = 75,
    CourseType = 76,
    TimeInPowerZone = 77,
    NavigationTurn = 78,
    CourseLocation = 79,
    NavigationLocation = 80,
    Compass = 81,
    GearCombo = 82,
    MuscleOxygen = 83,
    Icon = 84,
    CompassHeading = 85,
    GpsHeading = 86,
    GpsElevation = 87,
    AnaerobicTrainingEffect = 88,
    Course = 89,
    OffCourse = 90,
    GlideRatio = 91,
    VerticalDistance = 92,
    Vmg = 93,
    AmbientPressure = 94,
    Pressure = 95,
    Vam = 96,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type AutoActivityDetect = u32;
pub type SupportedExdScreenLayouts = u32;
pub type FitBaseType = u8;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum TurnType {
    ArrivingIdx = 0,
    ArrivingLeftIdx = 1,
    ArrivingRightIdx = 2,
    ArrivingViaIdx = 3,
    ArrivingViaLeftIdx = 4,
    ArrivingViaRightIdx = 5,
    BearKeepLeftIdx = 6,
    BearKeepRightIdx = 7,
    ContinueIdx = 8,
    ExitLeftIdx = 9,
    ExitRightIdx = 10,
    FerryIdx = 11,
    Roundabout45Idx = 12,
    Roundabout90Idx = 13,
    Roundabout135Idx = 14,
    Roundabout180Idx = 15,
    Roundabout225Idx = 16,
    Roundabout270Idx = 17,
    Roundabout315Idx = 18,
    Roundabout360Idx = 19,
    RoundaboutNeg45Idx = 20,
    RoundaboutNeg90Idx = 21,
    RoundaboutNeg135Idx = 22,
    RoundaboutNeg180Idx = 23,
    RoundaboutNeg225Idx = 24,
    RoundaboutNeg270Idx = 25,
    RoundaboutNeg315Idx = 26,
    RoundaboutNeg360Idx = 27,
    RoundaboutGenericIdx = 28,
    RoundaboutNegGenericIdx = 29,
    SharpTurnLeftIdx = 30,
    SharpTurnRightIdx = 31,
    TurnLeftIdx = 32,
    TurnRightIdx = 33,
    UturnLeftIdx = 34,
    UturnRightIdx = 35,
    IconInvIdx = 36,
    IconIdxCnt = 37,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type BikeLightBeamAngleMode = u8;
pub type FitBaseUnit = u16;
pub type SetType = u8;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum MaxMetCategory {
    Generic = 0,
    Cycling = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type ExerciseCategory = u16;
pub type BenchPressExerciseName = u16;
pub type CalfRaiseExerciseName = u16;
pub type CardioExerciseName = u16;
pub type CarryExerciseName = u16;
pub type ChopExerciseName = u16;
pub type CoreExerciseName = u16;
pub type CrunchExerciseName = u16;
pub type CurlExerciseName = u16;
pub type DeadliftExerciseName = u16;
pub type FlyeExerciseName = u16;
pub type HipRaiseExerciseName = u16;
pub type HipStabilityExerciseName = u16;
pub type HipSwingExerciseName = u16;
pub type HyperextensionExerciseName = u16;
pub type LateralRaiseExerciseName = u16;
pub type LegCurlExerciseName = u16;
pub type LegRaiseExerciseName = u16;
pub type LungeExerciseName = u16;
pub type OlympicLiftExerciseName = u16;
pub type PlankExerciseName = u16;
pub type PlyoExerciseName = u16;
pub type PullUpExerciseName = u16;
pub type PushUpExerciseName = u16;
pub type RowExerciseName = u16;
pub type ShoulderPressExerciseName = u16;
pub type ShoulderStabilityExerciseName = u16;
pub type ShrugExerciseName = u16;
pub type SitUpExerciseName = u16;
pub type SquatExerciseName = u16;
pub type TotalBodyExerciseName = u16;
pub type TricepsExtensionExerciseName = u16;
pub type WarmUpExerciseName = u16;
pub type RunExerciseName = u16;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum WaterType {
    Fresh = 0,
    Salt = 1,
    En13319 = 2,
    Custom = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum TissueModelType {
    Zhl16c = 0,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum DiveGasStatus {
    Disabled = 0,
    Enabled = 1,
    BackupOnly = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum DiveAlert {
    NdlReached = 0,
    GasSwitchPrompted = 1,
    NearSurface = 2,
    ApproachingNdl = 3,
    Po2Warn = 4,
    Po2CritHigh = 5,
    Po2CritLow = 6,
    TimeAlert = 7,
    DepthAlert = 8,
    DecoCeilingBroken = 9,
    DecoComplete = 10,
    SafetyStopBroken = 11,
    SafetyStopComplete = 12,
    CnsWarning = 13,
    CnsCritical = 14,
    OtuWarning = 15,
    OtuCritical = 16,
    AscentCritical = 17,
    AlertDismissedByKey = 18,
    AlertDismissedByTimeout = 19,
    BatteryLow = 20,
    BatteryCritical = 21,
    SafetyStopStarted = 22,
    ApproachingFirstDecoStop = 23,
    SetpointSwitchAutoLow = 24,
    SetpointSwitchAutoHigh = 25,
    SetpointSwitchManualLow = 26,
    SetpointSwitchManualHigh = 27,
    AutoSetpointSwitchIgnored = 28,
    SwitchedToOpenCircuit = 29,
    SwitchedToClosedCircuit = 30,
    TankBatteryLow = 32,
    Po2CcrDilLow = 33,
    DecoStopCleared = 34,
    ApneaNeutralBuoyancy = 35,
    ApneaTargetDepth = 36,
    ApneaSurface = 37,
    ApneaHighSpeed = 38,
    ApneaLowSpeed = 39,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum DiveAlarmType {
    Depth = 0,
    Time = 1,
    Speed = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum DiveBacklightMode {
    AtDepth = 0,
    AlwaysOn = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum SleepLevel {
    Unmeasurable = 0,
    Awake = 1,
    Light = 2,
    Deep = 3,
    Rem = 4,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Spo2MeasurementType {
    OffWrist = 0,
    SpotCheck = 1,
    ContinuousCheck = 2,
    Periodic = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum CcrSetpointSwitchMode {
    Manual = 0,
    Automatic = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum DiveGasMode {
    OpenCircuit = 0,
    ClosedCircuitDiluent = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
pub type FaveroProduct = u16;
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum SplitType {
    AscentSplit = 1,
    DescentSplit = 2,
    IntervalActive = 3,
    IntervalRest = 4,
    IntervalWarmup = 5,
    IntervalCooldown = 6,
    IntervalRecovery = 7,
    IntervalOther = 8,
    ClimbActive = 9,
    ClimbRest = 10,
    SurfActive = 11,
    RunActive = 12,
    RunRest = 13,
    WorkoutRound = 14,
    RwdRun = 17,
    RwdWalk = 18,
    WindsurfActive = 21,
    RwdStand = 22,
    Transition = 23,
    SkiLiftSplit = 28,
    SkiRunSplit = 29,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum ClimbProEvent {
    Approach = 0,
    Start = 1,
    Complete = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum GasConsumptionRateType {
    PressureSac = 0,
    VolumeSac = 1,
    Rmv = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum TapSensitivity {
    High = 0,
    Medium = 1,
    Low = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum RadarThreatLevelType {
    ThreatUnknown = 0,
    ThreatNone = 1,
    ThreatApproaching = 2,
    ThreatApproachingFast = 3,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum MaxMetSpeedSource {
    OnboardGps = 0,
    ConnectedGps = 1,
    Cadence = 2,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum MaxMetHeartRateSource {
    Whr = 0,
    Hrm = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum HrvStatus {
    None = 0,
    Poor = 1,
    Low = 2,
    Unbalanced = 3,
    Balanced = 4,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum NoFlyTimeMode {
    Standard = 0,
    Flat24Hours = 1,
    #[num_enum(catch_all)]
    UnknownValue(u8),
}
