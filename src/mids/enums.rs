use std::{fmt::Debug, marker::PhantomData};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct BitVec<T> {
    value: u32,
    _marker: PhantomData<T>,
}

impl<T: Debug + TryFrom<u32>> Debug for BitVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut l = f.debug_list();
        let mut bit = self.value.trailing_zeros();
        while bit < u32::BITS {
            if let Ok(v) = T::try_from(bit) {
                l.entry(&v);
            }
            bit += 1;
            bit += (self.value >> bit).trailing_zeros();
        }
        l.finish()
    }
}

macro_rules! cs_enum {
        (Ord; $name:ident $tt:tt) => {
            #[allow(non_camel_case_types)]
            #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize_repr, Serialize_repr)]
            #[repr(u32)]
            pub enum $name $tt
        };
        ($name:ident $tt:tt) => {
            #[allow(non_camel_case_types)]
            #[derive(Clone, Copy, Debug, PartialEq, Deserialize_repr, Serialize_repr)]
            #[repr(u32)]
            pub enum $name $tt
        }
    }

macro_rules! bit_enum {
        ($name:ident, $bits:ident $tt:tt) => {
            #[allow(non_camel_case_types)]
            #[derive(Clone, Copy, Debug, TryFromPrimitive, IntoPrimitive)]
            #[repr(u32)]
            pub enum $bits $tt
            pub type $name = BitVec<$bits>;
        };
    }

cs_enum!(Aspect { Res, Max, Abs, Str, Cur });

cs_enum!(AttribType {
    Magnitude,
    Duration,
    Expression,
});

bit_enum!(
    CastFlags,
    CastKind {
        NearGround,
        TargetNearGround,
        CastableAfterDeath,
    }
);

cs_enum!(ClassType {
    None,
    Hero,
    HeroEpic,
    Villain,
    VillainEpic,
    Henchman,
    Pet,
});

cs_enum!(Damage {
    None,
    Smashing,
    Lethal,
    Fire,
    Cold,
    Energy,
    Negative,
    Toxic,
    Psionic,
    Special,
    Melee,
    Ranged,
    AoE,
    Unique1,
    Unique2,
    Unique3
});

cs_enum!(EffectArea {
    None,
    Character,
    Sphere,
    Cone,
    Location,
    Volume,
    Map,
    Room,
    Touch,
});

cs_enum!(EffectClass {
    Primary,
    Secondary,
    Tertiary,
    Special,
    Ignored,
    DisplayOnly,
});

cs_enum!(EffectType {
    None,
    Accuracy,
    ViewAttrib,
    Damage,
    DamageBuff,
    Defense,
    DropToggles,
    Endurance,
    EnduranceDiscount,
    Enhancement,
    Fly,
    SpeedFlying,
    GrantPower,
    Heal,
    HitPoints,
    InterruptTime,
    JumpHeight,
    SpeedJumping,
    Meter,
    Mez,
    MezResist,
    MovementControl,
    MovementFriction,
    PerceptionRadius,
    Range,
    RechargeTime,
    Recovery,
    Regeneration,
    ResEffect,
    Resistance,
    RevokePower,
    Reward,
    SpeedRunning,
    SetCostume,
    SetMode,
    Slow,
    StealthRadius,
    StealthRadiusPlayer,
    EntCreate,
    ThreatLevel,
    ToHit,
    Translucency,
    XPDebtProtection,
    SilentKill,
    Elusivity,
    GlobalChanceMod,
    CombatModShift,
    UnsetMode,
    Rage,
    MaxRunSpeed,
    MaxJumpSpeed,
    MaxFlySpeed,
    DesignerStatus,
    PowerRedirect,
    TokenAdd,
    ExperienceGain,
    InfluenceGain,
    PrestigeGain,
    AddBehavior,
    RechargePower,
    RewardSourceTeam,
    VisionPhase,
    CombatPhase,
    ClearFog,
    SetSZEValue,
    ExclusiveVisionPhase,
    Absorb,
    XAfraid,
    XAvoid,
    BeastRun,
    ClearDamagers,
    EntCreate_x,
    Glide,
    Hoverboard,
    Jumppack,
    MagicCarpet,
    NinjaRun,
    Null,
    NullBool,
    Stealth,
    SteamJump,
    Walk,
    XPDebt,
    ForceMove,
    ModifyAttrib,
    ExecutePower
});

cs_enum!(Enhance {
    None,
    Accuracy,
    Damage,
    Defense,
    EnduranceDiscount,
    Endurance,
    SpeedFlying,
    Heal,
    HitPoints,
    Interrupt,
    JumpHeight,
    SpeedJumping,
    Mez,
    Range,
    RechargeTime,
    X_RechargeTime,
    Recovery,
    Regeneration,
    Resistance,
    SpeedRunning,
    ToHit,
    Slow,
    Absorb
});

bit_enum!(
    Entity,
    EntityKind {
        Caster,
        Player,
        DeadPlayer,
        Teammate,
        DeadTeammate,
        DeadOrAliveTeammate,
        Villain,
        DeadVillain,
        NPC,
        Friend,
        DeadFriend,
        Foe,
        Location = 13,
        Teleport,
        Any,
        MyPet,
        DeadFoe,
        FoeRezzingFoe,
        Leaguemate,
        DeadLeaguemate,
        AnyLeaguemate,
        DeadMyCreation,
        DeadMyPet,
        DeadOrAliveFoe,
        DeadOrAliveLeaguemate,
        DeadPlayerFriend,
        MyOwner,
    }
);

cs_enum!(GridType {
    None,
    Accolade,
    Class,
    Incarnate,
    Inherent,
    Pet,
    Power,
    Powerset,
    Prestige,
    Temp,
});

cs_enum!(Mez {
    None,
    Confused,
    Held,
    Immobilized,
    Knockback,
    Knockup,
    OnlyAffectsSelf,
    Placate,
    Repel,
    Sleep,
    Stunned,
    Taunt,
    Terrorized,
    Untouchable,
    Teleport,
    ToggleDrop,
    Afraid,
    Avoid,
    CombatPhase,
    Intangible
});

bit_enum!(
    ModeFlags,
    ModeFlag {
        Arena,
        Disable_All,
        Disable_Enhancements,
        Disable_Epic,
        Disable_Inspirations,
        Disable_Market_TP,
        Disable_Pool,
        Disable_Rez_Insp,
        Disable_Teleport,
        Disable_Temp,
        Disable_Toggle,
        Disable_Travel,
        Domination,
        Peacebringer_Blaster_Mode,
        Peacebringer_Lightform_Mode,
        Peacebringer_Tanker_Mode,
        Raid_Attacker_Mode,
        Shivan_Mode,
        Warshade_Blaster_Mode = 19,
        Warshade_Tanker_Mode,
    }
);

cs_enum!(Notify {
    Always,
    Never,
    MissOnly,
    HitOnly,
});

cs_enum!(OverrideBoolean {
    NoOverride,
    TrueOverride,
    FalseOverride,
});

cs_enum!(PowerAttribs {
    None,
    Accuracy,
    ActivateInterval,
    Arc,
    CastTime,
    EffectArea,
    EnduranceCost,
    InterruptTime,
    MaxTargets,
    Radius,
    Range,
    RechargeTime,
    SecondaryRange,
});

cs_enum!(PowerSetType {
    None,
    Primary,
    Secondary,
    Ancillary,
    Inherent,
    Pool,
    Accolade,
    Temp,
    Pet,
    SetBonus,
    Boost,
    Incarnate,
    Redirect,
});

cs_enum!(PowerType {
    Click,
    Auto,
    Toggle,
    Boost,
    Inspiration,
    GlobalBoost,
});

cs_enum!(PvX { Any, Pve, Pvp });

cs_enum!(Ord; SetType {
    Untyped,
    MeleeST,
    RangedST,
    RangedAoE,
    MeleeAoE,
    Snipe,
    Pets,
    Defense,
    Resistance,
    Heal,
    Hold,
    Stun,
    Immob,
    Slow,
    Sleep,
    Fear,
    Confuse,
    Flight,
    Jump,
    Run,
    Teleport,
    DefDebuff,
    EndMod,
    Knockback,
    Threat,
    ToHit,
    ToHitDeb,
    PetRech,
    Travel,
    AccHeal,
    AccDefDeb,
    AccToHitDeb,
    Arachnos,
    Blaster,
    Brute,
    Controller,
    Corruptor,
    Defender,
    Dominator,
    Kheldian,
    Mastermind,
    Scrapper,
    Stalker,
    Tanker,
    UniversalDamage,
    Sentinel,
    RunNoSprint,
    JumpNoSprint,
    FlightNoSprint,
    TeleportNoSprint
});

cs_enum!(SpecialCase {
    None,
    Hidden,
    Domination,
    Scourge,
    Mezzed,
    CriticalHit,
    CriticalBoss,
    CriticalMinion,
    Robot,
    Assassination,
    Containment,
    Defiance,
    TargetDroneActive,
    Combo,
    VersusSpecial,
    NotDisintegrated,
    Disintegrated,
    NotAccelerated,
    Accelerated,
    NotDelayed,
    Delayed,
    ComboLevel0,
    ComboLevel1,
    ComboLevel2,
    ComboLevel3,
    FastMode,
    NotAssassination,
    PerfectionOfBody0,
    PerfectionOfBody1,
    PerfectionOfBody2,
    PerfectionOfBody3,
    PerfectionOfMind0,
    PerfectionOfMind1,
    PerfectionOfMind2,
    PerfectionOfMind3,
    PerfectionOfSoul0,
    PerfectionOfSoul1,
    PerfectionOfSoul2,
    PerfectionOfSoul3,
    TeamSize1,
    TeamSize2,
    TeamSize3,
    NotComboLevel3,
    ToHit97,
    DefensiveAdaptation,
    EfficientAdaptation,
    OffensiveAdaptation,
    NotDefensiveAdaptation,
    NotDefensiveNorOffensiveAdaptation,
    BoxingBuff,
    KickBuff,
    Supremacy,
    SupremacyAndBuffPwr,
    PetTier2,
    PetTier3,
    PackMentality,
    NotPackMentality,
    FastSnipe,
    NotFastSnipe,
    CrossPunchBuff,
    NotCrossPunchBuff,
    NotBoxingBuff,
    NotKickBuff
});

cs_enum!(Stacking { No, Yes });

cs_enum!(SummonEntity { Pet, Henchman });

bit_enum!(
    Suppress,
    SuppressType {
        Held,
        Sleep,
        Stunned,
        Immobilized,
        Terrorized,
        Knocked,
        Attacked,
        HitByFoe,
        MissionObjectClick,
        ActivateAttackClick,
        Damaged,
        Phased1,
        Confused,
    }
);

cs_enum!(ToWho {
    Unspecified,
    Target,
    Self_,
    All,
});

bit_enum!(
    Vector,
    VectorKind {
        Melee,
        Ranged,
        Aoe,
        Smashing,
        Lethal,
        Cold,
        Fire,
        Energy,
        NegativeEnergy,
        Psionic,
        Toxic,
    }
);
