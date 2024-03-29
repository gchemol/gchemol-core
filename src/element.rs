// [[file:../gchemol-core.note::*imports][imports:1]]
use serde::*;
// imports:1 ends here

// [[file:../gchemol-core.note::cab264f1][cab264f1]]
const ELEMENT_DATA: [(&str, &str); 118] = [
    ("H", "Hydrogen"),
    ("He", "Helium"),
    ("Li", "Lithium"),
    ("Be", "Beryllium"),
    ("B", "Boron"),
    ("C", "Carbon"),
    ("N", "Nitrogen"),
    ("O", "Oxygen"),
    ("F", "Fluorine"),
    ("Ne", "Neon"),
    ("Na", "Sodium"),
    ("Mg", "Magnesium"),
    ("Al", "Aluminum"),
    ("Si", "Silicon"),
    ("P", "Phosphorus"),
    ("S", "Sulfur"),
    ("Cl", "Chlorine"),
    ("Ar", "Argon"),
    ("K", "Potassium"),
    ("Ca", "Calcium"),
    ("Sc", "Scandium"),
    ("Ti", "Titanium"),
    ("V", "Vanadium"),
    ("Cr", "Chromium"),
    ("Mn", "Manganese"),
    ("Fe", "Iron"),
    ("Co", "Cobalt"),
    ("Ni", "Nickel"),
    ("Cu", "Copper"),
    ("Zn", "Zinc"),
    ("Ga", "Gallium"),
    ("Ge", "Germanium"),
    ("As", "Arsenic"),
    ("Se", "Selenium"),
    ("Br", "Bromine"),
    ("Kr", "Krypton"),
    ("Rb", "Rubidium"),
    ("Sr", "Strontium"),
    ("Y", "Yttrium"),
    ("Zr", "Zirconium"),
    ("Nb", "Niobium"),
    ("Mo", "Molybdenum"),
    ("Tc", "Technetium"),
    ("Ru", "Ruthenium"),
    ("Rh", "Rhodium"),
    ("Pd", "Palladium"),
    ("Ag", "Silver"),
    ("Cd", "Cadmium"),
    ("In", "Indium"),
    ("Sn", "Tin"),
    ("Sb", "Antimony"),
    ("Te", "Tellurium"),
    ("I", "Iodine"),
    ("Xe", "Xenon"),
    ("Cs", "Cesium"),
    ("Ba", "Barium"),
    ("La", "Lanthanum"),
    ("Ce", "Cerium"),
    ("Pr", "Praesodymium"),
    ("Nd", "Neodymium"),
    ("Pm", "Promethium"),
    ("Sm", "Samarium"),
    ("Eu", "Europium"),
    ("Gd", "Gadolinium"),
    ("Tb", "Terbium"),
    ("Dy", "Dyprosium"),
    ("Ho", "Holmium"),
    ("Er", "Erbium"),
    ("Tm", "Thulium"),
    ("Yb", "Ytterbium"),
    ("Lu", "Lutetium"),
    ("Hf", "Hafnium"),
    ("Ta", "Tantalium"),
    ("W", "Wolfram"),
    ("Re", "Rhenium"),
    ("Os", "Osmium"),
    ("Ir", "Iridium"),
    ("Pt", "Platinum"),
    ("Au", "Gold"),
    ("Hg", "Mercury"),
    ("Tl", "Thallium"),
    ("Pb", "Lead"),
    ("Bi", "Bismuth"),
    ("Po", "Polonium"),
    ("At", "Astatine"),
    ("Rn", "Radon"),
    ("Fr", "Francium"),
    ("Ra", "Radium"),
    ("Ac", "Actinium"),
    ("Th", "Thorium"),
    ("Pa", "Protactinium"),
    ("U", "Uranium"),
    ("Np", "Neptunium"),
    ("Pu", "Plutonium"),
    ("Am", "Americium"),
    ("Cm", "Curium"),
    ("Bk", "Berkelium"),
    ("Cf", "Californium"),
    ("Es", "Einsteinium"),
    ("Fm", "Fermium"),
    ("Mv", "Mendelevium"),
    ("No", "Nobelium"),
    ("Lr", "Lawrencium"),
    ("Rf", "Rutherfordium"),
    ("Db", "Dubnium"),
    ("Sg", "Seaborgium"),
    ("Bh", "Bohrium"),
    ("Hs", "Hassium"),
    ("Mt", "Meitnerium"),
    ("Uun", "Ununnilium"),
    ("Uuu", "Unununium"),
    ("Uub", "Ununbium"),
    ("Uut", "Ununtrium"),
    ("Uuq", "Ununquadium"),
    ("Uup", "Ununpentium"),
    ("Uuh", "Ununhexium"),
    ("Uus", "Ununseptium"),
    ("Uuo", "Ununoctium"),
];
// cab264f1 ends here

// [[file:../gchemol-core.note::*base][base:1]]
/// Represents different kind of atom, such as cheimcial element, dummy atom,
/// etc.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum AtomKind {
    /// Chemical element.
    Element(usize),

    /// Dummy atom for special purpose.
    Dummy(String),
}

use self::AtomKind::{Dummy, Element};

impl AtomKind {
    /// Element symbol.
    pub fn symbol(&self) -> &str {
        match &self {
            Element(num) => ELEMENT_DATA[num - 1].0,
            Dummy(sym) => sym,
        }
    }

    /// Atomic number.
    pub fn number(&self) -> usize {
        match &self {
            Element(num) => *num,
            Dummy(_) => 0,
        }
    }

    /// Element name.
    pub fn name(&self) -> &str {
        match &self {
            Element(num) => ELEMENT_DATA[num - 1].1,
            Dummy(sym) => sym,
        }
    }
}
// base:1 ends here

// [[file:../gchemol-core.note::3ffd6c5d][3ffd6c5d]]
gut::config::lazy_static! {
    /// Global cache for quick get element number from symbol
    pub static ref ELEMENTS: std::collections::HashMap<&'static str, usize> = {
        ELEMENT_DATA.iter().zip(1..).map(|((s, _), i)| (*s, i)).collect()
    };
}

impl std::fmt::Display for AtomKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:}", self.symbol())
    }
}

impl std::convert::From<usize> for AtomKind {
    fn from(value: usize) -> Self {
        match value {
            0 => Dummy("dummy".into()),
            _ => Element(value),
        }
    }
}

impl std::convert::From<&str> for AtomKind {
    fn from(label: &str) -> Self {
        // from element symbol
        if let Some(&n) = ELEMENTS.get(label) {
            return Element(n);
        }

        // element specified in number
        if let Ok(x) = label.parse::<usize>() {
            return Element(x);
        }

        // element specified in symbol or long name
        let sym = label.to_uppercase();
        for (i, &(s, n)) in ELEMENT_DATA.iter().enumerate() {
            if s.to_uppercase() == sym || n.to_uppercase() == sym {
                return Element(i + 1);
            }
        }

        // set as dummy atom as the last resort
        Dummy(label.into())
    }
}

impl std::convert::From<String> for AtomKind {
    fn from(label: String) -> Self {
        Self::from(label.as_str())
    }
}

impl std::convert::From<&String> for AtomKind {
    fn from(label: &String) -> Self {
        Self::from(label.as_str())
    }
}
// 3ffd6c5d ends here

// [[file:../gchemol-core.note::*test][test:1]]
#[test]
fn test_element() {
    let h1: AtomKind = 1.into();
    let h2: AtomKind = "H".into();
    let h3: AtomKind = "h".into();
    let h4: AtomKind = "1".into();

    assert_eq!(h1, h2);
    assert_eq!(h1, h3);
    assert_eq!(h1, h4);

    let si: AtomKind = "SI".into();
    assert_eq!(si.number(), 14);
    assert_eq!(si.to_string(), "Si");
    assert_eq!(si.name(), "Silicon");

    // dummy atom
    let x: AtomKind = "X".into();
    assert_eq!(x.symbol(), "X");
    assert_eq!(x.number(), 0);

    let s = String::from("Fe");
    let fe: AtomKind = s.into();
    assert_eq!(fe.symbol(), "Fe");
}
// test:1 ends here
