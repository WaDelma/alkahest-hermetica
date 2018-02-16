extern crate nom;

use std::collections::HashMap;

enum Symbol {
    Quintessence,
    Air,
    Fire,
    Earth,
    Water,
    Aquafortis,
    AquaRegia,
    AquaRegia2, // Balneum arenae (sand bath), Lapis armenus (Armenian stone).
    AquaVitae, // Spiritus vini.
    AquaVitae2,
    Vinegar, //crucible; acid; distill; atrament; vitriol; red sulfur; borax; wine; alkali salt; mercurius vivus, quick silver
    Vinegar2, // Distilled vinegar,
    Vinegar3,
    Sulfur, // Brimstone
    PhilosophersSulfur,
    BlackSulfur, // Sulfur nigra, dye
    MercurySublimate,
    MercurySublimate2,
    MercurySublimate3,
    Cinnabar,
    Salt,
    Nitre,
    Vitrol,
    Vitrol2,
    RockSalt, // Sal gemmae
    RockSalt2,
    Gold,
    Silver,
    IronOre,
    IronOre2,
    CrocusOfIron, // Crocus martis, Red or yellow calcined powder of iron
    RegulusOfIron, // Regulus martis, Scoria from refining stibnite/antimony with iron
    CopperOre,
    IronCopperOre,
    SublimateOfCopper,
    CrocusOfCopper, // Crocus veneris, aes ustum
    CrocusOfCopper2,
    CopperAntimoniate, // Crocus of copper, Crocus veneris, lapis haematites
    SublimateOfSaltOfCopper,
    Verdigris, // Aes viride, copper subacetate, early astronomical symbol of earth
    TinOre, 
    LeadOre,
    AntimonyOre, // Stibnite
    SublimateOfAntimony,
    SaltOfAntimony, // Cinnabar
    SublimateOfSaltOfAntimony,
    VinegarOfAntimony,
    RegulusOfAntimony, // Antimony metal
    RegulusOfAntimony2,
    Regulus,
    Regulus2,
    Regulus3,
    Regulus4,
    Alkali, // Sal alkali
    Alkali2,
    Marcasite, // Iron pyrite, iron sulfide
    SalAmmoniac, // Ammonium choride
    Arsenic,
    Realgar, // Arsenic sulfide
    Realgar2, // Arsenic sulfide
    Auripigment, // Orpiment, Arsenic trisulfide
    BismuthOre, // Tinglass.
    Tartar, // Impure potassium tartrate.
    Tartar2, // Impure potassium tartrate.
    QuickLime, // Calx viva, Calcium oxide.
    Borax,
    Borax2,
    Borax3,
    Alum,
    Oil,
    Spirit,
    Tincture,
    Gum,
    Wax, // Cera
    Powder, // Pulvis
    Calx, // Calcinare, Oxide residue, Calcium oxide.
    Tutty, // Tutia, Crude zinc oxide sublimate, Aes viride.
    CaputMortuum, // Worthless residue of sublimation or distillation.
    ScepterOfJove,
    Caduceus,
    Trident,
    StarredTrident,
    Lodestone, // Magnes.
    Soap,
    Urine,
    HorseDung, // Fimus equinus.
    Ashes, // Cineres.
    PotAshes, // Cineres clavellati, Alumen
    Brick,
    PowderedBrick, // Later cibratus, Farina laterum.
    Amalgam,
    SuperStratum,
    SuperStratum2,
    Sublimation, // Action
    Precipitate, // Action
    Distill, // Sublimate. Action
    Dissolve, // Action
    Dissolve2, // Water, Aqua. Action
    Purify, // Action
    Putrefaction, // Action
    Crucible, // Tigellum. Variable
    Crucible2, // Variable
    Crucible3, // Variable
    Crucible4, // Variable
    Crucible5, // Variable
    Alembic, // Variable
    BathOfMary, // Balneum mariae.
    BathOfVapours, // Balneum vaporis.
    Retort, // Variable
    Hour, // Measure
    Night, // Measure
    DayNight, // Measure
    Month, // Mensis. Measure
    HalfDram // Drachma semis. Measure
}

enum Element {
    Air,
    Fire,
    Earth,
    Water,

    Aether,

    Sulphur,
    Mercury,
    Salt,
}

struct Cauldron {
    contents: HashMap<Element, f64>,
}

// 🝭

impl Symbol {
    fn as_char(self) -> char {
        [
            '🜀','🜁','🜂','🜃','🜄','🜅',
            '🜆','🜇','🜈','🜉','🜊','🜋','🜌',
            '🜍','🜎','🜏','🜐','🜑','🜒','🜓',
            '🜔','🜕','🜖','🜗','🜘','🜙','🜚',
            '🜛','🜜','🜝','🜞','🜟','🜠','🜡',
            '🜢','🜣','🜤','🜥','🜦','🜧','🜨','🜩',
            '🜪','🜫','🜬','🜭','🜮','🜯','🜰','🜱','🜲',
            '🜳','🜴','🜵','🜶','🜷','🜸','🜹','🜺',
            '🜻','🜼','🜽','🜾','🜿','🝀','🝁','🝂',
            '🝃','🝄','🝅','🝆','🝇','🝈','🝉','🝊',
            '🝋','🝌','🝍','🝎','🝏','🝐','🝑','🝒',
            '🝓','🝔','🝕','🝖','🝗','🝘','🝙','🝚',
            '🝛','🝜','🝝','🝞','🝟','🝠','🝡','🝢',
            '🝣','🝤','🝥','🝦','🝧','🝨','🝩','🝪',
            '🝫','🝬','🝭','🝮','🝯','🝰','🝱','🝲','🝳'
        ][self as usize]
    }

}
