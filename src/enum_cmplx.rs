
use super::enum_unit::*;

#[derive(Debug)]
pub struct Sensacion {
    pub tipo: TipoSensacion,
    pub intensidad: Intensidad,
}
#[derive(Debug)]
pub struct SensaciónCorporal {
    pub parte: ParteCuerpo,
    pub sensación: Sensacion,
}
pub enum Ambito {
  Tonal {
      época: String,
      lado_izquierdo: &'static str, // parte práctica
      lado_derecho: &'static str,   // parte mental
  },
  Nagual,
}

#[derive(Debug)]
pub enum AspectoPersonal {
  Fisico {
      sensaciones: SensaciónCorporal,
      postura: Postura,
      limites: (i32, i32),
      fuerza: i32,
  },
  Emocional(Emoción),
  Mental(String),
  Energético(TipoEnergía),
}